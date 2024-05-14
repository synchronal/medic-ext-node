# Medic Node

An extension pack for using [medic](https://github.com/synchronal/medic-rs)
with NodeJS projects or projects that build assets with Node.

## Installation

```shell
brew tap synchronal/tap
brew install medic-ext-node
```

Example `Brewfile`:

```shell
tap 'synchronal/tap'

brew  'synchronal/tap/medic'
brew  'synchronal/tap/medic-ext-node'
```

## Usage

```toml
[doctor]
checks = [
  { check = "homebrew" },
  { check = "asdf", command = "plugin-installed", args = { plugin = "nodejs" } },
  { check = "asdf", command = "package-installed", args = { plugin = "nodejs" } },
  { check = "node", command = "corepack-shim-installed", args = { name = "pnpm", version = "8.6.5" } },
  { check = "node", command = "packages-installed", args = { prefix = "assets" } },
]

[outdated]
checks = [
  { check = "node", cd = "subdirectory" }
]
```


## medic-check-node

Checks for whether a Node project is configured.

### Corepack shim installed?

Corepack is a library that ships with modern versions of Node, providing bridges between
package managers. Libraries may require pnpm, yarn, etc in order to be compiled, and through
corepack be installable from npm.

```shell
medic-check-node corepack-shim-installed --name <name> --version <version>
medic-check-node corepack-shim-installed --name pnpm --version 8.6.5
```

### NPM exists?

Is `npm` available in the `PATH`?

```shell
medic-check-node npm-exists
```

### Packages installed?

Are all packages installed? Optionally change directories to another project,
or pass `prefix` to `npm` when checking for installed packages.

```shell
medic-check-node packaged-installed
medic-check-node packaged-installed --cd <dir>
medic-check-node packaged-installed --prefix <dir>
```


## medic-outdated-node

Check for outdated packages.

