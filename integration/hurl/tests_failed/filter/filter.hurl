GET http://localhost:8000/error-filter
HTTP 200
[Asserts]
jsonpath "$.file" base64Decode == hex,e4bda0e5a5bde4b896e7;
jsonpath "$.number" base64Decode == 123
jsonpath "$.number" base64Encode == "MTIz"
jsonpath "$.file" base64UrlSafeDecode == hex,e4bda0e5a5bde4b896e7;
jsonpath "$.base64_string" base64UrlSafeDecode == hex,e4bda0e5a5bde4b896e7;
jsonpath "$.number" base64UrlSafeDecode == 123
jsonpath "$.number" base64UrlSafeEncode == "MTIz"
jsonpath "$.id" toHex == "d188d0b5d0bbd0bbd18b"
jsonpath "$.id" toInt == 123
jsonpath "$.id" first == 1
jsonpath "$.id" last == 3
jsonpath "$.empty_list" first == "1"
jsonpath "$.empty_list" last == "x"
jsonpath "$.status" toInt == 0
jsonpath "$.unknown" toInt == 1
jsonpath "$.list" nth 5 == 3
jsonpath "$.id" toDate "%a, %d %b %Y %H:%M:%S GMT" == "unused"
jsonpath "$.id" daysAfterNow == 1
jsonpath "$.id" daysBeforeNow == 1
jsonpath "$.id" decode "utf-8" == "help"
jsonpath "$.id" dateFormat "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
jsonpath "$.number" htmlEscape == "a &gt; b"
jsonpath "$.number" htmlUnescape == "a > b"
regex /(.*number.*)/ jsonpath "$.name" == "help"
bytes jsonpath "$.name" == "help"
jsonpath "$.id" nth 0 == 123
jsonpath "$.list" nth -4 == 123
jsonpath "$.number" regex /Hello (.*)!/ == "Bob"
jsonpath "$.number" replace ", " "|" == "192.168.2.1|10.0.0.20|10.0.0.10"
jsonpath "$.number" replaceRegex /\d/ "x" == "xxx.xxx.x.x,xx.x.x.xx|xx.x.x.xx"
jsonpath "$.number" split ", " count == 3
jsonpath "$.number" toDate "%Y-%m-%dT%H:%M:%S%.fZ" dateFormat "%A" == "Monday"
jsonpath "$.number" urlDecode == "https://mozilla.org/?x=шеллы"
jsonpath "$.number" urlEncode == "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
jsonpath "$.number" urlQueryParam "x" == "шеллы"
jsonpath "$.number" xpath "string(//body)" == "你好世界"
jsonpath "$.list" toString == "[]"
jsonpath "$.invalid_xml" xpath "normalize-space(//book)" == "foo"
jsonpath "$.id" urlQueryParam "q" == "something"
jsonpath "$.date" toDate "%Y-%m-%dT%H:%M:%S%.fZ" dateFormat "%👻" == "Monday"
jsonpath "$.big_int" toFloat == 10000000000000000365.0
jsonpath "$.id" toFloat == 1.23
jsonpath "$.date" toDate "%+" toFloat == 3.14
jsonpath "$.date" toDate "%Y-%m-%dT%H:%M:%S%.fZ" format "%👻" == "Monday"


GET http://localhost:8000/error-filter
HTTP 200
[Captures]
foo: jsonpath "$.list" jsonpath "$.foo"
