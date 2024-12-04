# Releasing Process

We always have to start with current version x.y.0-snapshot (in all Cargo.toml).

## CHANGELOG

- Add Enhancement or Bug label to the issue
- Add target milestone to the issue
- Use a well formatted description on PR (starts with a verb)
- Add link(s) to the issue

## Release steps

Used to publish a new release from master branch (normal process).

- Run `release.yml` workflow on `master` branch
- Fill `Desired delivery version` input with the `x.y.z` version you want to publish, it will:
  - Clean pending release
  - Create new `release/x.y.0` branch
  - Checkout this new branch
  - Update all toml, crates, man and docs with `x.y.0`
  - Generate CHANGELOG
  - Commit all updates
  - Create the `x.y.0` tag
  - Create draft GitHub release `x.y.0`
  - Create PR from `release/x.y.0` to `master`
  - Publish the draft release
- Test release on external test project
- Change the release status from draft to public on github
- Accept the PR from `release/x.y.0` to `master` with `/accept`
- Run `extra-package.yml` workflow on `master` branch  filling in the `desired tag version` and selecting  wanted extra packages
  - Once `push to chocolatey` is done, all notifications about validation process will be sent to lepapareil's mail
  - To make `push to winget` work, please wait for the message "Initiating GitHub login..." in job log and follow the guide !  
- Run `update-branch-version.yml` workflow on `master` branch, filling in the `desired SNAPSHOT version`, it will:
  - Create `bot/update-branch-version-master` branch
  - Checkout this new branch
  - Update all toml, crates, and man with `desired SNAPSHOT version`
  - Commit all updates
  - Create PR from `bot/update-branch-version-master` to `master`
- Accept the PR from `bot/update-branch-version-master` to `master` with `/accept`

## Hotfix steps

Used when you want to deliver a fix from a published release (tag).

- Create a new branch `release/x.y.z` from desired tag `x.y.z` by increasing the patch version, for example `release/4.0.1` from tag `4.0.0`
- Run `release.yml` workflow on existing `release/x.y.z` branch
- Fill `Desired delivery version` input your `x.y.z` version, it will:
  - Clean pending release
  - Checkout this new branch
  - Update all toml, crates, man and docs with `x.y.z`
  - Generate CHANGELOG
  - Commit all updates
  - Create the `x.y.z` tag
  - Create draft GitHub release `x.y.z`
  - Create PR from `release/x.y.z` to `master`
  - Publish the draft release
- Change the release status from draft to public on github
- Close the PR from `release/x.y.z` to `master` and manage it manually rebasing commits to reorder history and keep it linear

## Additional

- Push package to [Chocolatey](https://github.com/Orange-OpenSource/hurl/tree/master/contrib/windows/windows_package_managers/chocolatey)
- Push package to [Winget](https://github.com/Orange-OpenSource/hurl/tree/master/contrib/windows/windows_package_managers/winget)
- Push package to [Docker](contrib/docker)
- Push package to [Brew](https://github.com/Orange-OpenSource/hurl/tree/master/contrib/brew)
- Push source packages to crates.io

