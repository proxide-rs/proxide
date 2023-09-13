### Hey, I'm {{ github_username }}

#### 🔥 Check out what I'm working on

{% for contribution in recentContributions() -%}
- [{{ contribution.repo_name }}]({{ contribution.repo_url }}) - {{ contribution.repo_desc }}
{% endfor %}

#### 🧪 Latest PRs

{% for pr in recentPullRequests() -%}
- [{{ pr.title }}]({{ pr.url }}) on [{{ pr.repo.name }}]({{ pr.repo.url }})
{% endfor %}

#### 📜 My recent blog posts

{% for post in rssFeed(url="https://domain.tld/feed.xml", count=3) -%}
- [{{ post.title }}](https://domain.tld{{ post.link }})
{% endfor %}