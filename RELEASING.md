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

- Create a new branch `hotfix/x.y.z` from desired tag `x.y.0`, for example `hotfix/1.8.1` from tag `1.8.0`
- Run `update-branch-version.yml` workflow on the new branch filling version field with `x.y.z`
- Run release.yml workflow on `hotfix/x.y.z` branch, it will:
  - Clean pending release
  - Create new `release/x.y.z` branch
  - Checkout this new branch
  - Update all toml, crates, man and docs with `x.y.z`
  - Generate CHANGELOG
  - Commit all updates
  - Create the `x.y.z` tag
  - Create draft GitHub release `x.y.`
  - Create PR from `release/x.y.z` to `master`
- Publish the draft release
- You have to manually `merge` as a revert rebase to reorder commits between this new release and master

## Additional

- Push package to Chocolatey
- Push package to winget
- Push package to Docker
- Push package to Brew
- Push source packages to crates.io

