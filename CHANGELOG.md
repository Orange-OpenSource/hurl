[0.99.14 (2020-11-17)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#0.99.14)
==========================================================================================================

Changes:

* Update hurlfmt usage [#81](https://github.com/Orange-OpenSource/hurl/issues/81)

* Migrate fully to Github Actions [#69](https://github.com/Orange-OpenSource/hurl/issues/69)

* Add Hurl File JSON export  [#65](https://github.com/Orange-OpenSource/hurl/issues/65)

* Support wildcard value in implicit status code reponse [#55](https://github.com/Orange-OpenSource/hurl/issues/55)


Bugs Fixes:

* Can not parse user in url (Basic Authentication) [#73](https://github.com/Orange-OpenSource/hurl/issues/73)

* MultipartFormData is not present in json export [#63](https://github.com/Orange-OpenSource/hurl/issues/63)

* Hurl usage doesn't end with newline  [#60](https://github.com/Orange-OpenSource/hurl/issues/60)






[0.99.13 (2020-10-28)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#0.99.13)
==========================================================================================================

Hurl 0.99.13 now uses libcurl under the hood instead of reqwest rust crate.
This makes hurl even closer to curl in terms of behavior and semantic.


Changes:

* Improve Cookie Asserts [#5](https://github.com/Orange-OpenSource/hurl/issues/5)

* Request Cookies Section should not change cookie store [#25](https://github.com/Orange-OpenSource/hurl/issues/25)

* Uncompress response body for queries [#35](https://github.com/Orange-OpenSource/hurl/issues/35)

* Add option --compressed [#36](https://github.com/Orange-OpenSource/hurl/issues/36)

* Predicates with not qualifier [#39](https://github.com/Orange-OpenSource/hurl/issues/39)

* Support Multiple Content-Encoding (at the same time) [#40](https://github.com/Orange-OpenSource/hurl/issues/40)

* Add option -u, --user [#41](https://github.com/Orange-OpenSource/hurl/issues/41)

* Do not add header Expect automatically [#44](https://github.com/Orange-OpenSource/hurl/issues/44)

* Add timeout option (--connect-timeout and --max-time) [#30](https://github.com/Orange-OpenSource/hurl/issues/30)

* Add option --compressed [#34](https://github.com/Orange-OpenSource/hurl/issues/34)

* Decompress response body [#38](https://github.com/Orange-OpenSource/hurl/issues/38)



Bugs Fixes:

* Fix Host" request header with specific port [#6](https://github.com/Orange-OpenSource/hurl/issues/6)

* Fix Assert with different types of values [#37](https://github.com/Orange-OpenSource/hurl/issues/37)


[0.99.12 (2020-08-27)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#0.99.12)
==========================================================================================================

Initial Release (beta)
