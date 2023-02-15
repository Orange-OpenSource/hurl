# Hurl Contributing Guide

Thank you for investing time in our project!

## Issues

Whether you have discovered a bug, want a new feature in Hurl, or change code, [please fill an issue] or [start a discussion]
before any PR. We like to discuss things before implementation and want to be sure that: 

- Any new features are coherent with [Hurl core values].
- You don't waste time on a feature that will not fit Hurl.
- All options have been considered if possible.
- We try to minimize dependencies and import new crates parsimoniously.

We really want to be focused and consider any new features carefully before committing to it. A new idea can be really 
relevant to you and we understand it; that's said, we try to reflect on every aspect (maintainability, feature fitting with future evolutions etc...). Don't be too harsh on us if we postpone your proposal, it's for the sake of Hurl!

## Hurl Core Values

- Hurl is a first class citizen CLI tool, fast and reliable.
- Hurl is a cherry on the top of curl. What you can do with curl, you could do it with Hurl.
- Hurl file format is text plain, loosely based on HTTP.
- Hurl is multiplatform, working on Linux, macOS, Windows.

## How Can You Help ?

- Installing / Packet managers: bundle Hurl for a particular packet manager is welcome. Currently, we built binaries for
  Linux, macOS, Windows and we support a narrow set of packet manager. [More would be better!].
- IDE Support: everything from color syntax (in VSCode, Vim, IntelliJ, TextMate etc...) would be a good idea. An
  integrated way to run Hurl file would be cool also.
- [Documentation] is a never finished work and could be always improved. Don't hesitate to clarify, even fix typos etc...
- Report bugs: if possible some simple repro steps with `hurl --version`, name of the platform etc... PR for bugs fixes are really appreciated. If there is [an integration test] that complement it, it's the cherry on the cake.

## Pull Requests

- [Create a new Git branch], don't use `master` branch for PR.
- All Git commits are [required to be signed] and marked as "Verified": signed with a GPG, SSH, or S/MIME that is successfully verified by GitHub.
- All tests must be green before merge. Our CI/CD will run [a test suite] to insure everything is OK.
- Hurl Git history is linear, so we may rebase your PR on your fork before final merge.

## Build and Test

Hurl is a Rust project, so you will need the Rust toolchain to build it. You can check the [Hurl build documentation] to 
see how to build locally the latest version (master branch).

Once your setup is ready, just build the project:

```shell
$ cargo build
   Compiling hurl_core v2.0.0-SNAPSHOT (/Users/jc/Documents/Dev/hurl/packages/hurl_core)
   ...
   Compiling hurlfmt v2.0.0-SNAPSHOT (/Users/jc/Documents/Dev/hurl/packages/hurlfmt)
   Compiling hurl v2.0.0-SNAPSHOT (/Users/jc/Documents/Dev/hurl/packages/hurl)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

Hurl unit and integration tests need Python 3.6+ to be run. You can use a [virtual environment] and install the dependencies needed
by the tests suite:

```shell
$ python3 -m venv .venv
$ source .venv/bin/activate
$ pip3 install --requirement bin/requirements-frozen.txt
```

Then, you can launch our local server (used to test Hurl features):

```shell
$ cd integration
$ python3 server.py >server.log 2>&1 &
$ python3 ssl/server.py >server-ssl.log 2>&1 &
$ mitmdump --listen-host 127.0.0.1 --listen-port 8888 --modify-header "/From-Proxy/Hello" >mitmdump.log 2>&1 &
$ jobs
[1]    running    python3 server.py > server.log 2>&1
[2]  - running    python3 ssl/server.py > server-ssl.log 2>&1
[3]  + running    mitmdump --listen-host 127.0.0.1 --listen-port 8888 --modify-header  >  2>&1
```

You can check [`bin/test/test_prerequisites.sh`] and [`bin/test/test_prerequisites.ps1`] for more details.

Now, you can follow these steps when you make changes:

1. Build `cargo build`
2. Run Clippy `cargo clippy`
3. Format `cargo fmt`
4. Run units tests `cargo test`
5. Run integration tests `cd integration && python3 integration.py`

Et voilà 🎉! 


[please fill an issue]: https://github.com/Orange-OpenSource/hurl/issues
[start a discussion]: https://github.com/Orange-OpenSource/hurl/discussions
[More would be better!]: https://hurl.dev/docs/installation.html
[Documentation]: https://hurl.dev
[Hurl build documentation]: https://hurl.dev/docs/installation.html#building-from-sources
[`bin/test/test_prerequisites.sh`]: /bin/test/test_prerequisites.sh
[`bin/test/test_prerequisites.ps1`]: /bin/test/test_prerequisites.ps1
[virtual environment]: https://docs.python.org/3/tutorial/venv.html
[Hurl core values]: #hurl-core-values
[a test suite]: https://github.com/Orange-OpenSource/hurl/actions
[an integration test]: https://github.com/Orange-OpenSource/hurl/tree/master/integration/tests_ok
[Create a new Git branch]: https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-and-deleting-branches-within-your-repository
[required to be signed]: https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification
