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


# We check that parsed JSON are reused across queries.
GET http://localhost:8000/large/json
HTTP 200
Content-Encoding: gzip
Content-Type: application/json
[Asserts]
jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15


# Captures are reusing parsing cache also.
GET http://localhost:8000/large/json
HTTP 200
Content-Encoding: gzip
Content-Type: application/json
[Captures]
var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count


# Captures and asserts are reusing parsing cache.
GET http://localhost:8000/large/json
HTTP 200
Content-Encoding: gzip
Content-Type: application/json
[Captures]
var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

var1: jsonpath "$.parse.title"
var2: jsonpath "$.parse.pageid"
var3: jsonpath "$.parse.revid"
var4: jsonpath "$.parse.categories" count
var5: jsonpath "$.parse.images" count
var6: jsonpath "$.parse.links" count
var7: jsonpath "$.parse.sections" count

[Asserts]
jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

jsonpath "$.parse.title" == "List of Alfred Hitchcock Presents episodes"
jsonpath "$.parse.pageid" == 1892897
jsonpath "$.parse.revid" == 1232145637
jsonpath "$.parse.categories" count == 6
jsonpath "$.parse.images" count == 2
jsonpath "$.parse.links" count == 1626
jsonpath "$.parse.sections" count == 15

# A call without explicit asserts to give a comparison for the first call.
GET http://localhost:8000/large/json
HTTP 200
Content-Encoding: gzip
Content-Type: application/json
