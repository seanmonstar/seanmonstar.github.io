use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde_json::json;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), errors::Main> {
    println!("Starting POSSE sync...");

    // Get the list of micro posts
    // Filter if they do not have frontmatter for a network
    let micro_posts_dir = std::env::var("MICRO_POSTS_DIR")?;
    let posts = unsynced_micro_posts(micro_posts_dir.as_ref()).await?;
    println!("Found {} micro post(s) needing sync.", posts.len());

    let client = reqwest::Client::builder()
        .user_agent("seanmonstar.com/0.0.1")
        .build()?;

    // Login to each network from ENV variables
    let bsky_identifier = std::env::var("BSKY_IDENTIFIER")?;
    let bsky_password = std::env::var("BSKY_PASSWORD")?;
    let bsky = bsky_login(&client, &bsky_identifier, &bsky_password).await?;
    println!("Logged in to Bluesky as {bsky_identifier}.");
    let masto = std::env::var("MASTODON_TOKEN")?;
    let masto_url = std::env::var("MASTODON_URL")?;
    let site_url = std::env::var("SITE_URL")?;
    // For each post
    //     Split it into paragaphs
    //     Publish the list into each network
    //     Each paragraph does as a reply in a chain
    for post in posts {
        println!("Syncing {}...", post.path.display());
        let mut updates = Vec::new();
        if post.bsky_url.is_none() {
            let url = bsky_post(&client, &bsky, &post, &site_url).await?;
            println!("Bluesky sync succeeded: {url}");
            updates.push(("bsky_url", url));
        } else {
            println!("Bluesky already synced.");
        }
        if post.mastodon_url.is_none() {
            let url = masto_post(&client, &masto_url, &masto, &post, &site_url).await?;
            println!("Mastodon sync succeeded: {url}");
            updates.push(("mastodon_url", url));
        } else {
            println!("Mastodon already synced.");
        }
        if !updates.is_empty() {
            update_post_frontmatter(&post.path, &updates).await?;
            println!("Updated frontmatter for {}.", post.path.display());
        }
    }
    // Store the URL to post on each network in the frontmatter
    Ok(())
}

const BSKY_MAX_CHARS: usize = 300;
const MASTODON_MAX_CHARS: usize = 500;

struct Post {
    path: PathBuf,
    // frontmatter: String,
    body: String,
    // permalink: Option<String>,
    title: Option<String>,
    // date: Option<String>,
    mastodon_url: Option<String>,
    bsky_url: Option<String>,
}

async fn unsynced_micro_posts(dir: &Path) -> Result<Vec<Post>, Error> {
    let mut entries = tokio::fs::read_dir(dir).await?;
    let mut file_paths = Vec::new();

    // 1. Gather all file paths first without reading their contents
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            file_paths.push(path);
        }
    }

    // 2. Sort paths alphabetically in REVERSE (newest dates first)
    // "2026-06-08-posse.md" will come before "2026-06-01-posse.md"
    file_paths.sort_by(|a, b| b.cmp(a));

    let mut unsynced_posts = Vec::new();

    // 3. Process files from newest to oldest
    for path in file_paths {
        let content = tokio::fs::read_to_string(&path).await?;
        let (metadata, body) = parse_markdown_file(&content);

        let mastodon_url = metadata.get("mastodon_url").cloned();
        let bsky_url = metadata.get("bsky_url").cloned();

        // TODO: Track sync state per-network instead of stopping only once both
        // networks are present.
        // If we hit a file that has already been synced to both platforms,
        // we can safely assume everything older than this file is also synced.
        if mastodon_url.is_some() && bsky_url.is_some() {
            break;
        }

        let title = metadata.get("title").cloned();
        // let date = metadata.get("date").cloned();
        // let permalink = metadata.get("permalink").cloned();
        // let raw_frontmatter = content.split("---").nth(1).unwrap_or("").to_string();

        unsynced_posts.push(Post {
            path,
            // frontmatter: raw_frontmatter,
            body: body.trim().to_string(),
            // permalink,
            title,
            // date,
            mastodon_url,
            bsky_url,
        });
    }

    // 4. Flip the final vector back so they are returned in chronological order
    // (oldest unsynced first, so your poster script publishes them in the right order)
    unsynced_posts.reverse();

    Ok(unsynced_posts)
}

