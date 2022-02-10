# Releasing Process

We are starting with current version x.y.0-snapshot (in Cargo.toml).

Releasing a new version of Hurl will create a release x.y.0
and update master to x.(y+1).0-snapshot

Steps:
1. Update CHANGELOG (COMMIT)
2. Update README (COMMIT)
3. Remove -snapshot suffix in Cargo.toml to x.y.0 (COMMIT)
4. Create Tag/Release x.y.0 
5. Copy CHANGELOG into Release x.y.0
6. Upload artifacts into Release x.y.0
7. Push to crates.io   
8. Increase Version in Cargo.toml to x.(y+1).0 (COMMIT)


## CHANGELOG

- Issues are grouped into Changes (Enhancement) and Bug Fixes.
- Use description from PR (starts with a verb)
- Add link(s) to related issue(s)