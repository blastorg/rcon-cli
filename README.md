# RCON-CLI

A simple CLI for valves RCON server management.

![rcon cli preview](./media/rcon-demo-min.gif?raw=true)

## Usage

```bash
rcon --help
```

example commands:

- `rcon server list` // list all servers
- `rcon server add` // add a new server
- `rcon -d action exec` // execute a command on the **Default server**
- `rcon action exec` // execute a command on a server _(selected from a list)_
- `rcon action shell` // open a recurring command executor
- `rcon shell-completion <Filename e.g. completion.sh>` // generate shell completion script (auto detects shell)

## Installation

### Using Binaries

Download the [latest release](https://github.com/blastorg/rcon-cli-rs/releases/latest) for your operating system and add them to your PATH.

#### CLI Autocomplete

You can generate shell completion scripts by running the following command:

```bash
rcon shell-completion <Filename e.g. completion.sh>
```

Installing it depends on your shell, for example, for bash:

```bash
source completion.sh
```

## Building from source

Make sure you've set up your rust environment by following the instructions [here](https://www.rust-lang.org/tools/install).

```bash
  cargo build
```

or

```bash
  cargo build --release
```

## Logging verbosity flag 🚩

- -q silences output
- -v show warnings
- -vv show info
- -vvv show debug
- -vvvv show trace