struct BskySession {
    did: String,
    access_jwt: String,
}

// pass in credentials as argument
async fn bsky_login(
    client: &reqwest::Client,
    username: &str,
    pass: &str,
) -> Result<BskySession, Error> {
    let session = client
        .post("https://bsky.social/xrpc/com.atproto.server.createSession")
        .json(&json!({
            "identifier": username,
            "password": pass,
        }))
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;

    let did = session
        .get("did")
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| errors::new("missing did"))?;
    let access_jwt = session
        .get("accessJwt")
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| errors::new("missing accessJwt"))?;
    Ok(BskySession { access_jwt, did })
}

// pass in post details as argument, return bsky post url
async fn bsky_post(
    client: &reqwest::Client,
    session: &BskySession,
    post: &Post,
    site_url: &str,
) -> Result<String, Error> {
    let paragraphs = social_posts(post, site_url, BSKY_MAX_CHARS)?;
    log_social_posts("Bluesky", post, &paragraphs);

    let mut root_ref: Option<serde_json::Value> = None;
    let mut parent_ref: Option<serde_json::Value> = None;
    let mut first_url = None;

    for paragraph in paragraphs {
        let mut record = json!({
            "$type": "app.bsky.feed.post",
            "text": paragraph,
            "createdAt": jiff::Timestamp::now().to_string(),
        });

        let facets = bsky_link_facets(&paragraph);
        if !facets.is_empty() {
            record["facets"] = json!(facets);
        }

        // Bluesky does not fetch Open Graph metadata for posts created through
        // the API, so include a text-only website card on the thread root.
        if root_ref.is_none() {
            record["embed"] = bsky_external_embed(post, site_url)?;
        }

        // If this isn't the first post, attach the threading architecture
        if let (Some(root), Some(parent)) = (&root_ref, &parent_ref) {
            record["reply"] = json!({
                "root": root.clone(),
                "parent": parent.clone(),
            });
        }

        // Post the record to the repository
        let post_output = client
            .post("https://bsky.social/xrpc/com.atproto.repo.createRecord")
            .bearer_auth(&session.access_jwt)
            .json(&json!({
                "repo": session.did,
                "collection": "app.bsky.feed.post",
                "record": record
            }))
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        // Initialize or update tracking pointers
        if root_ref.is_none() {
            root_ref = Some(bsky_strong_ref(&post_output)?);
            first_url = Some(bsky_app_url(&session.did, &post_output)?);
        }
        parent_ref = Some(bsky_strong_ref(&post_output)?);
    }

    first_url.ok_or_else(|| errors::new("empty bsky post").into())
}

fn bsky_external_embed(post: &Post, site_url: &str) -> Result<serde_json::Value, Error> {
    let uri = micro_permalink(&post.path, site_url)?;
    let title = post
        .title
        .as_deref()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .unwrap_or("seanmonstar");
    let description = markdown_to_social_text(&post.body, Some(site_url))
        .split("\n\n")
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    Ok(json!({
        "$type": "app.bsky.embed.external",
        "external": {
            "uri": uri,
            "title": title,
            "description": description,
        }
    }))
}

fn bsky_link_facets(text: &str) -> Vec<serde_json::Value> {
    let mut facets = Vec::new();
    let mut cursor = 0;

    while cursor < text.len() {
        let haystack = &text[cursor..];
        let Some(relative_start) = earliest_url_start(haystack) else {
            break;
        };
        let start = cursor + relative_start;
        let mut end = text[start..]
            .find(char::is_whitespace)
            .map(|relative_end| start + relative_end)
            .unwrap_or(text.len());

        end = trim_url_end(&text[start..end], start);
        if end > start {
            let uri = &text[start..end];
            facets.push(json!({
                "index": {
                    "byteStart": start,
                    "byteEnd": end,
                },
                "features": [{
                    "$type": "app.bsky.richtext.facet#link",
                    "uri": uri,
                }],
            }));
        }

        cursor = end.max(start + 1);
    }

    facets
}

fn earliest_url_start(text: &str) -> Option<usize> {
    match (text.find("https://"), text.find("http://")) {
        (Some(https), Some(http)) => Some(https.min(http)),
        (Some(https), None) => Some(https),
        (None, Some(http)) => Some(http),
        (None, None) => None,
    }
}

