GET http://localhost:8000/filter
HTTP 200
[Captures]
url: jsonpath "$.url"
text: jsonpath "$.text"
[Asserts]
jsonpath "$.file" base64Decode == hex,d188d0b5d0bbd0bbd18b;
jsonpath "$.file" base64Decode base64Encode == "0YjQtdC70LvRiw=="
jsonpath "$.file" base64Decode base64UrlSafeEncode == "0YjQtdC70LvRiw"
jsonpath "$.text_encoded" == "PDw_Pz8-Pg"
jsonpath "$.text_encoded" base64UrlSafeDecode == hex,3c3c3f3f3f3e3e;
jsonpath "$.text_encoded" base64UrlSafeDecode base64Encode == "PDw/Pz8+Pg=="
jsonpath "$.text_encoded" base64UrlSafeDecode base64UrlSafeEncode == "PDw_Pz8-Pg"
jsonpath "$.list" count == 3
jsonpath "$.list" first == 1
jsonpath "$.list" last == 3
jsonpath "$.list" nth 1 == 2
jsonpath "$.list" nth -1 == 3
jsonpath "$.message" regex /Hello (.*)!/ == "Bob"
jsonpath "$.message" regex /Hi (.*)!/ not exists
jsonpath "$.url" == "https://mozilla.org/?x=шеллы"
jsonpath "$.url" urlEncode == "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
jsonpath "$.encoded_url" urlDecode == "https://mozilla.org/?x=шеллы"
variable "url" urlEncode urlDecode == "{{url}}"
jsonpath "$.text" == "a > b && a < c"
jsonpath "$.text" htmlEscape == "a &gt; b &amp;&amp; a &lt; c"
jsonpath "$.escaped_html[0]" htmlUnescape == "a > b && a < c"
jsonpath "$.escaped_html[1]" htmlUnescape == "Foo © bar 𝌆 baz ☃ qux"
jsonpath "$.escaped_html[2]" htmlUnescape == "A foo"
variable "text" htmlEscape htmlUnescape == "{{text}}"
jsonpath "$.id" toInt == 123
jsonpath "$.id" toFloat == 123
jsonpath "$.score" toInt == 1
jsonpath "$.score" toFloat == 1.6
jsonpath "$.score" toString == "1.6"
jsonpath "$.file" base64Decode toHex == "d188d0b5d0bbd0bbd18b"
jsonpath "$.ips" split ", " count == 3
jsonpath "$.ips" split ", " first == "192.168.2.1"
jsonpath "$.ips" split ", " last == "10.0.0.10"
jsonpath "$.ips" replace ", " "|" == "192.168.2.1|10.0.0.20|10.0.0.10"
jsonpath "$.id" replaceRegex /\d/ "x" == "xxx"
jsonpath "$.message" replaceRegex "B[a-z]b" "dude" == "Hello dude!"
jsonpath "$.message" replaceRegex /(?i)b[a-z]b/ "dude" == "Hello dude!"
# First args of replace filter _is not_ a regex
jsonpath "$.id" replace "\\d" "x" == "123"
jsonpath "$.json" jsonpath "$.message" == "Hello"
jsonpath "$.pi" toFloat == 3.141592653589793
jsonpath "$.pi" toFloat > 3
jsonpath "$.pi" toFloat < 4
jsonpath "$.ten" toInt == 10
jsonpath "$.url" urlQueryParam "x" == "шеллы"
jsonpath "$.url" urlQueryParam "y" not exists
jsonpath "$.encoded_url" urlQueryParam "x" == "шеллы"
{
  "file": "0YjQtdC70LvRiw==",
  "text_encoded": "PDw_Pz8-Pg",
  "list": [1,2,3],
  "message": "Hello Bob!",
  "url": "https://mozilla.org/?x=шеллы",
  "encoded_url": "https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B",
  "text": "a > b && a < c",
  "escaped_html": [
    "a &gt; b &amp;&amp; a &lt; c",
    "Foo &#xA9; bar &#x1D306; baz &#x2603; qux",
    "&#65 foo"
  ],
  "id": "123",
  "score": 1.6,
  "ips": "192.168.2.1, 10.0.0.20, 10.0.0.10",
  "json": "{\"message\": \"Hello\"}",
  "pi": "3.141592653589793",
  "ten": 10
}


# Tests some filters with templates
GET http://localhost:8000/filter
[Options]
variable: single_digit=\\d
variable: index_first=1
variable: index_last=-1
HTTP 200
[Asserts]
jsonpath "$.id" replaceRegex "{{single_digit}}" "x" == "xxx"
jsonpath "$.list" nth {{index_first}} == 2
jsonpath "$.list" nth {{index_last}} == 3
{
  "file": "0YjQtdC70LvRiw==",
  "text_encoded": "PDw_Pz8-Pg",
  "list": [1,2,3],
  "message": "Hello Bob!",
  "url": "https://mozilla.org/?x=шеллы",
  "encoded_url": "https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B",
  "text": "a > b && a < c",
  "escaped_html": [
    "a &gt; b &amp;&amp; a &lt; c",
    "Foo &#xA9; bar &#x1D306; baz &#x2603; qux",
    "&#65 foo"
  ],
  "id": "123",
  "score": 1.6,
  "ips": "192.168.2.1, 10.0.0.20, 10.0.0.10",
  "json": "{\"message\": \"Hello\"}",
  "pi": "3.141592653589793",
  "ten": 10
}
