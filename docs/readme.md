# gitup

[![Build Status](https://travis-ci.org/afnanenayet/gitup.svg?branch=master)](https://travis-ci.org/afnanenayet/gitup)

## Synopsis

`gitup` is a tool to help you manage git repositories that you only consume.
I found that I have a few git repositories that I never really contribute to, but
only need a copy of the latest master/stable branch, and this tool allows you
to manage and update all of these repositories at once.

This is great for miscellaneous git based projects that don't quite fit in any
of your other package managers, but you want to easily update.

This is currently a work in progress and is in a pre-alpha stage. It is not
usable.

## Usage

`gitup <command> [options]`

### Commands

Note that `gitup`, after startup, will default to updating all git
repositories if no commands or options are supplied. This behavior can be
configured.

- `gitup`: By default, updates all of the repositories currently registered.
  If there are no repositories registered, it will do nothing.
  - `add <path>`: Add a git repo at some path to be managed by `gitup`. If no
    git repo is found at the root of the directory, the command will throw an
    error to `STDERR` and exit.
  - `update`: By default, this will attempt to update all of the repositories
    being tracked by `gitup`. You can pass flags to it in order to more
    specifically track which repos you want to update. If updating any of the
    repositories fail, `gitup` will report the repositories that failed and
    why. Fixing them may require manual intervention for an operation such as
    a merge.
    - `--include A B C …` Include any repositories that match the regular
      expressions that follow the flag. An arbitrary number of regular
      expressions can be passed to the flag.
    - `--exclude A B C …` Include any repositories that match the regular
      expressions that follow the flag. An arbitrary number of regular
      expressions can be passed to the flag.

#### Options

These options are top-level flags, so they can be applied in conjunction with
any command.

- `-h, --help`: Display the help message with information about commands and flags.
- `-c <file>, --config <file>`: Use the config file at the given path. This
  overrides default behavior.
- `-d, --dry-run`: Display the actions `gitup` plans to take, but don't modify
  anything.

### Exit codes

- 0: Success.
- 1: Unknown/unexpected internal error.
- 2: Invalid git repository.
- 3: Failed to connect to git remote.
- 4: Failed to pull changes from git remote.
- 5: Merge errors occurred after pulling from git remote.
- 6: Internal git error.

### Configuration

`gitup` can be configured through a configuration file found at
`$XDG_HOME/gitup`. If `$XDG_HOME` is not present in your environment, then
the default value of `~/.config` will be used, so your file will likely
be placed in `~/.config/gitup/conf.yaml`.

The configuration file keeps track of which git directories `gitup` should
try to update, and on which branch or tag setup `gitup` should from pull from.
It uses the `yaml` file format for the config file. The config can be set
through the command line or edited directly.

The details of the configuration structure are currently being worked on and
will be posted when they are finalized.

## Development

This is a standard Rust/Cargo project. To build the project locally:

```sh
cargo build --release
```

To run tests:

```sh
cargo test
```

## Roadmap

- [ ] Design documentation
- [ ] Configuration structure
- [ ] MVP
