# Dotty

Dotty is a work-in-progress configuration file manager for Linux systems. It is
similar to [`etckeeper`][etckeeper], but has some key differences.

 - Store the configuration of multiple hostnames in one repository
 - Selectively add any file from `/` to the repository, if desired
 - Be very snappy and responsive
 - Store metadata of files, such as permissions

[etckeeper]: https://etckeeper.branchable.com/

## How It Works

Dotty is written in Rust, wraps around a bare Git repository and provides
commands to add any file on the file system, without being hit by performance
issues. It also tracks the metadata of files in a simple JSON file. This makes
it possible, for example, to track files in `/etc`.

## Commands

Note that most, if not all, commands require `sudo` to work if you're going to track something not in `$HOME`.

The following environment variables are supported:

| Name           | Description                                                                                     |
|----------------|-------------------------------------------------------------------------------------------------|
| DOTTY_ROOT     | The path with which all other paths are resolved, similar to `chroot`.                          |
| DOTTY_HOSTNAME | Overrides the name of the device that is being configured. Defaults to the machine's host name. |

The following global CLI flags are supported when invoking Dotty:

 - `--hostname` Same as the `DOTTY_HOSTNAME` environment variable. Takes
   precedence over `DOTTY_HOSTNAME`.
 - `--root-dir` Same as the `DOTTY_ROOT` environment variable. Takes precedence
   over `DOTTY_ROOT`.
 - `--force` Do not ask for confirmations when executing. Files will be
   overwritten without question. This is potentially very dangerous and should
   generally only be used in scripts.

### `dotty init`

Initialize a new repository with the default settings.

The repository will by default be initialized at `/.dotty`, unless `DOTTY_ROOT`
has been set.

### `dotty add <path..>`

Mark a file for inclusion in the next commit by adding it to the staging area.

This is roughly the same as `git add`, but with less options.

### `dotty status`

Display a compact list of files that have been staged to be committed.

This is somewhat similar to `git status --short`, but with less options.

### `dotty commit [-m message] [-a]`

Push the staged changes into a new commit and move HEAD.

Options:

 - `-m`, `--message` A description of the commit. If omitted, Dotty will open
   an editor.
 - `-a`, `--amend` Change the last commit on HEAD instead of creating a new commit.

> [!WARNING]
>
> The `--amend` flag currently does not do anything.

### `dotty log [flags..]`

Print a log of commits to the terminal.

This is identical to running `git log` and accepts the same flags except for `--git-dir`.

### `dotty list`

A small utility to list what exactly is stored in the latest commit.

## License

This repository is licensed under the MIT license.
