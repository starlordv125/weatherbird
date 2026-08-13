#!/bin/bash

# Sign RPM
rpmsign -k ./decoded-key.gpg -p $GPG_PASS -n "Cameron Reynolds <cameron@starlordv125.net>" -s "target/generate-rpm/weatherbird-$VERSION-2.x86_64.rpm" -f "_##" -o signed_rpm/

# Upload RPM
curl --user starlordv125:$API_KEY \
--upload-file signed_rpm/weatherbird-$VERSION-2.x86_64.rpm \
https://forgejo.starlordv125.net/api/packages/starlordv125/rpm/upload