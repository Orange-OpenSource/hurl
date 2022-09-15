# Releasing Process

We are starting with current version x.y.0-snapshot (in Cargo.toml).

Releasing a new version of Hurl will create a release M.m.0
and update master to M.(m+1).0-snapshot

## Steps

0. Create branch/PR release/M.m.0
1. Update CHANGELOG from Issues/PR
2. Update Cargo.toml (x3) remove -snapshot suffix and Cargo.lock
3. Update version in docs/installations.md
4. Regenerate man pages and README
5. Commit
6. Tag M.n.0
7. Create GitHub Release
8. Copy Changelog and upload artifacts
9. Merge (Fast forward) release branch to master
10. Increase Version in Cargo.toml to M.(m+1).0-snapshot
11. Commit
12. Upload packages to external package managers

## CHANGELOG

- Issues are grouped into Changes (Enhancement) and Bug Fixes.
- Use description from PR (starts with a verb)
- Add link(s) to related issue(s)

## Additional

- push source packages (checkout tag) to crates.io

