# sila@0.3.1

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
> sila pin PJ1 PJ3
> ... run {command}
[PJ1]> {command}
[PJ3]> {command}

> sila unpin PJ1
>... run {command}
[PJ3]> {command}
...
```

### Helper commands

```bash
> sila help

list                      List the terminal names.
pin     <term1> <term2>   Pin one or multiple terminals separated by space. Following commands will be run on top of pinned ones.
unpin   [term1]           Unpin all terminals if no argument is provided or the specific ones.
count                     Count the number of terminals.
exit                      Close the application.
```
