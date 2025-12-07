# PKGBUILD[^1]

Format for building Arch Linux packages.

PKGBUILD is a bash script.

## Sources

Sources are to be specified in the `source` array:

```bash
source=("name::url")
```
(`name::` part can be ommited, and `makepkg` will choose the name instead.)

Files will be locates at `${srcdir}/name`.

By default, `makepkg` may extract the archives. Add their names to the `noextract`
array if this is not desired.

```bash
noextract=("name")
```

`makepkg` will strip binaries by default. Add `options+=('!strip')` to remove this
behavior, i.e. AppImage's will break if this is not disabled[^2].

Sha256 hashes can be specified in the `sha256sums` array (or `md5sums` for MD5 hashes, highly
unrecommended though[^5]) in order as appears in `source` array. `SKIP` can be used to skip
hashing for a source. Hash checks are performed before extraction.

```bash
source=("a", "b", "c")
sha256sums=("SKIP"
            "0263829989b6fd954f72baaf2fc64bc2e2f01d692d4de72986ea808f6e99813f"
            "SKIP")
```

## Kinds of sources

### Local file

Local files are located in the same directory as `PKGBUILD` and are added by specifying their names.

Default name is the same as of the last path segment.

```bash
# Located at <PKGROOT>/a, symlinked at `${srcdir}/a`
source=("a")
```

### Git

Git repos are added by specifying the git URL prepended by `git+`.

Default name is the same as of the last path segment (i.e. repo name on GitHub).

```bash
# Symlinked at `${srcdir}/linux`
source=("git+https://github.com/torvalds/linux")
```

For specifying the tag, `#tag=<..>` must be added at the end.

```bash
# Will use commit `7d0a66e`
source=("git+https://github.com/torvalds/linux#tag=v6.18")
```

### HTTP source

Similar to local file, but with a link instead.

Default name is the same as of the last path segment.

```bash
# Symlinked at `${srcdir}/README`
source=("https://raw.githubusercontent.com/torvalds/linux/refs/heads/master/README")
```

## Building and installing

To build a package, use `$ makepkg -si`. By default, `makepkg` will not rebuild the package if it
already was. Add `-f` flag to force it to.

## Publishing

`.SRCINFO`[^4] must be generated before publishing the package: `$ makepkg --printsrcinfo > .SRCINFO`.

[^1]: https://wiki.archlinux.org/title/PKGBUILD
[^2]: https://bbs.archlinux.org/viewtopic.php?pid=1970683#p1970683
[^3]: https://man.archlinux.org/man/makepkg.8 (or `$ man 8 makepkg`)
[^4]: https://wiki.archlinux.org/title/.SRCINFO
[^5]: https://www.mscs.dal.ca/~selinger/md5collision/
