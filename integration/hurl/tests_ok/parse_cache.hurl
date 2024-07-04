# This file test that response are parsed only once even if there
# is multiple XPath asserts.
GET http://localhost:8000/large/html
HTTP 200
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
[Asserts]
xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364


# Captures query should be cached also
GET http://localhost:8000/large/html
HTTP 200
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
[Captures]
var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count


# Captures and asserts should share their query cache.
GET http://localhost:8000/large/html
HTTP 200
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
[Captures]
var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

var1: xpath "//h1" count
var2: xpath "//h2" count
var3: xpath "//h3" count
var4: xpath "//h4" count
var5: xpath "//p" count
var6: xpath "//a" count

[Asserts]
xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364

xpath "//h1" count == 1
xpath "//h2" count == 6
xpath "//h3" count == 10
xpath "//h4" count == 0
xpath "//p" count == 397
xpath "//a" count == 3364


# A call without explicit asserts to give a comparison for the first call.
GET http://localhost:8000/large/html
HTTP 200
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