fn trim_url_end(candidate: &str, start: usize) -> usize {
    let mut end = candidate.len();

    while end > 0 {
        let trimmed = &candidate[..end];
        let Some(ch) = trimmed.chars().next_back() else {
            break;
        };

        let should_trim = match ch {
            '.' | ',' | ';' | ':' | '!' | '?' => true,
            ')' => trimmed.matches(')').count() > trimmed.matches('(').count(),
            ']' => trimmed.matches(']').count() > trimmed.matches('[').count(),
            '}' => trimmed.matches('}').count() > trimmed.matches('{').count(),
            _ => false,
        };

        if !should_trim {
            break;
        }

        end -= ch.len_utf8();
    }

    start + end
}

// pass in post details as argument, return masto post url
async fn masto_post(
    client: &reqwest::Client,
    instance: &str,
    token: &str,
    post: &Post,
    site_url: &str,
) -> Result<String, Error> {
    let paragraphs = social_posts(post, site_url, MASTODON_MAX_CHARS)?;
    log_social_posts("Mastodon", post, &paragraphs);

    let mut post_id: Option<String> = None;
    let mut parent_id: Option<String> = None;
    let mut first_url = None;

    for paragraph in paragraphs {
        let mut payload = json!({
            "status": paragraph,
            "visibility": "public"
        });

        // If this is a follow-up paragraph, link it to the previous one
        if let Some(ref id) = parent_id {
            payload["in_reply_to_id"] = json!(id);
        }

        let resp = client
            .post(format!("{}/api/v1/statuses", instance))
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        // Update the ID so the next paragraph threads under this one
        parent_id = resp.get("id").and_then(|v| v.as_str()).map(String::from);
        if post_id.is_none() {
            post_id = parent_id.clone();
            first_url = resp.get("url").and_then(|v| v.as_str()).map(String::from);
        }
    }

    first_url.ok_or_else(|| errors::new("empty mastodon post").into())
}

fn log_social_posts(network: &str, post: &Post, paragraphs: &[String]) {
    println!(
        "{network}: prepared {} post(s) for {}:",
        paragraphs.len(),
        post.path.display()
    );
    for (index, paragraph) in paragraphs.iter().enumerate() {
        println!(
            "--- {network} post {}/{} ({} chars) ---\n{}\n--- end {network} post {} ---",
            index + 1,
            paragraphs.len(),
            paragraph.chars().count(),
            paragraph,
            index + 1,
        );
    }
}

fn social_posts(post: &Post, site_url: &str, max_len: usize) -> Result<Vec<String>, Error> {
    let body = markdown_to_social_text(&post.body, Some(site_url));
    let mut body_posts = split_post_text(&body, max_len);
    let permalink = micro_permalink(&post.path, site_url)?;
    let attribution = format!("Originally at {permalink}");

    let first_text = post
        .title
        .as_deref()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .map(String::from)
        .or_else(|| {
            if body_posts.is_empty() {
                None
            } else {
                Some(body_posts.remove(0))
            }
        })
        .ok_or_else(|| errors::new("empty post"))?;

    let mut posts = Vec::new();
    let mut first_posts = split_first_post_with_attribution(&first_text, &attribution, max_len)?;
    posts.append(&mut first_posts);
    posts.append(&mut body_posts);

    Ok(posts)
}

fn split_first_post_with_attribution(
    text: &str,
    attribution: &str,
    max_len: usize,
) -> Result<Vec<String>, Error> {
    let separator = "\n\n";
    let attribution_len = attribution.chars().count();
    let separator_len = separator.chars().count();
    if attribution_len + separator_len >= max_len {
        return Err(errors::new("permalink attribution is too long").into());
    }

    let combined = format!("{text}{separator}{attribution}");
    if combined.chars().count() <= max_len {
        return Ok(vec![combined]);
    }

    let first_text_max = max_len - attribution_len - separator_len;
    let mut text_posts = split_post_text(text, first_text_max);
    let Some(first) = text_posts.first_mut() else {
        return Ok(vec![attribution.to_string()]);
    };
    first.push_str(separator);
    first.push_str(attribution);

    Ok(text_posts)
}

