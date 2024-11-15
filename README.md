# START OF README.md TEMPLATE SETUP

## How to use this template?

Create a new repository https://github.com/new using this template.

Clone the new repository.

## Modify this file according to your needs

* Replace the text '___REPOSITORY_NAME___' in all files with your repository name.
* Replace the text '___PROJECT_NAME___' in all files with the name of the project.
* Replace the text '___PROJECT_DESCRIPTION___' in all files with the name of the project.

## Setup the required vcpkg packages

Open the file `vcpkg.json` and add the required packages, for example:

```json
{
  "$schema": "https://raw.githubusercontent.com/microsoft/vcpkg-tool/main/docs/vcpkg.schema.json",
  "dependencies": [
     {
      "name": "gstreamer",
      "features": [
        "plugins-bad",
        "plugins-base",
        "plugins-good",
        "plugins-ugly",
        {
          "name": "nvcodec",
          "platform": "!osx"
        }
      ]
    },
    {
      "name": "librsvg"
    }
  ]
}
```

## Setup the .github/workflows/check.yml file

Enable or remove the test job according to your needs.

## Final steps

* Remove this `README.md TEMPLATE SETUP` section.
* Amend the first commit to follow the conventional commit guidelines and force push the changes with `git push --force-with-lease origin main`

# END OF README.md TEMPLATE SETUP

[![CI checks](https://github.com/x-software-com/___REPOSITORY_NAME___/actions/workflows/check.yml/badge.svg)](https://github.com/x-software-com/___REPOSITORY_NAME___/actions/workflows/check.yml)
[![dependency status](https://deps.rs/repo/github/x-software-com/___REPOSITORY_NAME___/status.svg)](https://deps.rs/repo/github/x-software-com/___REPOSITORY_NAME___)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

# Overview

___PROJECT_DESCRIPTION___

# Contributing

This is an open source project, and is thus built with your contributions.

Here are some ways you can contribute:

* [Submit Issues][contributing:submit-issue]
* [Submit Fixes and New Features][contributing:submit-pr]

Please refer to our [Contributing Guide](CONTRIBUTING.md) for more details.

[contributing:submit-issue]: https://github.com/x-software-com/___REPOSITORY_NAME___/issues/new/choose
[contributing:submit-pr]: https://github.com/x-software-com/___REPOSITORY_NAME___/pulls

# License

The code in this repository is licensed under either of [APACHE-2.0 License](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
