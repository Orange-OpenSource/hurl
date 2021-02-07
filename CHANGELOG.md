[1.1.0 (2021-02-07)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.1.0)
========================================================================================================================

Changes:

* Add windows build documentation [#150](https://github.com/Orange-OpenSource/hurl/issues/150)

* Add verbose output when a ssl error occurs [#145](https://github.com/Orange-OpenSource/hurl/issues/145)

* Migrate integration scripts to python [#126](https://github.com/Orange-OpenSource/hurl/issues/126)

* Add option --interactive [#121](https://github.com/Orange-OpenSource/hurl/issues/121)

* Improve Template Support in JSON body [#116](https://github.com/Orange-OpenSource/hurl/issues/116)

* Update to Rust 1.49.0 [#112](https://github.com/Orange-OpenSource/hurl/issues/112)

* Add option --variables-file  / --variables [#42](https://github.com/Orange-OpenSource/hurl/issues/42)


Bugs Fixes:

* Insecure mode for a full session [#143](https://github.com/Orange-OpenSource/hurl/issues/143)

* Display error message when hurl input can not be decoded [#139](https://github.com/Orange-OpenSource/hurl/issues/139)

* Cookie value in cookie section doesn't accept some value [#132](https://github.com/Orange-OpenSource/hurl/issues/132)

* Running cargo test in windows [#128](https://github.com/Orange-OpenSource/hurl/issues/128)

* Input Cookie file [#124](https://github.com/Orange-OpenSource/hurl/issues/124)



[1.0.0 (2020-12-18)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.0.0)
========================================================================================================================

Changes:

* Update to Rust 1.48.0 [#107](https://github.com/Orange-OpenSource/hurl/issues/107)

* Add type predicates [#98](https://github.com/Orange-OpenSource/hurl/pull/98)

* Arithmetic predicates for number (Integer or Float) [#95](https://github.com/Orange-OpenSource/hurl/issues/95)

* Add predicates to test value types [#94](https://github.com/Orange-OpenSource/hurl/issues/94)

* Add duration query [#90](https://github.com/Orange-OpenSource/hurl/issues/90)

* Add comparison predicates [#89](https://github.com/Orange-OpenSource/hurl/issues/89)


Bugs Fixes:

* Serialization of cookie query for Expires attributes with hurlfmt [#100](https://github.com/Orange-OpenSource/hurl/issues/100)

* Valid Jsonpath query is not parsed [#93](https://github.com/Orange-OpenSource/hurl/issues/93)





[0.99.14 (2020-11-17)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#0.99.14)
========================================================================================================================

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
