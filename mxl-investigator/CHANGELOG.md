# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -

## [0.2.5](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.2.4...mxl-investigator-v0.2.5) - 2025-11-25

### Other

- improved build.rs files

## [0.2.4](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.2.3...mxl-investigator-v0.2.4) - 2025-11-24

### Other

- fix missing dependency
- improved dialogs

## [0.2.3](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.2.2...mxl-investigator-v0.2.3) - 2025-10-30

### Added

- upgrade dependencies relm4 and gstreamer

### Other

- add license documents to any crate

## [0.2.2](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.2.1...mxl-investigator-v0.2.2) - 2025-10-23

### Other

- *(deps)* update zip requirement from 4 to 6
- *(deps)* update sysinfo requirement from 0.36 to 0.37

## [0.2.1](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.2.0...mxl-investigator-v0.2.1) - 2025-07-15

### Fixed

- changed process directory name from "run" to "proc"

## [0.2.0](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.1.24...mxl-investigator-v0.2.0) - 2025-07-14

### Other

- [**breaking**] change behavior when a directory is deleted or retained
- *(deps)* update sysinfo requirement from 0.35 to 0.36

## [0.1.24](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.1.23...mxl-investigator-v0.1.24) - 2025-07-04

### Other

- *(deps)* update zip requirement from 2 to 4
- *(deps)* update sysinfo requirement from 0.33 to 0.35
- fix clippy warnings, use variables directly in the format string
- update README links to point to mxl-crates
- correct typo in comment
- fix clippy issues

## [0.1.23](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.1.22...mxl-investigator-v0.1.23) - 2025-03-05

### Other

- *(mxl-investigator)* upgrade fs4 to 0.13

## [0.1.22](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.1.21...mxl-investigator-v0.1.22) - 2025-02-27

### Added

- add button to open the report file directory, closes #13

### Other

- update rust edition to 2024

## [0.1.21](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.1.20...mxl-investigator-v0.1.21) - 2024-12-19

### Other

- format backtrace in panics

## [0.1.20](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.1.19...mxl-investigator-v0.1.20) - 2024-12-12

### Other

- *(deps)* update sysinfo requirement from 0.32 to 0.33
- replaced dependency backtrace
- *(deps)* update fs4 requirement from 0.11 to 0.12
- replaced once_cell dependency

## [0.1.19](https://github.com/x-software-com/mxl-crates/compare/mxl-investigator-v0.1.18...mxl-investigator-v0.1.19) - 2024-11-20

### Other

- *(mxl-investigator)* fix nightly clippy warning
- *(mxl-investigator)* fix unused_variables warning when sysinfo feature is disabled
- cleanup cargo dependencies

