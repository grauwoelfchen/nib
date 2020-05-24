# Nib CLI

`nib-cli` is a command line utility to a yet another static site generator Nib.


## Usage

### Installation

```zsh
% cargo install nib-cli
```

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

### Writing article

```zsh
% cat blog/post/article.rst
.. title:: Nice foo
.. lang:: en # optional
.. date:: 2018-01-31 12:04:00 UTC
.. description:: This is an article about nice foo. # optional
.. slug:: foo.html # optional (default filename as is)

Foo is nice! # above blank line before content is required
```

### Generate

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


## Development

See `nib-server`.


## License

`Apache-2.0`

See [LICENSE](https://gitlab.com/grauwoelfchen/nib/-/blob/master/LICENSE)
