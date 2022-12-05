GET http://localhost:8000/filter

HTTP 200
[Captures]
url: jsonpath "$.url"
[Asserts]
jsonpath "$.list" count == 3
jsonpath "$.message" regex /Hello (.*)!/ == "Bob"
jsonpath "$.url" == "https://mozilla.org/?x=шеллы"
jsonpath "$.url" urlEncode == "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
jsonpath "$.encoded_url" urlDecode == "https://mozilla.org/?x=шеллы"
variable "url" urlEncode urlDecode == "{{url}}"
jsonpath "$.html" == "a > b && a < c"
jsonpath "$.html" htmlEncode == "a &gt; b &amp;&amp; a &lt; c"
jsonpath "$.encoded_html" htmlDecode == "a > b && a < c"
variable "html" urlEncode urlDecode == "{{html}}"
{
  "list": [1,2,3],
  "message": "Hello Bob!",
  "url": "https://mozilla.org/?x=шеллы",
  "encoded_url": "https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B",
  "html": "a > b && a < c",
  "encoded_html": "a &gt; b &amp;&amp; a &lt; c"
}
