#!/bin/bash

# Upload RPM
curl --user starlordv125:$API_KEY \
--upload-file target/generate-rpm/weatherbird-$VERSION-3.x86_64.rpm \
https://forgejo.starlordv125.net/api/packages/starlordv125/rpm/upload