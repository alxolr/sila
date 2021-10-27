# sila@0.3.2 [![Build Status](https://app.travis-ci.com/alxolr/sila.svg?branch=main)](https://app.travis-ci.com/alxolr/sila)[![codecov](https://codecov.io/gh/alxolr/sila/branch/main/graph/badge.svg?token=KPKR339KY4)](https://codecov.io/gh/alxolr/sila)


It's a simple terminal multiplexer written in rust that is operating using a config yaml file.

_sila_ - reads [shila] in pali language stands for morality;

## Install

In order to install sila you need `cargo` and `rust` in your sistem.

```
cargo install sila
```

## Configure

Create a yaml file with your projects that you want to multiplex the commands.

```yaml
# config.yaml

- name: PJ1
  path: /path/to/your/project1

- name: PJ2
  path: /path/to/your/project2

- name: PJ3
  path: /path/to/your/project3
```

## How to use:

```bash
> sila config.yaml

> git describe

[PJ1]> git describe
...

[PJ2]> git describe

...
[PJ3]> git describe
...
```

## Features

### Piping commands

```bash
> cat Cargo.toml | grep version

[PJ1]> cat Cargo.toml | grep version
version = "0.1.0"

[PJ2]> cat Cargo.toml | grep version
version = "0.2.0"
serde = { version = "1.0", features = ["derive"] }

[PJ3]> cat Cargo.toml | grep version
version = "0.1.0"
```

### Pinning terminals

- pin/unpin terminals and run a simple command only in a couple of terminals

```bash
> pin PJ1 PJ3
> ... run {command}
[PJ1]> {command}
[PJ3]> {command}

> unpin PJ1
>... run {command}
[PJ3]> {command}
...
```

### Helper commands

```bash
> help

sila@0.3.1
A command line multiplexer.
created by Alexandru Olaru <alxolr@gmail.com>

COMMANDS:
pin     <term1> <term2>   Pin one or multiple terminals separated by space. Following commands will run on top of pinned ones only.
unpin   [term1]           Unpin all terminals if no argument is provided or the specific ones.
ban     <term1> <term2>   Ban one or multiple terminals separated by space. The following commands will not run in banned terminals
unban   [term2]           Unban the specificed terminals or all if no arguments provided.
list                      List the active terminal names.
help                      Displays help information.
exit                      Close the application.
```