fn micro_permalink(path: &Path, site_url: &str) -> Result<String, Error> {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| errors::new("micro post path has no filename"))?;

    let mut parts = stem.splitn(4, '-');
    let year = parts
        .next()
        .filter(|part| part.len() == 4 && part.chars().all(|ch| ch.is_ascii_digit()))
        .ok_or_else(|| errors::new("micro post filename missing year"))?;
    let month = parts
        .next()
        .filter(|part| part.len() == 2 && part.chars().all(|ch| ch.is_ascii_digit()))
        .ok_or_else(|| errors::new("micro post filename missing month"))?;
    let day = parts
        .next()
        .filter(|part| part.len() == 2 && part.chars().all(|ch| ch.is_ascii_digit()))
        .ok_or_else(|| errors::new("micro post filename missing day"))?;
    let slug = parts
        .next()
        .filter(|part| !part.is_empty())
        .ok_or_else(|| errors::new("micro post filename missing slug"))?;

    Ok(format!(
        "{}/micro/{year}{month}{day}-{slug}/",
        site_url.trim_end_matches('/')
    ))
}

fn split_post_text(body: &str, max_len: usize) -> Vec<String> {
    let mut out = Vec::new();

    for paragraph in body
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        split_long_text(paragraph, max_len, &mut out);
    }

    out
}

fn markdown_to_social_text(markdown: &str, site_url: Option<&str>) -> String {
    let without_code = replace_code_fences(markdown);
    transform_inline_links(&without_code, site_url)
}

fn replace_code_fences(markdown: &str) -> String {
    let newline = if markdown.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut out = Vec::new();
    let mut in_fence = false;
    let mut fence_marker = "";

    for line in markdown.lines() {
        let trimmed = line.trim_start();
        let marker = if trimmed.starts_with("```") {
            Some("```")
        } else if trimmed.starts_with("~~~") {
            Some("~~~")
        } else {
            None
        };

        if let Some(marker) = marker {
            if in_fence && marker == fence_marker {
                in_fence = false;
                fence_marker = "";
            } else if !in_fence {
                in_fence = true;
                fence_marker = marker;
                out.push("(Code omitted)");
            }
            continue;
        }

        if !in_fence {
            out.push(line);
        }
    }

    out.join(newline)
}

fn transform_inline_links(text: &str, site_url: Option<&str>) -> String {
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;

    while cursor < text.len() {
        if text[cursor..].starts_with("![") {
            if let Some(end) = markdown_link_end(text, cursor + 1) {
                cursor = end;
                continue;
            }
        }

        if text[cursor..].starts_with('[') {
            if let Some((end, label, url)) = parse_inline_link(text, cursor) {
                out.push_str(label);
                out.push_str(" (");
                out.push_str(&absolute_url(url, site_url));
                out.push(')');
                cursor = end;
                continue;
            }
        }

        let ch = text[cursor..]
            .chars()
            .next()
            .expect("cursor should point at a char boundary");
        out.push(ch);
        cursor += ch.len_utf8();
    }

    out
}

fn markdown_link_end(text: &str, start: usize) -> Option<usize> {
    parse_inline_link(text, start).map(|(end, _, _)| end)
}

fn parse_inline_link(text: &str, start: usize) -> Option<(usize, &str, &str)> {
    let label_start = start + 1;
    let label_end = find_unescaped(text, label_start, ']')?;
    let url_open = label_end + 1;
    if !text[url_open..].starts_with('(') {
        return None;
    }
    let url_start = url_open + 1;
    let url_end = find_unescaped(text, url_start, ')')?;

    Some((
        url_end + 1,
        &text[label_start..label_end],
        text[url_start..url_end].trim(),
    ))
}

fn find_unescaped(text: &str, start: usize, needle: char) -> Option<usize> {
    let mut escaped = false;
    for (index, ch) in text[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == needle {
            return Some(start + index);
        }
    }
    None
}

fn absolute_url(url: &str, site_url: Option<&str>) -> String {
    if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("mailto:") {
        return url.to_string();
    }

    let Some(site_url) = site_url else {
        return url.to_string();
    };

    if url.starts_with('/') {
        format!("{}{}", site_url.trim_end_matches('/'), url)
    } else {
        format!("{}/{url}", site_url.trim_end_matches('/'))
    }
}

