---
layout: default
title: Blog tags
permalink: /blog/tags/
---

<article class="post article">
<div class="regular post-body">

<h1 class="post-title">Blog tags</h1>

{% assign all_posts = site.posts | concat: site.micro | sort: "date" | reverse %}
{% assign all_tags = "" | split: "," %}
{% for post in all_posts %}
  {% if post.tags %}
    {% assign all_tags = all_tags | concat: post.tags %}
  {% endif %}
{% endfor %}
{% assign sorted_tags = all_tags | uniq | sort %}
<ul class="tags">
{% for tag in sorted_tags %}
  <li><a href="#{{ tag | slugify }}">{{ tag }}</a></li>
{% endfor %}
</ul>

{% for tag in sorted_tags %}
<h2 id="{{ tag | slugify }}">{{ tag }}</h2>

<ul class="post-list">
  {% for post in all_posts %}
    {% if post.tags contains tag %}
    {% unless post.hidden == true %}
  <li>
    <time datetime="{{ post.date | date_to_xmlschema }}">{{ post.date | date: "%Y-%m-%d" }}</time>
    <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">{{ post.title }}</a>
  </li>
    {% endunless %}
    {% endif %}
  {% endfor %}
</ul>
{% endfor %}

</div>
</article>
