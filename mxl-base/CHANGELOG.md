# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -

## [0.2.9](https://github.com/x-software-com/mxl-crates/compare/mxl-base-v0.2.8...mxl-base-v0.2.9) - 2025-07-14

### Other

- update Cargo.toml dependencies

## [0.2.8](https://github.com/x-software-com/mxl-crates/compare/mxl-base-v0.2.7...mxl-base-v0.2.8) - 2025-07-04

### Other

- fix clippy warnings, use variables directly in the format string
- correct typo in function name
- update README links to point to mxl-crates
- correct typo in comment

## [0.2.7](https://github.com/x-software-com/mxl-crates/compare/mxl-base-v0.2.6...mxl-base-v0.2.7) - 2025-02-27

### Other

- update rust edition to 2024
- *(deps)* update directories requirement from 5 to 6

## [0.2.6](https://github.com/x-software-com/mxl-crates/compare/mxl-base-v0.2.5...mxl-base-v0.2.6) - 2024-12-12

### Other

- replaced once_cell dependency

## [0.2.5](https://github.com/x-software-com/mxl-crates/compare/mxl-base-v0.2.4...mxl-base-v0.2.5) - 2024-11-20

### Other

- cleanup cargo dependencies

## [v0.2.4](https://github.com/x-software-com/mxl-base/compare/e6cc51322b5dce2f6fe3e0345f848e3758ef5983..v0.2.4) - 2024-09-02
#### Bug Fixes
- add workaround to print the current log file name - ([5770dc3](https://github.com/x-software-com/mxl-base/commit/5770dc36341c4fce26a8d906ec7af3066aa46533)) - acpiccolo
- add workaround to print the current log file name - ([e6cc513](https://github.com/x-software-com/mxl-base/commit/e6cc51322b5dce2f6fe3e0345f848e3758ef5983)) - acpiccolo
#### Build system
- **(deps)** bump crate-ci/typos from 1.18.2 to 1.20.4 - ([8cc8a7c](https://github.com/x-software-com/mxl-base/commit/8cc8a7c414f7ab5fde7a242e4c78cb6cccebe02d)) - dependabot[bot]
- change typos exclude - ([b7d92c1](https://github.com/x-software-com/mxl-base/commit/b7d92c187bca6bc8abd5012c12972c7dd6062718)) - acpiccolo
- removed --locked argument - ([2091604](https://github.com/x-software-com/mxl-base/commit/20916042f51cef0227b2ec4107b198defae1fba8)) - acpiccolo
#### Miscellaneous Chores
- update i18n-embed-fl and i18n-embed versions - ([36b7a55](https://github.com/x-software-com/mxl-base/commit/36b7a55a43a6461e7faaee1fdaba158d0d4765b8)) - acpiccolo

- - -

## [v0.2.3](https://github.com/x-software-com/mxl-base/compare/v0.2.2..v0.2.3) - 2024-03-11
#### Bug Fixes
- removed panic hook - ([dece613](https://github.com/x-software-com/mxl-base/commit/dece6133fcb5fa0a368fa7e05ae7e461d611b188)) - acpiccolo
#### Refactoring
- cleanup - ([63b169a](https://github.com/x-software-com/mxl-base/commit/63b169a0a5fef0a4cc27524e85a1d3baa0fa26af)) - acpiccolo
- change initialization - ([0731e64](https://github.com/x-software-com/mxl-base/commit/0731e64ce40083e3fa3fbbb4be10c0895860791f)) - acpiccolo

- - -

## [v0.2.2](https://github.com/x-software-com/mxl-base/compare/v0.2.1..v0.2.2) - 2024-03-07
#### Bug Fixes
- replace logging when panicking to prevent possible deadlocks - ([2717844](https://github.com/x-software-com/mxl-base/commit/27178442f335c47fc2a974fa134d105149b81b2a)) - acpiccolo
#### Refactoring
- changed localization init() - ([4cbe065](https://github.com/x-software-com/mxl-base/commit/4cbe06577dc91f3ac449c4a17f20956a4acccb2e)) - acpiccolo
- removed unused translations - ([99e9d71](https://github.com/x-software-com/mxl-base/commit/99e9d71ebd8805b497c8e944fc4a6a55a9341739)) - acpiccolo

- - -

## [v0.2.1](https://github.com/x-software-com/mxl-base/compare/v0.2.0..v0.2.1) - 2024-02-29
#### Bug Fixes
- moved some code to a different component - ([b87d04e](https://github.com/x-software-com/mxl-base/commit/b87d04ec4341e044624fcea71b36db056daebe11)) - acpiccolo
#### Build system
- **(deps)** update fs4 requirement from 0.7 to 0.8 - ([3bd73db](https://github.com/x-software-com/mxl-base/commit/3bd73dbbf3b70856e0b8c7b60ad098ccc648d9ef)) - dependabot[bot]
#### Miscellaneous Chores
- remove third party licenses functionality - ([e74b8e5](https://github.com/x-software-com/mxl-base/commit/e74b8e5bfd614667f26a6d4d9f67dbf965f2a28f)) - marcbull

- - -

## [v0.2.0](https://github.com/x-software-com/mxl-base/compare/v0.1.1..v0.2.0) - 2024-02-26
#### Build system
- add justfile, install just and make setup.py executable - ([42010c9](https://github.com/x-software-com/mxl-base/commit/42010c9598d6d2ecaef396cf5e4b5d92104b1f6c)) - marcbull
#### Continuous Integration
- do not install cargo-hack in extra step - ([01568d9](https://github.com/x-software-com/mxl-base/commit/01568d93265aa65a717f28d47befaf992e29ce9f)) - marcbull
- use just in hack job - ([2e9cedb](https://github.com/x-software-com/mxl-base/commit/2e9cedbf0831eeae9d78781f095b5053dcab6def)) - marcbull
- remove rust installation in typo job - ([e24cd65](https://github.com/x-software-com/mxl-base/commit/e24cd6551a78c9ed10784a62eac0e59a9a38b50a)) - marcbull
#### Features
- add third party licenses module - ([e6e2016](https://github.com/x-software-com/mxl-base/commit/e6e201600adb4ed26499996bb69eefd1b3eed519)) - marcbull
#### Refactoring
- make localization macro private - ([0e37a3e](https://github.com/x-software-com/mxl-base/commit/0e37a3eab39fffc671d9ee8e2566c3e9e3e8c982)) - acpiccolo

- - -

## [v0.1.1](https://github.com/x-software-com/mxl-base/compare/v0.1.0..v0.1.1) - 2024-02-22
#### Build system
- update Cargo.toml - ([b2e9adf](https://github.com/x-software-com/mxl-base/commit/b2e9adfbfe5b1c7d6eb089a31d9bc745e0b03a5d)) - acpiccolo

- - -

## [v0.1.0](https://github.com/x-software-com/mxl-base/compare/4a110e7c74dd889e51439c69507517bcf2df08da..v0.1.0) - 2024-02-20
#### Miscellaneous Chores
- Initial commit - ([5892bb0](https://github.com/x-software-com/mxl-base/commit/5892bb00b1cd548c0350238dc4b7c1253071e090)) - acpiccolo

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).