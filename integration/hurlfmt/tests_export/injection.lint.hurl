# This file checks that regex literal are well escaped and forbid JavaScript injections.
GET https://foo.com
HTTP 200
[Asserts]
jsonpath "$.body" matches /<img src="" onerror="alert('Hi!')">/
