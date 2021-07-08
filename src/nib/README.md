# Nib

`nib` is the main library of Nib and which is built to assist making a
static site generator application.

Please take a look at `nib-cli` or `nib-server`.


## Usage

### Installation

```zsh
% cargo install nib
```

### Example

TODO


## Development

```zsh
% cd /path/to/repo
% make build:debug:server
% ./target/debug/nib-server
```

```zsh
% cd /path/to/repo
% rm -fr src/**/target
% rm -fr dst/*
% make build:debug::cli
% ./target/debug/nib
```
