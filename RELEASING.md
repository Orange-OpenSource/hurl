# Releasing Process

We always have to start with current version x.y.0-snapshot (in all Cargo.toml).

## CHANGELOG

- Add Enhancement or Bug label to the issue
- Add target milestone to the issue
- Use a well formatted description on PR (starts with a verb)
- Add link(s) to the issue

## Release steps

- Run `release.yml` workflow on `master` branch, it will:
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
- Accept the PR from `release/x.y.0` to `master` with `/accept`
- Run `update-branch-version.yml` workflow on `master` branch, filling in the `desired SNAPSHOT version`, it will:
  - Create `bot/update-branch-version-master` branch
  - Checkout this new branch
  - Update all toml, crates, and man with `desired SNAPSHOT version`
  - Commit all updates
  - Create PR from `bot/update-branch-version-master` to `master`
- Accept the PR from `bot/update-branch-version-master` to `master` with `/accept`

## Hotfix steps

- Create a new branch `release/x.y.z` from desired tag `x.y.0`, for example `release/1.8.1` from tag `1.8.0`
- Run `update-branch-version.yml` workflow on the new branch filling version field with `x.y.z-SNAPSHOT`, for example `1.8.1-SNAPSHOT` for `release/1.8.1` branch
- Run release.yml workflow on `release/x.y.z` branch, it will:
  - Clean pending release
  - Checkout `release/x.y.z` branch
  - Update all toml with `x.y.z`
  - Update all crate with `x.y.z`
  - Update man and docs with `x.y.z`
  - Generate CHANGELOG
  - Commit all updates
  - Create the `x.y.z` tag
  - Create draft GitHub release `x.y.z`
  - Create PR from `release/x.y.z` to `master`
- You have to manually `merge` as a revert rebase to reorder commits between this new hotfix and master
- Run `update-branch-version.yml` workflow on `master` to restore actual dev version to `x.y+1.0-SNAPSHOT`, for example from `1.8.1` to `1.9.0-SNAPSHOT`

## Additional

- Push package to Chocolatey
- Push package to winget
- Push package to Docker
- Push package to Brew
- Push source packages to crates.io