## [v0.1.18](https://github.com/x-software-com/mxl-investigator/compare/992895ce29d3778d3a01f0b6041e889780e2471a..v0.1.18) - 2024-09-02
#### Miscellaneous Chores
- update mxl-relm4-components dependency to v0.2.2 - ([992895c](https://github.com/x-software-com/mxl-investigator/commit/992895ce29d3778d3a01f0b6041e889780e2471a)) - acpiccolo

- - -

## [v0.1.17](https://github.com/x-software-com/mxl-investigator/compare/384a9d4386f9cead68905f973925a0fabfb5509b..v0.1.17) - 2024-09-02
#### Features
- add method to execute commands and write stdout and stderr to files in the proc directory - ([0c2c862](https://github.com/x-software-com/mxl-investigator/commit/0c2c862e60a91ce1d6ab67c7e3ab056d90708faf)) - acpiccolo
- add callback before directory archiving - ([599b047](https://github.com/x-software-com/mxl-investigator/commit/599b0475970502bcda5041eee7eb7fb5afc9ebf9)) - acpiccolo
- user interface use a thread for report archive creation - ([85c9bf9](https://github.com/x-software-com/mxl-investigator/commit/85c9bf97fce0c374dbd600d0867c08742a0e2294)) - acpiccolo
- add check to prevent error if there are files in proc directory - ([384a9d4](https://github.com/x-software-com/mxl-investigator/commit/384a9d4386f9cead68905f973925a0fabfb5509b)) - acpiccolo
#### Miscellaneous Chores
- increase versions of dependencies - ([8587e19](https://github.com/x-software-com/mxl-investigator/commit/8587e19528d6172abd90e0d65d300f01e3048880)) - acpiccolo
#### Refactoring
- move system information feature - ([875a565](https://github.com/x-software-com/mxl-investigator/commit/875a565e15b14f7b2057805de205ed4ca184c5ad)) - acpiccolo
- move code - ([54b3823](https://github.com/x-software-com/mxl-investigator/commit/54b382336f873781aa7641f90f90a13484b93365)) - acpiccolo

- - -

## [v0.1.16](https://github.com/x-software-com/mxl-investigator/compare/bb48a1880c7e826e8a60eb36d63d255ded52c554..v0.1.16) - 2024-08-27
#### Features
- write file with system information - ([bb48a18](https://github.com/x-software-com/mxl-investigator/commit/bb48a1880c7e826e8a60eb36d63d255ded52c554)) - acpiccolo

- - -

## [v0.1.15](https://github.com/x-software-com/mxl-investigator/compare/9e235652f79a4826434602ca2b7caad03ea949ab..v0.1.15) - 2024-07-19
#### Miscellaneous Chores
- upgrade mxl-relm4-components dependency - ([d083c58](https://github.com/x-software-com/mxl-investigator/commit/d083c58b55d522233ee7ed58753efbf370929572)) - acpiccolo
- use glib::clone!() - ([9e23565](https://github.com/x-software-com/mxl-investigator/commit/9e235652f79a4826434602ca2b7caad03ea949ab)) - acpiccolo

- - -

## [v0.1.14](https://github.com/x-software-com/mxl-investigator/compare/d5f360e6f294808ad37752cd1610573c58ddfcac..v0.1.14) - 2024-07-18
#### Build system
- **(deps)** update VCPKG version to 2024.05.24 - ([9a79418](https://github.com/x-software-com/mxl-investigator/commit/9a79418b6525c2932d84422e4ed6fea03478ec37)) - marcbull
- **(deps)** update relm4-icons to 0.9 and mxl-relm4-components to v0.2.0 - ([1efbfbc](https://github.com/x-software-com/mxl-investigator/commit/1efbfbc820a45f14baca3117039dbd1e1dd26ecb)) - marcbull
- **(deps)** update vcpkg-ports-mxl submodule - ([08bb494](https://github.com/x-software-com/mxl-investigator/commit/08bb494326c47c8f0b8f4b537f37617007afec02)) - marcbull
#### Miscellaneous Chores
- **(deps)** bump crate-ci/typos from 1.20.4 to 1.23.2 (#19) - ([d5f360e](https://github.com/x-software-com/mxl-investigator/commit/d5f360e6f294808ad37752cd1610573c58ddfcac)) - dependabot[bot]

- - -

## [v0.1.13](https://github.com/x-software-com/mxl-investigator/compare/78fde496705ed4c61956208d0f67b1579c088093..v0.1.13) - 2024-07-12
#### Miscellaneous Chores
- update dependencies - ([78fde49](https://github.com/x-software-com/mxl-investigator/commit/78fde496705ed4c61956208d0f67b1579c088093)) - acpiccolo

- - -

## [v0.1.12](https://github.com/x-software-com/mxl-investigator/compare/4236f0b17756c9bdfe7bc7d95c8c8f8b3c1ca876..v0.1.12) - 2024-04-16
#### Miscellaneous Chores
- **(deps)** update trash requirement from 3 to 4 - ([e4265af](https://github.com/x-software-com/mxl-investigator/commit/e4265af6efd6a7467272965dc0093c44402e987e)) - dependabot[bot]
- **(deps)** bump crate-ci/typos from 1.18.2 to 1.20.4 - ([4236f0b](https://github.com/x-software-com/mxl-investigator/commit/4236f0b17756c9bdfe7bc7d95c8c8f8b3c1ca876)) - dependabot[bot]
- cleanup failed directory also on startup - ([f776e7d](https://github.com/x-software-com/mxl-investigator/commit/f776e7d7f347ae59cdd71933dfe84f4debb854c6)) - acpiccolo

- - -

## [v0.1.11](https://github.com/x-software-com/mxl-investigator/compare/026690549dbf9aeb477c8157b5c9e634526bb15b..v0.1.11) - 2024-04-08
#### Build system
- upgrade mxl-relm4-components - ([08ffa97](https://github.com/x-software-com/mxl-investigator/commit/08ffa97efbded70cd9c837d009560971fa95ef78)) - acpiccolo
- upgrade vcpkg to 2024.03.25 - ([2dd8e66](https://github.com/x-software-com/mxl-investigator/commit/2dd8e66759dc457ffb2b4d1a808a7abd3ca0aaa4)) - acpiccolo
- change typos exclude - ([a94bb89](https://github.com/x-software-com/mxl-investigator/commit/a94bb8974035ee82003b518ec9262c6368bfc058)) - acpiccolo
- exclude *.md files from typos - ([2607f3e](https://github.com/x-software-com/mxl-investigator/commit/2607f3e19779079af277d5aa9fcf56fcd08c509d)) - acpiccolo
- removed --locked argument - ([0266905](https://github.com/x-software-com/mxl-investigator/commit/026690549dbf9aeb477c8157b5c9e634526bb15b)) - acpiccolo

- - -

## [v0.1.10](https://github.com/x-software-com/mxl-investigator/compare/6bbb10508940898e4da1c2e6331a0df235d87306..v0.1.10) - 2024-04-08
#### Miscellaneous Chores
- upgrade mxl-relm4-components - ([6bbb105](https://github.com/x-software-com/mxl-investigator/commit/6bbb10508940898e4da1c2e6331a0df235d87306)) - acpiccolo

- - -

## [v0.1.9](https://github.com/x-software-com/mxl-investigator/compare/1e669f30804a9d64d7960550bbfcb73fe7937dfa..v0.1.9) - 2024-04-08
#### Miscellaneous Chores
- update vcpkg submodule - ([1e669f3](https://github.com/x-software-com/mxl-investigator/commit/1e669f30804a9d64d7960550bbfcb73fe7937dfa)) - acpiccolo

- - -

## [v0.1.8](https://github.com/x-software-com/mxl-investigator/compare/b0afa2a1a17871c2f646b8e90babebb9ef264dfb..v0.1.8) - 2024-04-08
#### Bug Fixes
- fixed report file creation - ([b0afa2a](https://github.com/x-software-com/mxl-investigator/commit/b0afa2a1a17871c2f646b8e90babebb9ef264dfb)) - acpiccolo

- - -

## [v0.1.7](https://github.com/x-software-com/mxl-investigator/compare/38b6102fdb436fcde1ae844c53e85d1cfea18565..v0.1.7) - 2024-03-21
#### Bug Fixes
- inisialization - ([38b6102](https://github.com/x-software-com/mxl-investigator/commit/38b6102fdb436fcde1ae844c53e85d1cfea18565)) - acpiccolo

- - -

## [v0.1.6](https://github.com/x-software-com/mxl-investigator/compare/a959db2f72beab0760543519910640a9e101083d..v0.1.6) - 2024-03-21
#### Bug Fixes
- initialization - ([a959db2](https://github.com/x-software-com/mxl-investigator/commit/a959db2f72beab0760543519910640a9e101083d)) - acpiccolo

- - -

## [v0.1.5](https://github.com/x-software-com/mxl-investigator/compare/13401b139e09c798d75457b085cfde0aabcb9cf3..v0.1.5) - 2024-03-21
#### Bug Fixes
- fix initialization - ([1faeae9](https://github.com/x-software-com/mxl-investigator/commit/1faeae99802ccdfa71bf8ff7a648898a279c133f)) - acpiccolo
#### Refactoring
- cleanup code - ([13401b1](https://github.com/x-software-com/mxl-investigator/commit/13401b139e09c798d75457b085cfde0aabcb9cf3)) - acpiccolo

- - -

## [v0.1.4](https://github.com/x-software-com/mxl-investigator/compare/v0.1.3..v0.1.4) - 2024-03-21
#### Miscellaneous Chores
- upgrade mxl-relm4-components - ([35f5552](https://github.com/x-software-com/mxl-investigator/commit/35f555229cd5bf8fb1a5772bb2a54e5383b152b1)) - acpiccolo

- - -

## [v0.1.3](https://github.com/x-software-com/mxl-investigator/compare/v0.1.2..v0.1.3) - 2024-03-11
#### Bug Fixes
- add panic handling - ([f2cae2d](https://github.com/x-software-com/mxl-investigator/commit/f2cae2dbd7bb226b59f8f0ec9cf05850dfb05c4b)) - acpiccolo

- - -

## [v0.1.2](https://github.com/x-software-com/mxl-investigator/compare/v0.1.1..v0.1.2) - 2024-03-07
#### Bug Fixes
- fix initialization - ([15eeebd](https://github.com/x-software-com/mxl-investigator/commit/15eeebd7709e7acaff38668e50885cfc55f8e1d2)) - acpiccolo

- - -

## [v0.1.1](https://github.com/x-software-com/mxl-investigator/compare/v0.1.0..v0.1.1) - 2024-03-06
#### Refactoring
- upgrade mxl-relm4-components - ([e6a0f2f](https://github.com/x-software-com/mxl-investigator/commit/e6a0f2fd525e1b7059ce0033d4628b84dc9f8b01)) - acpiccolo

- - -

## [v0.1.0](https://github.com/x-software-com/mxl-investigator/compare/eb79d534fa3962db8c47aff5e5e4bd9012752e24..v0.1.0) - 2024-02-29

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).