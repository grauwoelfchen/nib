# Nib

A yet another static blog generator.

## Usage

### Build

Put your articles under `blog` directory.

```zsh
% tree blog
blog
└── hello-word.rst
```

And then generate HTML files into `dst` directory.

```zsh
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
└── js
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
