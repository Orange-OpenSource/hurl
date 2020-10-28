0.99.13 (2020-10-28)
=====================================

Hurl 0.99.13 now uses libcurl under the hood instead of reqwest rust crate.
This makes hurl even closer to curl in terms of behavior and semantic.


Feature enhancements:

* Improve Cookie Asserts [FEATURE #5](https://github.com/Orange-OpenSource/hurl/issues/5)

* Request Cookies Section should not change cookie store [FEATURE #25](https://github.com/Orange-OpenSource/hurl/issues/25)

* Uncompress response body for queries [FEATURE #35](https://github.com/Orange-OpenSource/hurl/issues/35)

* Add option --compressed [FEATURE #36](https://github.com/Orange-OpenSource/hurl/issues/36)

* Predicates with not qualifier [FEATURE #39](https://github.com/Orange-OpenSource/hurl/issues/39)

* Support Multiple Content-Encoding (at the same time) [FEATURE #40](https://github.com/Orange-OpenSource/hurl/issues/40)

* Add option -u, --user [FEATURE #41](https://github.com/Orange-OpenSource/hurl/issues/41)

* Do not add header Expect automatically [FEATURE #44](https://github.com/Orange-OpenSource/hurl/issues/44)

* Add timeout option (--connect-timeout and --max-time) [FEATURE #30](https://github.com/Orange-OpenSource/hurl/issues/30)

* Add option --compressed [FEATURE #34](https://github.com/Orange-OpenSource/hurl/issues/34)

* Decompress response body [FEATURE #38](https://github.com/Orange-OpenSource/hurl/issues/38)



Bug fixes:

* Fix Host" request header with specific port [BUG #6](https://github.com/Orange-OpenSource/hurl/issues/6)

* Fix Assert with different types of values [BUG #37](https://github.com/Orange-OpenSource/hurl/issues/37)



0.99.12 (2020-08-27)
=====================================

Initial Release (beta)
