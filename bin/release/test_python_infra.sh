#!/bin/bash
set -Eeuo pipefail

echo "Test Python infra Hurl tests <https://github.com/python/psf-salt>"
echo "-----------------------------------------------------------------"

work_dir=build/
mkdir -p build/
cd "$work_dir".
rm -rf psf-salt || true
git clone --no-depth https://github.com/python/psf-salt.git
cd psf-salt


docker stop docs-redirects-nginx || true

docker run --rm --detach --quiet \
  --name docs-redirects-nginx \
  --tty \
  --publish 10000:10000 \
  --mount type=bind,source=./tests/docs-redirects/nginx.conf,target=/etc/nginx/conf.d/docs.conf,readonly \
  --mount type=bind,source=./salt/docs/config/nginx.docs-redirects.conf,target=/etc/nginx/docs-redirects.conf,readonly \
  nginx:1.26.1-alpine

echo -e "GET http://localhost:10000\n302" | hurl --retry 60 --retry-interval 1s > /dev/null

hurl --color --continue-on-error --variable host=http://localhost:10000 --test ./tests/docs-redirects/specs/*.hurl

docker stop docs-redirects-nginx
