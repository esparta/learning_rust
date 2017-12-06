

Blog Posts
<ul>
  {% for post in site.posts %}
    <li>
      <a href="learning_rust/{{ post.url }}">{{ post.title }}</a>
        {{ post.excerpt }}
    </li>
  {% endfor %}
</ul>
