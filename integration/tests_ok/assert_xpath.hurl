GET http://localhost:8000/assert-xpath

HTTP/1.0 200
[Asserts]
xpath "normalize-space(//data)" equals "café"
xpath "normalize-space(//data)" equals "caf\u{00e9}"
xpath "//toto" not exists

<data>café</data>
