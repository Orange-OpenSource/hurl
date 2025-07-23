[7.0.0 (TBD)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#7.0.0)
========================================================================================================================

Thanks to
[@ecolinet](https://github.com/ecolinet),
[@theoforger](https://github.com/theoforger),
[@Muntaner](https://github.com/Muntaner),
[@ashishajr](https://github.com/ashishajr),
[@patkujawa-wf](https://github.com/patkujawa-wf),
[@niklasweimann](https://github.com/niklasweimann),
[@alanbondarun](https://github.com/alanbondarun),
[@benkio](https://github.com/benkio),
[@dhth](https://github.com/dhth),
[@verigak](https://github.com/verigak),
[@markphilpot](https://github.com/markphilpot),
[@lambrospetrou](https://github.com/lambrospetrou),
[@aresler](https://github.com/aresler),
[@nfj25](https://github.com/nfj25),
[@nwellnhof](https://github.com/nwellnhof),
[@YannickAlex07](https://github.com/YannickAlex07),
[@lu-zero](https://github.com/lu-zero),
[@RaghavSood](https://github.com/RaghavSood),

Breaking Changes:

* Add replaceRegex filter and fix replace filter to not take regex [#4018](https://github.com/Orange-OpenSource/hurl/issues/4018)


Enhancements:

* Add query for HTTP redirects [#922](https://github.com/Orange-OpenSource/hurl/issues/922)
* Add urlQueryParam filter [#2199](https://github.com/Orange-OpenSource/hurl/issues/2199)
* Show curl command when error format option is set to long [#2226](https://github.com/Orange-OpenSource/hurl/issues/2226)
* Add option to configure max-time per request [#3162](https://github.com/Orange-OpenSource/hurl/issues/3162)
* Add date comparison predicates [#3480](https://github.com/Orange-OpenSource/hurl/issues/3480)
* Add pinnedpubkey cli option [#3563](https://github.com/Orange-OpenSource/hurl/issues/3563)
* Add base64 url safe encode and decode filters [#3840](https://github.com/Orange-OpenSource/hurl/issues/3840)
* parse curl's --cookie flag [#3877](https://github.com/Orange-OpenSource/hurl/issues/3877)
* Show hurl --help with color [#3882](https://github.com/Orange-OpenSource/hurl/issues/3882)
* Add toHex filter [#3963](https://github.com/Orange-OpenSource/hurl/issues/3963)
* Add first and last filters [#3998](https://github.com/Orange-OpenSource/hurl/issues/3998)
* Remove hex crate dependency [#4011](https://github.com/Orange-OpenSource/hurl/issues/4011)
* Small tweaks to --test progress output [#4028](https://github.com/Orange-OpenSource/hurl/issues/4028)
* Add support for negative values for nth filter [#4050](https://github.com/Orange-OpenSource/hurl/issues/4050)
* Add option to configure pinnedpubkey per request [#4084](https://github.com/Orange-OpenSource/hurl/issues/4084)
* Add timeline link on status label in source and run pages [#4128](https://github.com/Orange-OpenSource/hurl/issues/4128)
* Support template in nth filter parameter [#4152](https://github.com/Orange-OpenSource/hurl/issues/4152)
* Implement predicate `isUuid` [#4179](https://github.com/Orange-OpenSource/hurl/issues/4179)
* Improve captures error messages when filter chain returned no value [#4214](https://github.com/Orange-OpenSource/hurl/issues/4214)
* Add ntlm cli option [#4216](https://github.com/Orange-OpenSource/hurl/issues/4216)


Bugs Fixed:

* Fix incorrect curl command for POST redirect [#2797](https://github.com/Orange-OpenSource/hurl/issues/2797)
* Fix hurlfmt to disallow invalid header argument in curl command [#3668](https://github.com/Orange-OpenSource/hurl/issues/3668)
* Parse verbose flag in curl command [#3760](https://github.com/Orange-OpenSource/hurl/issues/3760)
* Keep secret value forever, even if a secret variable override an existing one [#3898](https://github.com/Orange-OpenSource/hurl/issues/3898)
* Fix zsh completion [#3938](https://github.com/Orange-OpenSource/hurl/issues/3938)
* Parse cookie Expires date attribute with '-' [#3956](https://github.com/Orange-OpenSource/hurl/issues/3956)
* Replace deprecated libxml2 initGenericErrorDefaultFunc with xmlSetGenericErrorFunc [#3975](https://github.com/Orange-OpenSource/hurl/issues/3975)
* HTML report: fix span for lines in comment. [#4002](https://github.com/Orange-OpenSource/hurl/issues/4002)
* Fix HTML closing tag for line with trailing comment [#4017](https://github.com/Orange-OpenSource/hurl/issues/4017)
* Add replaceRegex filter and fix replace filter to not take regex [#4018](https://github.com/Orange-OpenSource/hurl/issues/4018)
* Fix request body during redirections [#4073](https://github.com/Orange-OpenSource/hurl/issues/4073)
* Fix "variables" token in GraphQL HTML export [#4117](https://github.com/Orange-OpenSource/hurl/issues/4117)
* Support negative index in jsonpath [#4154](https://github.com/Orange-OpenSource/hurl/issues/4154)


Security Issues Fixed:

* Fix JavaScript injection in HTML report through regex literal [#4125](https://github.com/Orange-OpenSource/hurl/issues/4125)


[6.1.1 (2025-03-19)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#6.1.1)
========================================================================================================================

Thanks to
[@lu-zero](https://github.com/lu-zero),
[@andrejohansson](https://github.com/andrejohansson),
[@demostanis](https://github.com/demostanis),
[@techfg](https://github.com/techfg),

Bugs Fixed:

* Fix hurlfmt spacing [#3839](https://github.com/Orange-OpenSource/hurl/issues/3839)
* Fix filename parsing [#3848](https://github.com/Orange-OpenSource/hurl/issues/3848)
* Fix jsonpath array wildcard with missing attribute [#3859](https://github.com/Orange-OpenSource/hurl/issues/3859) [#3869](https://github.com/Orange-OpenSource/hurl/issues/3869)
* Fix predicate `contains` with none input [#3868](https://github.com/Orange-OpenSource/hurl/issues/3868)


[6.1.0 (2025-03-12)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#6.1.0)
========================================================================================================================

Thanks to
[@lilyhuang-github](https://github.com/lilyhuang-github),
[@ashishajr](https://github.com/ashishajr),
[@kidbrax](https://github.com/kidbrax),
[@theoforger](https://github.com/theoforger),
[@smokedlinq](https://github.com/smokedlinq),
[@docwhat](https://github.com/docwhat),
[@glb-cblin](https://github.com/glb-cblin),
[@Enoz](https://github.com/Enoz),
[@ikorason](https://github.com/ikorason),
[@uday-rana](https://github.com/uday-rana),
[@lu-zero](https://github.com/lu-zero),
[@nghiab1906724](https://github.com/nghiab1906724),
[@overbyte](https://github.com/overbyte),

Breaking Changes:

* Remove deprecated predicates (notEquals, greaterThan etc...) in favor of operators [#3532](https://github.com/Orange-OpenSource/hurl/issues/3532)
* Remove deprecated keyword HTTP/* for HTTP [#3697](https://github.com/Orange-OpenSource/hurl/issues/3697)


Enhancements:

* Removed limitation for --cookie-jar to use only one hurl file [#2537](https://github.com/Orange-OpenSource/hurl/issues/2537)
* Add HTTP version query [#1706](https://github.com/Orange-OpenSource/hurl/issues/1706)
* Add curl -H/--header option to globally add headers to all requests [#1905](https://github.com/Orange-OpenSource/hurl/issues/1905) [#2144](https://github.com/Orange-OpenSource/hurl/issues/2144)
* Add toString Filter [#2035](https://github.com/Orange-OpenSource/hurl/issues/2035) [#3798](https://github.com/Orange-OpenSource/hurl/issues/3798)
* Add base64 decode filter [#2145](https://github.com/Orange-OpenSource/hurl/issues/2145)
* Add base64 encode filter [#2145](https://github.com/Orange-OpenSource/hurl/issues/2145)
* Redacts secrets from JUnit reports [#2947](https://github.com/Orange-OpenSource/hurl/issues/2947) [#2972](https://github.com/Orange-OpenSource/hurl/issues/2972)
* Redacts secrets from JSON report [#2947](https://github.com/Orange-OpenSource/hurl/issues/2947) [#2972](https://github.com/Orange-OpenSource/hurl/issues/2972)
* Redact secret in HTML report [#2947](https://github.com/Orange-OpenSource/hurl/issues/2947) [#2972](https://github.com/Orange-OpenSource/hurl/issues/2972)
* Redact secrets from curl export [#2947](https://github.com/Orange-OpenSource/hurl/issues/2947) [#2972](https://github.com/Orange-OpenSource/hurl/issues/2972)
* Redact secrets from cookies export [#2947](https://github.com/Orange-OpenSource/hurl/issues/2947) [#2972](https://github.com/Orange-OpenSource/hurl/issues/2972)
* Add IP address query [#3106](https://github.com/Orange-OpenSource/hurl/issues/3106)
* Add isIpv4 / isIpv6 asserts on IP versions [#3106](https://github.com/Orange-OpenSource/hurl/issues/3106)
* Allow sending empty HTTP header [#3536](https://github.com/Orange-OpenSource/hurl/issues/3536)
* Redact dynamic values from logs [#3543](https://github.com/Orange-OpenSource/hurl/issues/3543)
* Add header option per request [#3575](https://github.com/Orange-OpenSource/hurl/issues/3575)
* Fix invalid escape in hurlfmt parse func [#3615](https://github.com/Orange-OpenSource/hurl/issues/3615)
* hurlfmt: Use Hurl predicates identifiers for Hurl to JSON file export [#3662](https://github.com/Orange-OpenSource/hurl/issues/3662)
* Add aarch64 deb package [#3829](https://github.com/Orange-OpenSource/hurl/issues/3829)


Bugs Fixed:

* Fix missing request line errors in HTML report [#3534](https://github.com/Orange-OpenSource/hurl/issues/3534)
* Eval template in JSON object key [#3593](https://github.com/Orange-OpenSource/hurl/issues/3593)
* Show error message if format is invalid in `format` filter [#3613](https://github.com/Orange-OpenSource/hurl/issues/3613)
* Create parent folders if missing when using --cookie-jar FILE [#3637](https://github.com/Orange-OpenSource/hurl/issues/3637)
* Remove lint errors and Fix non-zero exit code in case of error [#3648](https://github.com/Orange-OpenSource/hurl/issues/3648)
* Support BigInteger in variable [#3656](https://github.com/Orange-OpenSource/hurl/issues/3656)
* fix hurlfmt html export loosing some whitespaces [#3675](https://github.com/Orange-OpenSource/hurl/issues/3675)
* Fix template to source [#3675](https://github.com/Orange-OpenSource/hurl/issues/3675)
* Fix changing HTTP version per request sometimes not effective [#3719](https://github.com/Orange-OpenSource/hurl/issues/3719)
* Add bash file completion for hurl/hurlfmt [#3750](https://github.com/Orange-OpenSource/hurl/issues/3750)
* Fix multilines HTML export [#3768](https://github.com/Orange-OpenSource/hurl/issues/3768)
* Change parsing file content type in multipart form data [#3796](https://github.com/Orange-OpenSource/hurl/issues/3796)


Deprecations:

* Deprecate includes in favor of contains predicate [#1896](https://github.com/Orange-OpenSource/hurl/issues/1896)
* Warn for deprecated multilines string attributes [#3622](https://github.com/Orange-OpenSource/hurl/issues/3622)
* Warn for --interactive deprecation [#3763](https://github.com/Orange-OpenSource/hurl/issues/3763)


[6.0.0 (2024-12-03)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#6.0.0)
========================================================================================================================

Thanks to
[@cemoktra](https://github.com/cemoktra),
[@zikani03](https://github.com/zikani03),
[@lambrospetrou](https://github.com/lambrospetrou),
[@jmvargas](https://github.com/jmvargas),
[@quantonganh](https://github.com/quantonganh),
[@sandeshbhusal](https://github.com/sandeshbhusal),
[@thePanz](https://github.com/thePanz),
[@niklasweimann](https://github.com/niklasweimann),
[@infogulch](https://github.com/infogulch),
[@orlandow](https://github.com/orlandow),
[@bp7968h](https://github.com/bp7968h),

Breaking Changes:

* Check that variables do not conflict with existing functions [#3229](https://github.com/Orange-OpenSource/hurl/issues/3229)
* Remove deprecated --fail-at-end option [#3430](https://github.com/Orange-OpenSource/hurl/issues/3430)
* Change API for setting variable in hurl::runner::run [#3440](https://github.com/Orange-OpenSource/hurl/issues/3440)
* Remove hurlfmt deprecated --format option [#3445](https://github.com/Orange-OpenSource/hurl/issues/3445)
* Rename feature flag from vendored-openssl to static-openssl [#3460](https://github.com/Orange-OpenSource/hurl/issues/3460)


Enhancements:

* Implement function newUuid [#973](https://github.com/Orange-OpenSource/hurl/issues/973)
* Implement --limit-rate from curl [#1222](https://github.com/Orange-OpenSource/hurl/issues/1222)
* Add --curl option to export executed requests to curl commands [#2679](https://github.com/Orange-OpenSource/hurl/issues/2679)
* Configure --connect-timeout per request [#3163](https://github.com/Orange-OpenSource/hurl/issues/3163)
* Support short name for sections [QueryStringParams] => [Query], [FormParams] => [Form], [MultipartFormData] => [Multipart] [#3238](https://github.com/Orange-OpenSource/hurl/issues/3238)
* Remove url-specific parser (align with grammar) [#3244](https://github.com/Orange-OpenSource/hurl/issues/3244)
* Remove the crate float-cmp [#3247](https://github.com/Orange-OpenSource/hurl/issues/3247)
* Jsonpath / Add filter on boolean value  [#3252](https://github.com/Orange-OpenSource/hurl/issues/3252)
* Jsonpath / Add non-equal filter on string and number value [#3261](https://github.com/Orange-OpenSource/hurl/issues/3261)
* Add support for backtick strings in predicates values [#3317](https://github.com/Orange-OpenSource/hurl/issues/3317)
* Categorise options in --help [#3339](https://github.com/Orange-OpenSource/hurl/issues/3339)
* Support more JSON / XML "like" mimetypes with debug output [#3343](https://github.com/Orange-OpenSource/hurl/issues/3343)
* Add curl debug command to --json and JSON report [#3374](https://github.com/Orange-OpenSource/hurl/issues/3374)
* Add curl debug command to HTML report [#3386](https://github.com/Orange-OpenSource/hurl/issues/3386)
* Render Date value [#3431](https://github.com/Orange-OpenSource/hurl/issues/3431)
* Add newDate generator [#3443](https://github.com/Orange-OpenSource/hurl/issues/3443)


Bugs Fixed:

* Fix reading standard input multiple times [#3216](https://github.com/Orange-OpenSource/hurl/issues/3216)
* Fix filename parsing (used by cert option) [#3242](https://github.com/Orange-OpenSource/hurl/issues/3242)
* Add additional check for --max-filesize option [#3245](https://github.com/Orange-OpenSource/hurl/issues/3245)
* Support case-insensitive Cookie Attributes [#3265](https://github.com/Orange-OpenSource/hurl/issues/3265)
* Allow any string in Location Header when not following redirection [#3293](https://github.com/Orange-OpenSource/hurl/issues/3293)
* Fix graceful shutdown of workers threads in --test [#3297](https://github.com/Orange-OpenSource/hurl/issues/3297)
* Fix missing space in variable option HTML export [#3412](https://github.com/Orange-OpenSource/hurl/issues/3412)


[5.0.1 (2024-08-30)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#5.0.1)
========================================================================================================================

Bugs Fixed:

* Fix regression in --output when output file doesn't exist [#3195](https://github.com/Orange-OpenSource/hurl/issues/3195)

[5.0.0 (2024-08-29)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#5.0.0)
========================================================================================================================

Thanks to
[@lambrospetrou](https://github.com/lambrospetrou),
[@OverkillGuy](https://github.com/OverkillGuy),
[@badboy](https://github.com/badboy),
[@DenuxPlays](https://github.com/DenuxPlays),
[@linkdd](https://github.com/linkdd),
[@nkback](https://github.com/nkback),
[@claytonneal](https://github.com/claytonneal),


Breaking Changes:

* Encode oneline string only with one backtick [#3113](https://github.com/Orange-OpenSource/hurl/issues/3113)


Enhancements:

* Create intermediary directories if necessary when producing TAP report [#2860](https://github.com/Orange-OpenSource/hurl/issues/2860)
* Expose request comments in --out json (hurlfmt) [#2850](https://github.com/Orange-OpenSource/hurl/issues/2850)
* Create intermediary directories if necessary when producing JUnit report [#2842](https://github.com/Orange-OpenSource/hurl/issues/2842)
* Add global requests count in test summary [#2832](https://github.com/Orange-OpenSource/hurl/issues/2832)
* Replace output warnings by errors [#2815](https://github.com/Orange-OpenSource/hurl/issues/2815)
* Fix inconsistent case for fields queryString and httpVersion in --json [#2804](https://github.com/Orange-OpenSource/hurl/issues/2804)
* Run tests in parallel [#2753](https://github.com/Orange-OpenSource/hurl/issues/2753)
* Add support for importing curl url option in hurlfmt [#2750](https://github.com/Orange-OpenSource/hurl/issues/2750)
* Add JSON report [#2738](https://github.com/Orange-OpenSource/hurl/issues/2738)
* Add repeat option to repeat a sequence of Hurl file [#2680](https://github.com/Orange-OpenSource/hurl/issues/2680)
* Add repeat option per request [#2680](https://github.com/Orange-OpenSource/hurl/issues/2680)
* Add optional duration unit [#2653](https://github.com/Orange-OpenSource/hurl/issues/2653)
* Apply delay only once per entry, no matter how many retry [#1973](https://github.com/Orange-OpenSource/hurl/issues/1973)
* Add toFloat filter [#1732](https://github.com/Orange-OpenSource/hurl/issues/1732)
* Accept directory as Hurl arguments for processing file [#1446](https://github.com/Orange-OpenSource/hurl/issues/1446)


Bugs Fixed:

* Get SSL certificates info on reused connections, from a cache [#3031](https://github.com/Orange-OpenSource/hurl/issues/3031)
* Fix max-redirs: -1 in [Options] section [#3023](https://github.com/Orange-OpenSource/hurl/issues/3023)
* Fix error displayed in double with bad option [#2920](https://github.com/Orange-OpenSource/hurl/issues/2920)
* Fix hurlfmt exit code with lint error [#2919](https://github.com/Orange-OpenSource/hurl/issues/2919)
* Truncate file then append it when dumping response with --output. [#2886](https://github.com/Orange-OpenSource/hurl/issues/2886)
* Fix crash with --json option when capturing 'HttpOnly' and 'Secure' cookie attribute [#2871](https://github.com/Orange-OpenSource/hurl/issues/2871)
* Fix crash when capturing 'Expires' cookie attribute [#2870](https://github.com/Orange-OpenSource/hurl/issues/2870)
* Fix empty JSON key parsing [#2836](https://github.com/Orange-OpenSource/hurl/issues/2836)
* Do not filter 'Authorization' header if host doesn't change while following redirect [#2823](https://github.com/Orange-OpenSource/hurl/issues/2823)
* Fix sending 'Authorization' header from --user when following redirect [#2812](https://github.com/Orange-OpenSource/hurl/issues/2812)
* Fix URL when following redirect for certain 'Location' header [#2783](https://github.com/Orange-OpenSource/hurl/issues/2783)
* Fix powershell completion file [#2729](https://github.com/Orange-OpenSource/hurl/issues/2729)


[4.3.0 (2024-04-23)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#4.3.0)
========================================================================================================================

Thanks to
[@tarampampam](https://github.com/tarampampam),
[@CMiksche](https://github.com/CMiksche),
[@CodeMan99](https://github.com/CodeMan99),
[@OverkillGuy](https://github.com/OverkillGuy),
[@hsanson](https://github.com/hsanson),
[@jaminalder](https://github.com/jaminalder),
[@teto](https://github.com/teto),
[@humphd](https://github.com/humphd),
[@mohammed90](https://github.com/mohammed90),

Enhancements:

* Support --user option per request [#2585](https://github.com/Orange-OpenSource/hurl/issues/2585)
* Add isNumber predicate [#2538](https://github.com/Orange-OpenSource/hurl/issues/2538)
* Create intermediate directory when using --report-html [#2531](https://github.com/Orange-OpenSource/hurl/issues/2531)
* Use '-' to read Hurl file from standard input [#2523](https://github.com/Orange-OpenSource/hurl/issues/2523)
* Add --from-entry option to execute a file from a given entry [#2500](https://github.com/Orange-OpenSource/hurl/issues/2500)
* Add isIsoDate predicate (take a string, checks YYYY-MM-DDTHH:mm:sssZ) [#2427](https://github.com/Orange-OpenSource/hurl/issues/2427)
* Add completion files in Linux/MacOS packages [#2401](https://github.com/Orange-OpenSource/hurl/issues/2401)
* Fix tarball layout to Linux filesystem [#2401](https://github.com/Orange-OpenSource/hurl/issues/2401)
* Add --max-filesize option to limit HTTP response [#2353](https://github.com/Orange-OpenSource/hurl/issues/2353)
* Display source request when there are asserts/runtime errors [#2351](https://github.com/Orange-OpenSource/hurl/issues/2351)
* Using explicit stdout output [#2312](https://github.com/Orange-OpenSource/hurl/issues/2312)
* Prevent raw binary response to be displayed on standard output [#2306](https://github.com/Orange-OpenSource/hurl/issues/2306)
* Add --netrc, --netrc-file and --netrc-optional options [#2094](https://github.com/Orange-OpenSource/hurl/issues/2094)
* Generate bash completion for hurl/hurlfmt [#1864](https://github.com/Orange-OpenSource/hurl/issues/1864)
* Generate powershell completion for hurl/hurlfmt [#1864](https://github.com/Orange-OpenSource/hurl/issues/1864)
* Generate fish completion for hurl/hurlfmt [#1864](https://github.com/Orange-OpenSource/hurl/issues/1864)
* Generate zsh completion for hurl/hurlfmt [#1864](https://github.com/Orange-OpenSource/hurl/issues/1864)
* Add experimental --parallel / --jobs options [#88](https://github.com/Orange-OpenSource/hurl/issues/88) [#87](https://github.com/Orange-OpenSource/hurl/issues/87)


Bugs Fixed:

* Export begin_call, end_call timings fields to RFC3339 (microseconds) [#2699](https://github.com/Orange-OpenSource/hurl/issues/2699)
* Fix standalone css for regex [#2693](https://github.com/Orange-OpenSource/hurl/issues/2693)
* Fix charset parsing logic of Content-Type header [#2540](https://github.com/Orange-OpenSource/hurl/issues/2540)
* Fix filename templatization bug under certain conditions [#2533](https://github.com/Orange-OpenSource/hurl/issues/2533)
* Fix empty glob not always returning an error [#2517](https://github.com/Orange-OpenSource/hurl/issues/2517)
* Fix hurlfmt query certificate for hurl output format [#2511](https://github.com/Orange-OpenSource/hurl/issues/2511)
* Fix --test progress bar not displayed when verbose is used [#2506](https://github.com/Orange-OpenSource/hurl/issues/2506)
* Per request output takes file-root into account for path resolving [#2445](https://github.com/Orange-OpenSource/hurl/issues/2445)
* Fix silent error when verbose option use unset variable [#2444](https://github.com/Orange-OpenSource/hurl/issues/2444)
* Fix Content-type header override when used in lowercase [#2416](https://github.com/Orange-OpenSource/hurl/issues/2416)


Security Issues Fixed:

* Prevent script injection in HTML report [#2719](https://github.com/Orange-OpenSource/hurl/issues/2719)


[4.2.0 (2024-01-11)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#4.2.0)
========================================================================================================================

Thanks to
[@hi2code](https://github.com/hi2code),
[@lambrospetrou](https://github.com/lambrospetrou),
[@glb-cblin](https://github.com/glb-cblin),
[@moono](https://github.com/moono),
[@gmetal](https://github.com/gmetal),
[@mihirn](https://github.com/mihirn),
[@humphd](https://github.com/humphd),
[@RickMoynihan](https://github.com/RickMoynihan),
[@pit1sIBM](https://github.com/pit1sIBM),
[@janwytze](https://github.com/janwytze),
[@kingluo](https://github.com/kingluo),
[@teto](https://github.com/teto),
[@khimaros](https://github.com/khimaros),
[@iredmail](https://github.com/iredmail),
[@andres-lowrie](https://github.com/andres-lowrie),
[@nikeee](https://github.com/nikeee),
[@ztittle](https://github.com/ztittle),
[@legzo](https://github.com/legzo)

Enhancements:

* Add --location-trusted option [#2296](https://github.com/Orange-OpenSource/hurl/issues/2296)
* Add --unix-socket option [#2291](https://github.com/Orange-OpenSource/hurl/issues/2291)
* Export entry source line number in JSON output [#2273](https://github.com/Orange-OpenSource/hurl/issues/2273)
* Use Template for filename type [#2259](https://github.com/Orange-OpenSource/hurl/issues/2259) [#1731](https://github.com/Orange-OpenSource/hurl/issues/1731) [#464](https://github.com/Orange-OpenSource/hurl/issues/464)
* Add dark mode support for HTML report [#2254](https://github.com/Orange-OpenSource/hurl/issues/2254)
* Add --output option per request [#2184](https://github.com/Orange-OpenSource/hurl/issues/2184) [#1326](https://github.com/Orange-OpenSource/hurl/issues/1326)
* Add filter jsonpath [#2134](https://github.com/Orange-OpenSource/hurl/issues/2134) [#1632](https://github.com/Orange-OpenSource/hurl/issues/1632) [#440](https://github.com/Orange-OpenSource/hurl/issues/440)
* Improve JSON body parsing error reporting [#2056](https://github.com/Orange-OpenSource/hurl/issues/2056)
* Support template in option values [#2041](https://github.com/Orange-OpenSource/hurl/issues/2041)
* Support conda-forge installation [#2018](https://github.com/Orange-OpenSource/hurl/issues/2018)
* Add timestamps to the HTML reports [#1983](https://github.com/Orange-OpenSource/hurl/issues/1983)
* Log only non-default options in verbose mode [#1927](https://github.com/Orange-OpenSource/hurl/issues/1927)
* Support template in key string [#1877](https://github.com/Orange-OpenSource/hurl/issues/1877) [#1710](https://github.com/Orange-OpenSource/hurl/issues/1710) [#898](https://github.com/Orange-OpenSource/hurl/issues/898)
* Add skip option [#1815](https://github.com/Orange-OpenSource/hurl/issues/1815)
* Add --ipv4/--ipv6 option [#1727](https://github.com/Orange-OpenSource/hurl/issues/1727)
* Add --http3 option [#1155](https://github.com/Orange-OpenSource/hurl/issues/1155)
* Add --http2 option [#1155](https://github.com/Orange-OpenSource/hurl/issues/1155)
* Add --http1.0/-0 option [#1155](https://github.com/Orange-OpenSource/hurl/issues/1155)
* Add --http1.1 option [#1155](https://github.com/Orange-OpenSource/hurl/issues/1155)


Bugs Fixed:

* Add short name -v for verbose option [#2310](https://github.com/Orange-OpenSource/hurl/issues/2310)
* Fix unicode surrogate pair decoding in JSON request body [#2235](https://github.com/Orange-OpenSource/hurl/issues/2235)
* Better error description for some parse error [#2187](https://github.com/Orange-OpenSource/hurl/issues/2187)
* Fix undefined error for various I/O error using --output. [#2156](https://github.com/Orange-OpenSource/hurl/issues/2156)
* TAP reports can't be appended if there are failed tests [#2099](https://github.com/Orange-OpenSource/hurl/issues/2099)
* Support HTTP/2 on Windows [#2072](https://github.com/Orange-OpenSource/hurl/issues/2072)
* Support key password in --cert option (certificate[:password]) [#2047](https://github.com/Orange-OpenSource/hurl/issues/2047)
* Keep initial request headers when following redirects [#1990](https://github.com/Orange-OpenSource/hurl/issues/1990)


[4.1.0 (2023-09-21)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#4.1.0)
========================================================================================================================

Thanks to
[@danielzfranklin](https://github.com/danielzfranklin),
[@apparentorder](https://github.com/apparentorder),
[@ppaulweber](https://github.com/ppaulweber),
[@taigrr](https://github.com/taigrr),
[@kallelindqvist](https://github.com/kallelindqvist),
[@jlazic](https://github.com/jlazic),
[@Lythenas](https://github.com/Lythenas),
[@Jayshua](https://github.com/Jayshua),
[@chenrui333](https://github.com/chenrui333),
[@nikeee](https://github.com/nikeee),
[@jasonkarns](https://github.com/jasonkarns),
[@humphd](https://github.com/humphd),

Breaking Changes:

* Fix published release packages names [#1951](https://github.com/Orange-OpenSource/hurl/issues/1951)


Enhancements:

* Print host architecture with --version [#1893](https://github.com/Orange-OpenSource/hurl/issues/1893)
* Add the aws-sigv4 option to generate AWS SigV4 signed requests [#1840](https://github.com/Orange-OpenSource/hurl/issues/1840)
* Add delay CLI option [#1832](https://github.com/Orange-OpenSource/hurl/issues/1832)
* Add --delay Option [#1832](https://github.com/Orange-OpenSource/hurl/issues/1832)
* Support RFC-7807 application/problem+json for response body logging as text [#1766](https://github.com/Orange-OpenSource/hurl/issues/1766)
* Rename fail-at-end option with continue-on-error option [#1739](https://github.com/Orange-OpenSource/hurl/issues/1739)
* Add connect-to per request option [#1736](https://github.com/Orange-OpenSource/hurl/issues/1736)
* Add support for --resolve option per request [#1711](https://github.com/Orange-OpenSource/hurl/issues/1711)
* Add TAP report [#1666](https://github.com/Orange-OpenSource/hurl/issues/1666) [#601](https://github.com/Orange-OpenSource/hurl/issues/601)
* Implement isDate predicate [#1520](https://github.com/Orange-OpenSource/hurl/issues/1520)
* Add docker arm64 build [#536](https://github.com/Orange-OpenSource/hurl/issues/536)


Bugs Fixed:

* IsEmpty doesn't seem to work on object collections [#1788](https://github.com/Orange-OpenSource/hurl/issues/1788)
* Fix Cookie Query Parsing error [#1784](https://github.com/Orange-OpenSource/hurl/issues/1784)
* Support empty BasicAuth section [#1772](https://github.com/Orange-OpenSource/hurl/issues/1772)
* Fix standalone option for hurlfmt HTML output [#1759](https://github.com/Orange-OpenSource/hurl/issues/1759)
* Support IPv4/IPv6 address in proxy [Options] [#1756](https://github.com/Orange-OpenSource/hurl/issues/1756)
* Reuse same HTTP method on redirect for appropriate HTTP status codes [#1719](https://github.com/Orange-OpenSource/hurl/issues/1719)
* Fix URL runtime evaluation [#1716](https://github.com/Orange-OpenSource/hurl/issues/1716)


[4.0.0 (2023-06-28)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#4.0.0)
========================================================================================================================

Thanks to
[@elbart](https://github.com/elbart),
[@dross-carve](https://github.com/dross-carve),
[@kaala](https://github.com/kaala),
[@phaza](https://github.com/phaza),
[@FiloSottile](https://github.com/FiloSottile),
[@linker3000](https://github.com/linker3000),
[@pfeiferj](https://github.com/pfeiferj),
[@devnoname120](https://github.com/devnoname120),
[@jasonkarns](https://github.com/jasonkarns),

Breaking Changes:

* The option [--retry](https://hurl.dev/docs/manual.html#retry) now takes a number that specifies the explicit number of retries (same behaviour than curl)
* The `jsonpath` query/filter does not coerce single-entry collection any more [#1469](https://github.com/Orange-OpenSource/hurl/issues/1469)
* `hurl` crate: `LoggerOptionsBuilder` replaces `LoggerOption`, retry option from `RunnerOptionsBuilder` have changed, and verbose option have been moved from `RunnerOptionsBuilder` to `LoggerOptionsBuilder`


Enhancements:

* Add xpath filter [#1698](https://github.com/Orange-OpenSource/hurl/issues/1698)
* Introduce curl --path-as-is option [#1669](https://github.com/Orange-OpenSource/hurl/issues/1669)
* Deprecate word predicate when operator is available [#1662](https://github.com/Orange-OpenSource/hurl/issues/1662)
* Make hurlfmt support several input files (like Hurl) [#1650](https://github.com/Orange-OpenSource/hurl/issues/1650)
* Add timings info to very verbose mode [#1644](https://github.com/Orange-OpenSource/hurl/issues/1644)
* Add waterfall to HTML report for a Hurl file [#1613](https://github.com/Orange-OpenSource/hurl/issues/1613)
* Add proxy in Options section [#1602](https://github.com/Orange-OpenSource/hurl/issues/1602)
* Add decode filter [#1560](https://github.com/Orange-OpenSource/hurl/issues/1560)
* Add --error-format option to output HTTP context on errors [#1542](https://github.com/Orange-OpenSource/hurl/issues/1542)
* Update --retry option to match curl option [#1475](https://github.com/Orange-OpenSource/hurl/issues/1475)
* Add support for LINK, UNLINK, PURGE, LOCK, UNLOCK, PROPFIND and VIEW HTTP method [#967](https://github.com/Orange-OpenSource/hurl/issues/967)
* Support arbitrary HTTP methods [#967](https://github.com/Orange-OpenSource/hurl/issues/967)


Bugs Fixed:

* Export [Options] to JSON [#1673](https://github.com/Orange-OpenSource/hurl/issues/1673)
* Use --data-binary for log curl command when posting file [#1654](https://github.com/Orange-OpenSource/hurl/issues/1654)
* Fix extra request headers logs with large body. [#1651](https://github.com/Orange-OpenSource/hurl/issues/1651)
* Print error message when no file is found with --glob option [#1638](https://github.com/Orange-OpenSource/hurl/issues/1638)
* Support additional dot in jsonpath expression [#1555](https://github.com/Orange-OpenSource/hurl/issues/1555)
* Make Call, Certificate, Cookie, Header, Request, Response, Timings, Version public [#1548](https://github.com/Orange-OpenSource/hurl/issues/1548)
* Add quotes around certificate attribute in HTML export [#1515](https://github.com/Orange-OpenSource/hurl/issues/1515)
* Change API in jsonpath / remove single-entry coercion [#1469](https://github.com/Orange-OpenSource/hurl/issues/1469)


[3.0.1 (2023-06-01)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#3.0.1)
========================================================================================================================

Thanks to
[@RWDai](https://github.com/RWDai),
[@plul](https://github.com/plul),


Bugs Fixed:

* Do not parse key/value in certificate subject/issue any more [#1583](https://github.com/Orange-OpenSource/hurl/issues/1583)
* Fix hurlfmt less predicate [#1577](https://github.com/Orange-OpenSource/hurl/issues/1577)
* Patch encoding issue with xpath and libxml 2.11.1+ [#1535](https://github.com/Orange-OpenSource/hurl/issues/1535)


[3.0.0 (2023-05-03)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#3.0.0)
========================================================================================================================

Thanks to
[@DannyBen](https://github.com/DannyBen), [@dermetfan](https://github.com/dermetfan), [@joesantos418](https://github.com/joesantos418)

3.0.0 is a major update because of breaking changes in Hurl crates.


Enhancements:

* Add test attributes to JUnit XML [#1460](https://github.com/Orange-OpenSource/hurl/issues/1460)
* Add certificate query [#1384](https://github.com/Orange-OpenSource/hurl/issues/1384)
* Add daysAfterNow / daysBeforeNow filters [#1309](https://github.com/Orange-OpenSource/hurl/issues/1309)
* Add errors in HTML report [#1286](https://github.com/Orange-OpenSource/hurl/issues/1286)
* Use long options for curl command [#1236](https://github.com/Orange-OpenSource/hurl/issues/1236)
* Add progress bar for tests [#1224](https://github.com/Orange-OpenSource/hurl/issues/1224)
* Add date value/filters [#1206](https://github.com/Orange-OpenSource/hurl/issues/1206)
* Implement isEmpty predicate [#849](https://github.com/Orange-OpenSource/hurl/issues/849)
* Add curl input to hurlfmt [#316](https://github.com/Orange-OpenSource/hurl/issues/316)


Bugs Fixed:

* Fix performance issue in Reader remaining method [#1456](https://github.com/Orange-OpenSource/hurl/issues/1456)
* Parse empty JSON array body [#1424](https://github.com/Orange-OpenSource/hurl/issues/1424)
* Add meta utf-8 charset to HTML report [#1366](https://github.com/Orange-OpenSource/hurl/issues/1366)
* Use an uuid as identifier for the HTML Hurl file run report [#1285](https://github.com/Orange-OpenSource/hurl/issues/1285) [#1283](https://github.com/Orange-OpenSource/hurl/issues/1283)
* Improve HTML export [#1059](https://github.com/Orange-OpenSource/hurl/issues/1059)


[2.0.1 (2023-02-01)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#2.0.1)
========================================================================================================================

Thanks to
[@softprops](https://github.com/softprops),


Bugs Fixed:

* Fix GraphQL query with variables to HTTP body request [#1218](https://github.com/Orange-OpenSource/hurl/issues/1218)


[2.0.0 (2023-01-25)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#2.0.0)
========================================================================================================================

Thanks to
[@imsVLC](https://github.com/imsVLC),
[@ad8lmondy](https://github.com/ad8lmondy),
[@jlecour](https://github.com/jlecour),
[@ako](https://github.com/ako),
[@jmoore34](https://github.com/jmoore34),
[@robjtede](https://github.com/robjtede),
[@devnoname120](https://github.com/devnoname120),
[@dalejefferson-rnf](https://github.com/dalejefferson-rnf),
[@dnsmichi](https://github.com/dnsmichi),


Enhancements:

* Add option ssl-no-revoke [#1163](https://github.com/Orange-OpenSource/hurl/issues/1163)
* Add client cert/key to command line args and options [#1129](https://github.com/Orange-OpenSource/hurl/issues/1129)
* Add connect-to option [#1079](https://github.com/Orange-OpenSource/hurl/issues/1079)
* Add name attribute to JUnit report [#1078](https://github.com/Orange-OpenSource/hurl/issues/1078)
* Check HTTP version and status first, then other asserts [#1072](https://github.com/Orange-OpenSource/hurl/issues/1072)
* Support new one line string [#1041](https://github.com/Orange-OpenSource/hurl/issues/1041)
* Add filters for htmlEscape and htmlUnescape [#1038](https://github.com/Orange-OpenSource/hurl/issues/1038)
* Add toInt filter [#1029](https://github.com/Orange-OpenSource/hurl/issues/1029)
* Use HTTP instead of HTTP/* for any HTTP version match [#975](https://github.com/Orange-OpenSource/hurl/issues/975)
* Add RunnerOptionsBuilder to create instance of RunnerOptions [#972](https://github.com/Orange-OpenSource/hurl/issues/972)
* Add support for LINK, UNLINK, PURGE, LOCK, UNLOCK, PROPFIND and VIEW HTTP method [#967](https://github.com/Orange-OpenSource/hurl/issues/967)
* Accept multiple --variables-file options [#532](https://github.com/Orange-OpenSource/hurl/issues/532)
* Add GraphQL support [#504](https://github.com/Orange-OpenSource/hurl/issues/504)
* Add --resolve option [#379](https://github.com/Orange-OpenSource/hurl/issues/379)


Bugs Fixed:

* Support '-' in JSONPath dot notation [#1174](https://github.com/Orange-OpenSource/hurl/issues/1174)
* Fix cargo test --doc on Alpine [#1124](https://github.com/Orange-OpenSource/hurl/issues/1124)
* Do not add newline at eof with --no-format option [#1058](https://github.com/Orange-OpenSource/hurl/issues/1058)
* Fix variables update [#1037](https://github.com/Orange-OpenSource/hurl/issues/1037)
* Fix querystring key parsing [#1027](https://github.com/Orange-OpenSource/hurl/issues/1027)


[1.8.0 (2022-11-02)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.8.0)
========================================================================================================================

Thanks to
[@chenrui333](https://github.com/chenrui333),
[@Jiehong](https://github.com/Jiehong),
[@Goffen](https://github.com/Goffen),


Enhancements:

* Add curl logs [#899](https://github.com/Orange-OpenSource/hurl/issues/899)
* Add query url [#895](https://github.com/Orange-OpenSource/hurl/issues/895)
* Make compact help [#861](https://github.com/Orange-OpenSource/hurl/issues/861)
* List all libcurl features with --version [#836](https://github.com/Orange-OpenSource/hurl/issues/836)
* Add --retry and --retry-interval option to retry request until asserts and captures are ok [#525](https://github.com/Orange-OpenSource/hurl/issues/525)


Bugs Fixed:

* Fix hurlfmt --color crash [#957](https://github.com/Orange-OpenSource/hurl/issues/957)
* Fix missing line in HTML output [#924](https://github.com/Orange-OpenSource/hurl/issues/924)
* Fix HTTP HEAD [#903](https://github.com/Orange-OpenSource/hurl/issues/903)
* Fix relative redirect [#875](https://github.com/Orange-OpenSource/hurl/issues/875)


[1.7.0 (2022-09-13)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.7.0)
========================================================================================================================

Thanks to
[@danielbprice](https://github.com/danielbprice),
[@fourjay](https://github.com/fourjay),
[@datamuc](https://github.com/datamuc),
[@bdmorin](https://github.com/bdmorin),
[@humphd](https://github.com/humphd),
[@kautsig](https://github.com/kautsig),
[@Karrq](https://github.com/Karrq),
[@balroggg](https://github.com/balroggg),


Enhancements:

* Add string comparison predicates [#798](https://github.com/Orange-OpenSource/hurl/issues/798)
* Improve text summary [#779](https://github.com/Orange-OpenSource/hurl/issues/779) [#593](https://github.com/Orange-OpenSource/hurl/issues/593)
* Support NO_COLOR env variable (https://no-color.org) [#713](https://github.com/Orange-OpenSource/hurl/issues/713)
* Improve URL parsing error message [#662](https://github.com/Orange-OpenSource/hurl/issues/662)
* Display deprecated warning when using --progress and --summary option [#637](https://github.com/Orange-OpenSource/hurl/issues/637)
* Log body request in very verbose [#628](https://github.com/Orange-OpenSource/hurl/issues/628)
* Add options section [#612](https://github.com/Orange-OpenSource/hurl/issues/612)
* Install Hurl with npm [#544](https://github.com/Orange-OpenSource/hurl/issues/544)
* Add very verbose option [#499](https://github.com/Orange-OpenSource/hurl/issues/499)
* Add support for XML namespaces in XPath query [#493](https://github.com/Orange-OpenSource/hurl/issues/493)
* Use Template type for cookie value [#473](https://github.com/Orange-OpenSource/hurl/issues/473)


Bugs Fixed:

* Accept expression in comparison predicate [#799](https://github.com/Orange-OpenSource/hurl/issues/799)
* Fix file access authorization [#674](https://github.com/Orange-OpenSource/hurl/issues/674)
* Fix body file access [#674](https://github.com/Orange-OpenSource/hurl/issues/674)
* Fix implicit body asserts on compressed response body [#567](https://github.com/Orange-OpenSource/hurl/issues/567)
* Fix Brotli response body decoding [#564](https://github.com/Orange-OpenSource/hurl/issues/564)
* Fix sha256, md5, bytes query on compressed body [#563](https://github.com/Orange-OpenSource/hurl/issues/563)
* Fix redirection not followed [#552](https://github.com/Orange-OpenSource/hurl/issues/552)
* Encode string variables in JSON body [#530](https://github.com/Orange-OpenSource/hurl/issues/530)
* Fix curl export shell escape [#530](https://github.com/Orange-OpenSource/hurl/issues/530)
* Allow different types in JSON array [#495](https://github.com/Orange-OpenSource/hurl/issues/495)


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
* Improve error messages if the URL contains an illegal character [#207](https://github.com/Orange-OpenSource/hurl/issues/207)
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
* Migrate fully to GitHub Actions [#69](https://github.com/Orange-OpenSource/hurl/issues/69)
* Add Hurl File JSON export  [#65](https://github.com/Orange-OpenSource/hurl/issues/65)
* Support wildcard value in implicit status code response [#55](https://github.com/Orange-OpenSource/hurl/issues/55)


Bugs Fixes:

* Can not parse user in URL (Basic Authentication) [#73](https://github.com/Orange-OpenSource/hurl/issues/73)
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
