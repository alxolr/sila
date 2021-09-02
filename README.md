# sila

It's a simple terminal multiplexer written in rust that is operating using a config yaml file.

_sila_ - in pali language stands for morality, in russian means power. So it's somewhat a moral power.


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

## Improvement plan

- Properly parsing single/double quoted arguments
- Using pipelines in multiple terminals
- Lock and run a simple command only in a couple of terminals

``` bash
> lock PJ1 PJ3
> ... run {command}
[PJ1]> {command}
[PJ3]> {command}

> unlock
```