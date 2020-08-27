#!/bin/bash

set -e
set -u

tag=$1
shift

API_URL="https://api.github.com"
REPO_NAME=hurl
repo_url="$API_URL/repos/Orange-OpenSource/$REPO_NAME"
auth_header="Authorization: token $GITHUB_API_TOKEN"

echo "Uploading to $REPO_NAME for tag $tag"
release_id=$(curl -s -H "$auth_header" "$repo_url/releases/tags/$tag" | jq  '.id')
upload_url=https://uploads.github.com/repos/Orange-OpenSource/$REPO_NAME/releases/$release_id/assets
echo "release from tag: $release_id"


asset_files=$*

for asset_file in $asset_files; do
  echo "Uploading asset file $asset_file"
  if [ ! -f "$asset_file" ]; then
     echo "does not exist!"
     exit 1
  fi
  asset_name=$(basename "$asset_file")
  asset_url="https://github.com/Orange-OpenSource/$REPO_NAME/releases/download/$VERSION/$asset_name"
  echo "asset url: $asset_url"

   if curl --output /dev/null --silent --head --fail "$asset_url"; then
          echo "File already uploaded"
          exit 1
   fi

  curl -v -X POST \
      -H "$auth_header" \
      -H "Content-Type: application/octet-stream" \
      --data-binary "@$asset_file" "$upload_url?name=$asset_name"

done



