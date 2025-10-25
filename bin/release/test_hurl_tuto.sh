#!/bin/bash
set -Eeuo pipefail

echo "Test Hurl tutorial <https://github.com/jcamiel/hurl-express-tutorial>"
echo "---------------------------------------------------------------------"

work_dir=build/
mkdir -p build/
cd "$work_dir".
rm -rf hurl-express-tutorial || true
git clone --no-depth https://github.com/jcamiel/hurl-express-tutorial
cd hurl-express-tutorial


docker stop movies || true

docker run --rm --detach --quiet \
  --name movies \
  --publish 3000:3000 \
  ghcr.io/jcamiel/hurl-express-tutorial:latest

echo -e "GET http://localhost:3000\n200" | hurl --retry 60 --retry-interval 1s > /dev/null

hurl --variable host="http://localhost:3000" --test --color integration/*.hurl

docker stop movies