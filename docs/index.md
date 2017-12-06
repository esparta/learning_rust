

Blog Posts
<ul>
  {% for post in site.posts %}
    <li>
      <a href="./{{ post.url }}">{{ post.subtitle }}</a>
        {{ post.excerpt }}
    </li>
  {% endfor %}
</ul>
