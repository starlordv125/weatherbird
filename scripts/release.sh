#!/bin/bash

# Create tar archive
cp target/release/weatherbird package/weatherbird-linux-amd64
cp LICENSE package/weatherbird-linux-amd64
cp README.md package/weatherbird-linux-amd64
cd package
tar -czvf ./weatherbird-linux-amd64.tar.gz ./weatherbird-linux-amd64
cd ../

# Create release
curl -X 'POST' \
'https://forgejo.starlordv125.net/api/v1/repos/starlordv125/weatherbird/releases' \
-H 'accept: application/json' \
-H "Authorization: token $API_KEY" \
-H 'Content-Type: application/json' \
-d "{
"body": "$MESSAGE",
"draft": false,
"hide_archive_links": false,
"name": "v$VERSION",
"prerelease": false,
"tag_name": "v$VERSION",
"target_commitish": "main"
}"

# Upload binary
curl -X 'POST' \
"https://forgejo.starlordv125.net/api/v1/repos/starlordv125/weatherbird/releases/$RELEASE/assets?name=weatherbird-linux-amd64.tar.gz" \
-H 'accept: application/json' \
-H "authorization: token $API_KEY" \
-H 'Content-Type: multipart/form-data' \
-F 'attachment=@./package/weatherbird-linux-amd64.tar.gz;type=text/x-dsrc'