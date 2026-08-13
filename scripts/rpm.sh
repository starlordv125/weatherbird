#!/bin/bash

curl --user starlordv125:$API_KEY \
--upload-file signed_rpm/weatherbird-$VERSION-2.x86_64.rpm \
https://forgejo.starlordv125.net/api/packages/starlordv125/rpm/upload