# Nib

A yet another static site generator.

## Usage

### Configuration

```zsh
# e.g.
% cat config.toml
[website]
title = "An awesome blog"
description = """
"""
lang = "en" # language_code
url = "http://127.0.0.1:3000"
# the top most directory (e.g. blog) will be omitted in url
include = [
  "blog/**/*.rst"
]
license = "CC-BY-NC-SA-4.0"
# this or [[website.metadata.authors]] either is required
# authors = [
#  "Yasuhiro Яша Asaka <yasuhiro.asaka@grauwoelfchen.net>",
# ]

[build]
target-dir = "dst"

[[website.metadata.authors]]
name = "Yasuhiro Яша Asaka"
nick = "grauwoelfchen"
bio = """
A Programmer. I'm hacking on Gentoo/Funtoo Linux. I love greens and vegetables.
"""
email = "yasuhiro.asaka@grauwoelfchen.net"
avatar = "https://www.gravatar.com/avatar/...?s=40"
```

### Article

```zsh
% cat blog/post/article.rst
.. title:: Nice foo
.. lang:: en # optional
.. date:: 2018-01-31 12:04:00 UTC
.. description:: This is an article about nice foo. # optional
.. slug:: foo.html # optional (default filename as is)

Foo is nice! # above blank line before content is required
```

### Build

Put your articles under a directory which is contained in `include` section in
config file.

```zsh
# e.g. blog
% tree blog
blog
├── post
│   └── article.rst
└── hello-word.rst
```

And then generate HTML files into a directory specified with `target-dir`.

```zsh
% nib

# with built version
% make build
% ./target/debug/nib
```

The output result looks like this:

```zsh
% tree dst
dst
├── css
│   └── index.css
├── hello-world.html
├── img
├── index.html
├── js
└── post
    └── foo.html
```

### Development

Start development server. It serves contents in `dst` by listening
`127.0.0.1:300`.

```zsh
% make build:server
% ./target/debug/nib-server
```


## License

```text
Nib
Copyright 2020 Yasuhiro Яша Asaka

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
