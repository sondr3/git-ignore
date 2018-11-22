## 0.2.0
> 2018-11-23

Minor refactoring of how the command line arguments work, instead of using
subcommands we instead only have a single flag (`--list`) to toggle whether
you're listing available templates or getting them. The rest are comments and
behind-the-scenes fixes.

* [[`fe802b4888`](https://github.com/sondr3/git-ignore/commit/fe802b4888)] - Document all the things!
* [[`dfd8bbb235`](https://github.com/sondr3/git-ignore/commit/dfd8bbb235)] - Deny stupid things I shouldn't do
* [[`710779fa05`](https://github.com/sondr3/git-ignore/commit/710779fa05)] - Update README \[ci skip\]
* [[`44b49163f2`](https://github.com/sondr3/git-ignore/commit/44b49163f2)] - Go from subcommands to flags instead, because it makes more sense
* [[`2bd95735db`](https://github.com/sondr3/git-ignore/commit/2bd95735db)] - Add changelog \[ci skip\]

## 0.1.1
> 2018-11-05

Don't mind the patch release, it's just there cause I goofed up. This is the
initial release of `git-ignore`, a small and simple tool that allows you to
quickly and easily list and get all the templates that exists on
www.gitignore.io.

* [[`902e94eb61`](https://github.com/sondr3/git-ignore/commit/902e94eb61)] - Fix badge displaying the wrong URL on crates.io 
* [[`efc83813aa`](https://github.com/sondr3/git-ignore/commit/efc83813aa)] - Don't link to the wrong project, whoops \[ci skip\] 
* [[`8d363f2b93`](https://github.com/sondr3/git-ignore/commit/8d363f2b93)] - Add README \[ci skip\] 
* [[`9389ff9ff8`](https://github.com/sondr3/git-ignore/commit/9389ff9ff8)] - Add LICENSE, update name of package on crates.io and metadata 
* [[`420b4eba87`](https://github.com/sondr3/git-ignore/commit/420b4eba87)] - We cannot print stuff we cannot pipe to .gitignore 
* [[`f55376c734`](https://github.com/sondr3/git-ignore/commit/f55376c734)] - Include Cargo.lock since this is an application 
* [[`27ecae60ba`](https://github.com/sondr3/git-ignore/commit/27ecae60ba)] - Fetch and print gitignore templates 
* [[`93bbd19770`](https://github.com/sondr3/git-ignore/commit/93bbd19770)] - Make sure any matches are included 
* [[`3045c6fefd`](https://github.com/sondr3/git-ignore/commit/3045c6fefd)] - Silence errors 
* [[`edeafa8992`](https://github.com/sondr3/git-ignore/commit/edeafa8992)] - Format with rustfmt and fix Clippy lints 
* [[`64e9e21b37`](https://github.com/sondr3/git-ignore/commit/64e9e21b37)] - List all matches found 
* [[`c7beeb9472`](https://github.com/sondr3/git-ignore/commit/c7beeb9472)] - List all possible templates from gitignore.io 
* [[`11d66fcf6f`](https://github.com/sondr3/git-ignore/commit/11d66fcf6f)] - Add a very simple CLI interface 
* [[`7c077a87fa`](https://github.com/sondr3/git-ignore/commit/7c077a87fa)] - Forbid the usage of unsafe, fail on warnings and add Travis config 
* [[`7a65ce7915`](https://github.com/sondr3/git-ignore/commit/7a65ce7915)] - In the beginning there was darkness...
