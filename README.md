# Nib

A yet another static site generator.

This is composed of three crates `nib`, `nib-cli` and `nib-server`.

| name | crates.io | docs.rs | description |
|:---|:---:|:---:|:---|
| `nib` | [![crates.io][merit::nib]][crate::nib] | [![docs.rs][badge::nib]][doc::nib] | main library to assist making cli |
| `nib-cli` | [![crates.io][merit::nib-cli]][crate::nib-cli] | - | cli application to generate static site |
| `nib-server` | [![crates.io][merit::nib-server]][crate::nib-server] | - | local development server to host generated files |

[merit::nib]: https://meritbadge.herokuapp.com/nib
[merit::nib-cli]: https://meritbadge.herokuapp.com/nib-cli
[merit::nib-server]: https://meritbadge.herokuapp.com/nib-server

[crate::nib]: https://crates.io/crates/nib
[crate::nib-cli]: https://crates.io/crates/nib-cli
[crate::nib-server]: https://crates.io/crates/nib-server

[doc::nib]: https://docs.rs/nib
[badge::nib]: https://docs.rs/nib/badge.svg


## Guide

To create own website, please take a look at [nib-cli](src/nib-cli).

```zsh
% cargo install nib-cli
```

## Theme

* blog (default)
* documentation


## Development

This project is developed on [Gitlab.com](https://gitlab.com/grauwoelfchen/nib),
mirrored on [Github](https://github.com/grauwoelfchen/nib) and also [
git.sr.ht](https://git.sr.ht/~grauwoelfchen/nib).

Every issue and pull request on anywhere above is very welcome ;)


## Release

Check `nib`'s version in '`src/nib-cli/Cargo.toml` before release.


## License

`Apache-2.0`

```text
Nib
Copyright 2020-2021 Yasuhiro Яша Asaka

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
