# Contribution guidelines

First off, thank you for considering contributing to RustHouse.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/CarloMicieli/rusthouse-gtk/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

## Developing

### Conventional commits

This repository is following the conventional commits practice.

#### Enforcing using git hooks

```shell
  git config core.hooksPath .githooks
```

The hook itself can be found in `.githooks/commit-msg`.

#### Using Commitizen

Install [commitizen](https://github.com/commitizen-tools/commitizen)

```shell
  pip install commitizen
```

and then just use it

```shell
  cz commit
```
