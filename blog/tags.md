---
layout: default
title: Blog tags
permalink: /blog/tags/
---

<article class="post article">
<div class="regular post-body">

<h1 class="post-title">Blog tags</h1>

{% assign sorted_tags = site.tags | sort %}
<ul class="tags">
{% for tag_entry in sorted_tags %}
  {% assign tag = tag_entry[0] %}
  <li><a href="#{{ tag | slugify }}">{{ tag }}</a></li>
{% endfor %}
</ul>

{% for tag_entry in sorted_tags %}
  {% assign tag = tag_entry[0] %}
  {% assign tagged_posts = tag_entry[1] | sort: "date" | reverse %}
<h2 id="{{ tag | slugify }}">{{ tag }}</h2>

<ul class="post-list">
  {% for post in tagged_posts %}
    {% unless post.hidden == true %}
  <li>
    <time datetime="{{ post.date | date_to_xmlschema }}">{{ post.date | date: "%Y-%m-%d" }}</time>
    <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">{{ post.title }}</a>
  </li>
    {% endunless %}
  {% endfor %}
</ul>
{% endfor %}

</div>
</article>
