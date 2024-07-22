GET http://localhost:8000/filter
HTTP 200
[Captures]
url: jsonpath "$.url"
text: jsonpath "$.text"
[Asserts]
jsonpath "$.list" count == 3
jsonpath "$.list" nth 1 == 2
jsonpath "$.message" regex /Hello (.*)!/ == "Bob"
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
jsonpath "$.ips" split ", " count == 3
jsonpath "$.ips" replace ", " "|" == "192.168.2.1|10.0.0.20|10.0.0.10"
jsonpath "$.json" jsonpath "$.message" == "Hello"
jsonpath "$.pi" toFloat == 3.141592653589793
jsonpath "$.pi" toFloat > 3
jsonpath "$.pi" toFloat < 4
jsonpath "$.ten" toInt == 10
{
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
