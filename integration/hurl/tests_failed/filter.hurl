GET http://localhost:8000/error-filter
HTTP 200
[Asserts]
jsonpath "$.id" toInt == 123
jsonpath "$.status" toInt == 0
jsonpath "$.unknown" toInt == 1
jsonpath "$.list" nth 5 == 3
jsonpath "$.id" toDate "%a, %d %b %Y %H:%M:%S GMT" == "unused"
jsonpath "$.id" daysAfterNow == 1
jsonpath "$.id" daysBeforeNow == 1
jsonpath "$.id" decode "utf-8" == "help"
jsonpath "$.id" format "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
jsonpath "$.number" htmlEscape == "a &gt; b"
jsonpath "$.number" htmlUnescape == "a > b"
jsonpath "$.number" jsonpath "$.name" == "help"
jsonpath "$.id" jsonpath "$.name" == "help"
jsonpath "$.id" nth 0 == 123
jsonpath "$.number" regex /Hello (.*)!/ == "Bob"
jsonpath "$.number" replace ", " "|" == "192.168.2.1|10.0.0.20|10.0.0.10"
jsonpath "$.number" split ", " count == 3
jsonpath "$.number" toDate "%Y-%m-%dT%H:%M:%S%.fZ" format "%A" == "Monday"
jsonpath "$.number" urlDecode == "https://mozilla.org/?x=шеллы"
jsonpath "$.number" urlEncode == "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
jsonpath "$.number" xpath "string(//body)" == "你好世界"
