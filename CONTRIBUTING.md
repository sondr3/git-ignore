# Release checklist

- [ ] Update version in `Cargo.toml`
- [ ] Create release commit
- [ ] Create a changelog from commits from previous release to this release,
      ammend it onto release commit.
- [ ] Create a annotated tag (`git tag -a vN.N.N`)
- [ ] Publish new release to [Crates](https://crates.io/crates/git-ignore-generator)
- [ ] Update [Homebrew](https://github.com/sondr3/homebrew-taps)
- [ ] Update [Nixpkgs](https://github.com/sondr3/nixpkgs/blob/master/pkgs/applications/version-management/git-and-tools/git-ignore/default.nix)
- [ ] Update AUR package
- [ ] Add changelog entry to release on GitHub
