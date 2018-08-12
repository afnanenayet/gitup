# `gitup` Design and Architecture

## Synopsis

This is a document that intends to specify the design and architecture of
the `gitup` tool. This can also serve as a reference for development (both
initially and for maintenance) to provide a cohesive overview of the code
base.

## Purpose

The purpose of `gitup` is to serve as a tool that manages the updates of
git repositories. It's primarily designed for git repos that are
consumed, but not modified (projects that aren't managed by a package manager,
for example, but a user consumes without contributing to). It is also useful
for projects that have a branch, or multiple branches, that are not modified
by the user. For example, a project with a master branch that a user doesn't
directly modify, instead modifying other branches. In this case, the user would
want the master branch to constantly stay updated.

With many such git repositories that need to be updated, this tool acts as a
central hub that can oversee updates over git and keep track of these updates.
It will also allow users to configure which branch(es) to keep updated (whether
the user wants to update the `master`, `develop` branch, for example).

## Core functionality

- Update git repositories
  - Update any particular branch or multiple branches
    - Ensure that only one instance of `gitup` is updating the branches at
      any given moment
  - Allow for various ways to select which repositories should be updated
    - By default, update all tracked repositories
    - Use fuzzy matching or regular expressions to either include or
      exclude repositories from being updated
- Track which repositories and branches to update
- Allow options to be configured through any serializable format, or from the
  command line interface

## Modules

This is a more technical and specific description of how the codebase is
organized.

### `config`

The `config` module defines the configuration structure, as well
parsing/serialization/deserialization of the structure. It also defines a
method for finding where the configuration file actually is. If no
configuration is found, it will create a default config file.

### `git_fn`

This module serves as a wrapper for all git related operations. This is put in
place to reduce complexity related to external libraries (suppose the git
library is changed, or there is a breaking update to the crate), this way
less code has to be modified with respect to external libraries.

### `proc`

The `proc` module exists for process management. It maintains locks and ensures
that only one instance of `gitup` is running at any given moment. This prevents
config files from being corrupted, or any issues with updating git repos.

### `tui`

This module manages all user facing communication, so that it can be contained
in one module rather than be interspersed throughout the codebase. It will
contain functions to manage output and command line arguments/flags.