fn split_long_text(text: &str, max_len: usize, out: &mut Vec<String>) {
    let mut remaining = text.trim();

    while remaining.chars().count() > max_len {
        let split_at = best_split_at(remaining, max_len);
        let (head, tail) = remaining.split_at(split_at);
        let head = head.trim();
        if !head.is_empty() {
            out.push(head.to_string());
        }
        remaining = tail.trim_start_matches(|c: char| c.is_whitespace() || c == ',');
    }

    if !remaining.is_empty() {
        out.push(remaining.to_string());
    }
}

fn best_split_at(text: &str, max_len: usize) -> usize {
    let limit = nth_char_boundary(text, max_len);
    let candidate = &text[..limit];

    for delimiter in [". ", "! ", "? ", ", ", "; ", ": ", " "] {
        if let Some(index) = candidate.rfind(delimiter) {
            return index + delimiter.trim_end().len();
        }
    }

    limit
}

fn nth_char_boundary(text: &str, n: usize) -> usize {
    text.char_indices()
        .nth(n)
        .map(|(index, _)| index)
        .unwrap_or(text.len())
}

fn bsky_strong_ref(output: &serde_json::Value) -> Result<serde_json::Value, Error> {
    let uri = output
        .get("uri")
        .and_then(|v| v.as_str())
        .ok_or_else(|| errors::new("missing bsky uri"))?;
    let cid = output
        .get("cid")
        .and_then(|v| v.as_str())
        .ok_or_else(|| errors::new("missing bsky cid"))?;

    Ok(json!({
        "uri": uri,
        "cid": cid,
    }))
}

fn bsky_app_url(profile: &str, output: &serde_json::Value) -> Result<String, Error> {
    let uri = output
        .get("uri")
        .and_then(|v| v.as_str())
        .ok_or_else(|| errors::new("missing bsky uri"))?;
    let rkey = uri
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| errors::new("missing bsky rkey"))?;

    Ok(format!("https://bsky.app/profile/{profile}/post/{rkey}"))
}

async fn update_post_frontmatter(path: &Path, updates: &[(&str, String)]) -> Result<(), Error> {
    let mut content = tokio::fs::read_to_string(path).await?;
    for (key, value) in updates {
        content = update_frontmatter_field(&content, key, value);
    }
    tokio::fs::write(path, content).await?;
    Ok(())
}

fn update_frontmatter_field(content: &str, key: &str, value: &str) -> String {
    let newline = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let field = format!("{key}: {value}");

    let Some((frontmatter_start, frontmatter_end)) = frontmatter_bounds(content) else {
        return format!("---{newline}{field}{newline}---{newline}{content}");
    };

    let frontmatter = &content[frontmatter_start..frontmatter_end];
    let mut cursor = 0;
    while cursor < frontmatter.len() {
        let line_end = frontmatter[cursor..]
            .find('\n')
            .map(|index| cursor + index + 1)
            .unwrap_or(frontmatter.len());
        let line = &frontmatter[cursor..line_end];
        let line_without_newline = line.trim_end_matches(['\r', '\n']);
        if line_without_newline
            .split_once(':')
            .is_some_and(|(candidate, _)| candidate.trim() == key)
        {
            let replacement = if line.ends_with("\r\n") {
                format!("{field}\r\n")
            } else if line.ends_with('\n') {
                format!("{field}\n")
            } else {
                field
            };
            let absolute_start = frontmatter_start + cursor;
            let absolute_end = frontmatter_start + line_end;
            return format!(
                "{}{}{}",
                &content[..absolute_start],
                replacement,
                &content[absolute_end..]
            );
        }
        cursor = line_end;
    }

    format!(
        "{}{}{}{}",
        &content[..frontmatter_end],
        field,
        newline,
        &content[frontmatter_end..]
    )
}

