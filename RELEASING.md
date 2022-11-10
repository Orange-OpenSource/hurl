# Releasing Process

We always have to start with current version x.y.0-snapshot (in all Cargo.toml).

## CHANGELOG

- Add enhancement or Bug label to the issue
- Add target milestone to the issue
- Use a well formatted description on PR (starts with a verb)
- Add link(s) to the issue

## Release steps

- Run `release.yml` workflow on `master` branch, it will:
  - Clean pending release
  - Create new `release/x.y.0` branch
  - Checkout this new branch
  - Update all toml with `x.y.0`
  - Update all crate with `x.y.0`
  - Update man and docs with `x.y.0`
  - Generate CHANGELOG
  - Commit all updates
  - Create the `x.y.0` tag
  - Create draft github release `x.y.0`
  - Create PR from `release/x.y.0` to `master`
- You have to `/accept --release` this PR, it will:
  - Merge fast-forward this PR
  - Open a new one to update `master` files to next version `x.y+1.0-SNAPSHOT`

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
  - Create draft github release `x.y.z`
  - Create PR from `release/x.y.z` to `master`
- You have to manually `merge` as a revert rebase to reorder commits between this new hotfix and  master
- Run `update-branch-version.yml` workflow on `master` to restore actual dev version to `x.y+1.0-SNAPSHOT`, for example from `1.8.1` to `1.9.0-SNAPSHOT`

## Additional

- push package to chocolatey
- push package to winget
- push package to docker
- push package to brew
- push source packages to crates.io

