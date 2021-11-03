# Hurl Contributing Guide

Thank you for investing time in our project!

## Issues

Whether you have discovered a bug, want a new feature in Hurl, or change code, [please fill an issue] before any PR.
We like to discuss things before implementation and want to be sure that: 

- Any new features is coherent with Hurl core values
- You don't waste time on a feature that will not fit Hurl
- All options have been considered if possible

## Pull Requests

Automated tests are run for each commit, and all tests must be green before merge.

Hurl git history is linear, so we ask to rebase your PR before final merge. 

## Hurl Core Values

- Hurl is a first class citizen CLI tool, fast and reliable
- Hurl is a cherry on the top of curl. What you can do with curl, you could do it with Hurl
- Hurl file format is text plain, loosely based on HTTP
- Hurl is multiplatform, working on Linux, macOS, Windows

## How Can You Help ?

- Installing / Packet managers: bundle Hurl for a particular packet manager is welcome. Currently, we built binaries for
Linux, macOS, Windows and we support a narrow set of packet manager. [More would be better!]
- IDE Support: everything from color syntax (in VSCode, Vim, IntelliJ, TextMate etc...) would be a good idea. An 
integrated way to run Hurl file would be cool also
- [Documentation] is a never finished work and could be always improve. Don't hesitate to clarify, fix typos etc...
- Report bugs: if possible some simple repro steps with the Hurl version, name of the platform etc...





[please fill an issue]: https://github.com/Orange-OpenSource/hurl/issues
[More would be better!]: https://github.com/BurntSushi/ripgrep#installation
[Documentation]: https://github.com/BurntSushi/ripgrep#installation
