# This entry should pass with --max-filesize=255
GET http://localhost:8000/return-100-bytes
HTTP 200


# This entry should not pass with --max-filesize=255
GET http://localhost:8000/return-256-bytes
HTTP 200


# Check that --max-filesize=255 is respected even when there
# is a redirect
GET http://localhost:8000/redirect-to-return-256-bytes
HTTP 302


# See <https://github.com/Orange-OpenSource/hurl/issues/3245>
# On curl < 8.4.0, curl does and completes the transfer when it cannot
# detect upfront the size of resource.
GET https://unpkg.com/vue@3.4.27/dist/vue.global.prod.js
HTTP 200

