[1.6.1 (2022-03-25)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.6.1)
========================================================================================================================

Thanks to
[@humphd](https://github.com/humphd)

Bugs Fixes:

* Support @ for the username in [BasicAuth] section [#513](https://github.com/Orange-OpenSource/hurl/issues/513)

* Fix panicking while processing expected Regex value [#514](https://github.com/Orange-OpenSource/hurl/issues/514)


[1.6.0 (2022-02-10)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.6.0)
========================================================================================================================

Thanks to
[@linjan](https://github.com/linjan),
[@adworacz](https://github.com/adworacz),
[@humphd](https://github.com/humphd),
[@jpluscplusm](https://github.com/humphd),


Changes:

* Add -A/--user-agent curl option to specify a User Agent for all requests of a file [#452](https://github.com/Orange-OpenSource/hurl/issues/452)

* Support filter with nested object in jsonpath expression [#423](https://github.com/Orange-OpenSource/hurl/issues/423)

* Add BasicAuth section [#360](https://github.com/Orange-OpenSource/hurl/issues/360)

* Add next request in interactive mode [#268](https://github.com/Orange-OpenSource/hurl/issues/268)

* Improving pattern for regex capture and matches predicates [#4](https://github.com/Orange-OpenSource/hurl/issues/4)


Bugs Fixes:

* Add additional characters in cookie value [#466](https://github.com/Orange-OpenSource/hurl/issues/466)

* Add square brackets in key-string [#457](https://github.com/Orange-OpenSource/hurl/issues/457)

* Fix Build in Alpine [#448](https://github.com/Orange-OpenSource/hurl/issues/448)

* Check that data file is a child of user provided context dir [#405](https://github.com/Orange-OpenSource/hurl/issues/405)

* Report error on missing closing quote for quoted String [#403](https://github.com/Orange-OpenSource/hurl/issues/403)



[1.5.0 (2021-12-09)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.5.0)
========================================================================================================================

Thanks to
[@humphd](https://github.com/humphd),
[@tbolon](https://github.com/tbolon),
[@ansscfc](https://github.com/ansscfc),
[@atcol](https://github.com/atcol),
[@realtica](https://github.com/realtica),
[@ramkumarkb](https://github.com/ramkumarkb)

Changes:

* Support globs with --test arg [#387](https://github.com/Orange-OpenSource/hurl/issues/387)

* Add ARM Testing with Circle CI [#335](https://github.com/Orange-OpenSource/hurl/issues/335)

* Option --html renamed to --report-html [#333](https://github.com/Orange-OpenSource/hurl/issues/333)

* Add JUnit XML Report Output [#326](https://github.com/Orange-OpenSource/hurl/issues/326)

* Add option --cacert [#314](https://github.com/Orange-OpenSource/hurl/issues/314)

* Display libcurl error code/message [#310](https://github.com/Orange-OpenSource/hurl/issues/310)

* Display curl -V version in hurl -V [#309](https://github.com/Orange-OpenSource/hurl/issues/309)

* Use --json parameter to write JSON to stdout [#283](https://github.com/Orange-OpenSource/hurl/issues/283)

* Using Environment Variables in Hurl files [#122](https://github.com/Orange-OpenSource/hurl/issues/122)



Bugs Fixes:

* Fix memory allocation [#380](https://github.com/Orange-OpenSource/hurl/issues/380)

* Fix Decimal float values [#363](https://github.com/Orange-OpenSource/hurl/issues/363)

* Fix build for ARM [#334](https://github.com/Orange-OpenSource/hurl/issues/334)



[1.4.0 (2021-10-18)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.4.0)
========================================================================================================================

Thanks to 
[@youhavethewrong](https://github.com/youhavethewrong),
[@fourjay](https://github.com/fourjay),
[@tbolon](https://github.com/tbolon),
[@Morreski](https://github.com/Morreski),
[@andrejohansson](https://github.com/andrejohansson)

Changes:

* Install via Scoop [#289](https://github.com/Orange-OpenSource/hurl/issues/289)

* Support spaces in filenames [#287](https://github.com/Orange-OpenSource/hurl/issues/287)

* Remove deprecated option --append [#262](https://github.com/Orange-OpenSource/hurl/issues/262)

* Improve HTML output for hurlfmt [#260](https://github.com/Orange-OpenSource/hurl/issues/260)

* Add option --ignore-asserts [#254](https://github.com/Orange-OpenSource/hurl/issues/254)


Bugs Fixes:

* Support tilde in URL [#294](https://github.com/Orange-OpenSource/hurl/issues/294)

* Fix Windows Terminal output for non-UTF-8 byte sequences [#292](https://github.com/Orange-OpenSource/hurl/issues/292)

* Fix asserts entries in JSON report [#286](https://github.com/Orange-OpenSource/hurl/issues/286)

* Fix --test mode in Windows (/dev/null) [#273](https://github.com/Orange-OpenSource/hurl/issues/273)

* Support key with underscore in jsonpath dot notation [#269](https://github.com/Orange-OpenSource/hurl/issues/269)

* Fix Windows installer PATH [#267](https://github.com/Orange-OpenSource/hurl/issues/267)


[1.3.1 (2021-09-11)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.3.1)
========================================================================================================================

Bugs Fixes:

* Accept hyphen in variable name [#258](https://github.com/Orange-OpenSource/hurl/issues/258)

* Support # in header value [#255](https://github.com/Orange-OpenSource/hurl/issues/255)


[1.3.0 (2021-09-03)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.3.0)
========================================================================================================================

Changes:

* Simplify JSON/HTML report generation [#241](https://github.com/Orange-OpenSource/hurl/issues/241)

* Add --progress option to print progressive status and executed count [#236](https://github.com/Orange-OpenSource/hurl/issues/236)

* Add endsWith predicate [#234](https://github.com/Orange-OpenSource/hurl/issues/234)

* Add --summary option to print tests metrics [#232](https://github.com/Orange-OpenSource/hurl/issues/232)

* Add md5 query [#231](https://github.com/Orange-OpenSource/hurl/issues/231)

* Add Hex body [#230](https://github.com/Orange-OpenSource/hurl/issues/230)

* Add dependency check in the CI [#226](https://github.com/Orange-OpenSource/hurl/pull/226)

* Use startswith/contains predicate with bytearray [#224](https://github.com/Orange-OpenSource/hurl/issues/224)

* Add subquery count  [#217](https://github.com/Orange-OpenSource/hurl/issues/217)

* Add notEquals (!=) predicate [#216](https://github.com/Orange-OpenSource/hurl/issues/216)

* Accept predicate value raw-string and base64 [#215](https://github.com/Orange-OpenSource/hurl/issues/215)

* improve error messages when syntax is not recognized [#213](https://github.com/Orange-OpenSource/hurl/issues/213)

* Add operators for arithmetic predicates [#210](https://github.com/Orange-OpenSource/hurl/issues/210)

* Improve error messages if the url contains an illegal character [#207](https://github.com/Orange-OpenSource/hurl/issues/207)

* Improve Error message for the countEquals predicate [#195](https://github.com/Orange-OpenSource/hurl/issues/195)

* Improve Error Message "Could not Resolve Host" [#194](https://github.com/Orange-OpenSource/hurl/issues/194)

* Add HTTP Headers in the session json file. [#191](https://github.com/Orange-OpenSource/hurl/issues/191)

* Improve Hurl Report [#190](https://github.com/Orange-OpenSource/hurl/issues/190)

* Output curl command-line in verbose mode [#179](https://github.com/Orange-OpenSource/hurl/issues/179)

* Normalize win64 packages names [#178](https://github.com/Orange-OpenSource/hurl/pull/178)

* Add checksum body query (md5, sha1, sha256) [#102](https://github.com/Orange-OpenSource/hurl/issues/102)


Bugs Fixes:

* Fix incorrect JSON export for lessThan predicate [#212](https://github.com/Orange-OpenSource/hurl/issues/212)

* Fix segmentation fault with Invalid XPath Assert [#192](https://github.com/Orange-OpenSource/hurl/issues/192)


[1.2.0 (2021-03-03)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.2.0)
========================================================================================================================

Hurl 1.2.0 is now available for Windows.

Changes:

* Create release for Windows [#174](https://github.com/Orange-OpenSource/hurl/issues/174)

* Run Test Integ in Windows [#160](https://github.com/Orange-OpenSource/hurl/issues/160)

* Support terminal colors in Windows [#159](https://github.com/Orange-OpenSource/hurl/issues/159)

* Update to Rust 1.50.0 [#156](https://github.com/Orange-OpenSource/hurl/issues/156)

* Support Hurl File with UTF8 BOM [#151](https://github.com/Orange-OpenSource/hurl/issues/151)

* Type input variables [#120](https://github.com/Orange-OpenSource/hurl/issues/120)


Bugs Fixes:

* Support wildcard in jsonpath [#169](https://github.com/Orange-OpenSource/hurl/issues/169)

* Reading body file is too slow [#144](https://github.com/Orange-OpenSource/hurl/issues/144)



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
