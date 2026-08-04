#!/bin/bash

# Create Debian control file
echo "Source: weatherbird
Section: main
Priority: optional
Maintainer: Cameron Reynolds <cameron@starlordv125.net>
Homepage: https://forgejo.starlordv125.net/starlordv125/weatherbird
Package: weatherbird
Version: $VERSION-1
Architecture: amd64
Depends:
Description: A simple weather program written in Rust.
Distribution: trixie
Component: main" >> package/deb/weatherbird/DEBIAN/control

# Bash autocompletion
echo '#!/bin/bash

_Weatherbird() {
local cur
cur=${COMP_WORDS[COMP_CWORD]}
COMPREPLY=( $(compgen -W "set days hours --help --version" -- "$cur") )
}
complete -F _Weatherbird weatherbird' >> package/deb/weatherbird/usr/share/bash-completion/completions/weatherbird

# Man compression
cp man/weatherbird.1 package/deb/weatherbird/usr/share/man/man1
gzip package/deb/weatherbird/usr/share/man/man1/weatherbird.1

# Debian packaging
cp target/release/weatherbird package/deb/weatherbird/usr/bin
cd package/deb
chmod -R 755 weatherbird
dpkg-deb --root-owner-group --build weatherbird
curl --user starlordv125:$API_KEY \
--upload-file ./weatherbird.deb \
https://forgejo.starlordv125.net/api/packages/starlordv125/debian/pool/trixie/main/upload