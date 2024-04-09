GET http://localhost:8000/assert-xpath
HTTP 200
[Asserts]
xpath "normalize-space(//data)" == "café"
xpath "normalize-space(//data)" == "caf\u{00e9}"
xpath "normalize-space(//data)" > "CAFÉ"
xpath "//toto" not exists

<data>café</data>


# Test XPath assert with XML namespace.
GET http://localhost:8000/assert-xpath-simple-namespaces
HTTP 200
[Asserts]

xpath "string(//bk:book/bk:title)" == "Cheaper by the Dozen"
xpath "string(//*[name()='bk:book']/*[name()='bk:title'])" == "Cheaper by the Dozen"
xpath "string(//*[local-name()='book']/*[local-name()='title'])" == "Cheaper by the Dozen"

xpath "string(//bk:book/isbn:number)" == "1568491379"
xpath "string(//*[name()='bk:book']/*[name()='isbn:number'])" == "1568491379"
xpath "string(//*[local-name()='book']/*[local-name()='number'])" == "1568491379"


# Test XPath assert with default XML namespace.
# _ can be used to target a default namespace.
GET http://localhost:8000/assert-xpath-svg
HTTP 200
[Asserts]
xpath "//_:svg/_:g/_:circle" count == 3
xpath "//*[local-name()='svg']/*[local-name()='g']/*[local-name()='circle']" count == 3
xpath "//*[name()='svg']/*[name()='g']/*[name()='circle']" count == 3


# Test XPath assert with default and prefixed XML namespace.
# _ can be used to target a default namespace.
GET http://localhost:8000/assert-xpath-namespaces
HTTP 200
[Asserts]
xpath "string(//_:book/_:title)" == "Cheaper by the Dozen"
xpath "string(//_:book/_:title)" > "Cheaper"
xpath "string(//_:book/isbn:number)" == "1568491379"
xpath "//*[name()='book']/*[name()='notes']" count == 1
xpath "//*[local-name()='book']/*[local-name()='notes']" count == 1
xpath "//_:book/_:notes/*[local-name()='p']" count == 1