fn frontmatter_bounds(content: &str) -> Option<(usize, usize)> {
    let frontmatter_start = if content.starts_with("---\r\n") {
        5
    } else if content.starts_with("---\n") {
        4
    } else {
        return None;
    };

    let closing_marker = if content.starts_with("---\r\n") {
        "\r\n---"
    } else {
        "\n---"
    };
    let relative_end = content[frontmatter_start..].find(closing_marker)?;
    let marker_start = frontmatter_start + relative_end;
    let frontmatter_end = marker_start + closing_marker[..closing_marker.len() - 3].len();

    Some((frontmatter_start, frontmatter_end))
}

// A simple frontmatter extractor that isolates the YAML block from the markdown body
fn parse_markdown_file(content: &str) -> (HashMap<String, String>, String) {
    let mut frontmatter = HashMap::new();
    let mut body = String::new();

    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return (frontmatter, content.to_string());
    }

    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return (frontmatter, body);
    }

    let mut in_frontmatter = false;
    let mut frontmatter_raw = Vec::new();
    let mut body_lines = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 && *line == "---" {
            in_frontmatter = true;
            continue;
        }
        if in_frontmatter && *line == "---" {
            in_frontmatter = false;
            continue;
        }

        if in_frontmatter {
            frontmatter_raw.push(*line);
            // Parse simple "key: value" structures directly
            if let Some((key, val)) = line.split_once(':') {
                let k = key.trim().to_string();
                let v = val.trim().trim_matches('"').trim_matches('\'').to_string();
                frontmatter.insert(k, v);
            }
        } else {
            body_lines.push(*line);
        }
    }

    body = body_lines.join("\n");
    (frontmatter, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsky_link_facets_finds_multiple_links() {
        let text = "One https://example.com/a and http://example.net/b";

        let facets = bsky_link_facets(text);

        assert_eq!(facets.len(), 2);
        assert_eq!(facets[0]["index"]["byteStart"], 4);
        assert_eq!(facets[0]["index"]["byteEnd"], 25);
        assert_eq!(facets[0]["features"][0]["uri"], "https://example.com/a");
        assert_eq!(facets[1]["index"]["byteStart"], 30);
        assert_eq!(facets[1]["index"]["byteEnd"], 50);
        assert_eq!(facets[1]["features"][0]["uri"], "http://example.net/b");
    }

    #[test]
    fn bsky_link_facets_trims_wrapping_punctuation() {
        let text = "A link (https://example.com/a), and more.";

        let facets = bsky_link_facets(text);

        assert_eq!(facets.len(), 1);
        assert_eq!(facets[0]["index"]["byteStart"], 8);
        assert_eq!(facets[0]["index"]["byteEnd"], 29);
        assert_eq!(facets[0]["features"][0]["uri"], "https://example.com/a");
    }

    #[test]
    fn bsky_link_facets_uses_utf8_byte_offsets() {
        let text = "✨ https://example.com";

        let facets = bsky_link_facets(text);

        assert_eq!(facets.len(), 1);
        assert_eq!(facets[0]["index"]["byteStart"], 4);
        assert_eq!(facets[0]["index"]["byteEnd"], 23);
        assert_eq!(facets[0]["features"][0]["uri"], "https://example.com");
    }

    #[test]
    fn micro_permalink_matches_jekyll_collection_permalink() {
        let path = Path::new("../../_micro/2026-06-23-owning-my-microblog-with-posse.md");

        let permalink = micro_permalink(path, "https://seanmonstar.com/").unwrap();

        assert_eq!(
            permalink,
            "https://seanmonstar.com/micro/20260623-owning-my-microblog-with-posse/"
        );
    }

    #[test]
    fn bsky_external_embed_is_a_text_only_card() {
        let post = Post {
            path: PathBuf::from("../../_micro/2026-06-23-posse.md"),
            body: "A short **description**.\n\nA second paragraph.".to_string(),
            title: Some("Owning my microblog".to_string()),
            mastodon_url: None,
            bsky_url: None,
        };

        let embed = bsky_external_embed(&post, "https://seanmonstar.com/").unwrap();

        assert_eq!(embed["$type"], "app.bsky.embed.external");
        assert_eq!(
            embed["external"]["uri"],
            "https://seanmonstar.com/micro/20260623-posse/"
        );
        assert_eq!(embed["external"]["title"], "Owning my microblog");
        assert_eq!(embed["external"]["description"], "A short **description**.");
        assert!(embed["external"].get("thumb").is_none());
    }
}
