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
  - Allow for various ways to select which repositories should be updated
    - By default, update all tracked repositories
    - Use fuzzy matching or regular expressions to either include or
      exclude repositories from being updated
- Track which repositories and branches to update
- Allow options to be configured through any serializable format, or from the
  command line interface
