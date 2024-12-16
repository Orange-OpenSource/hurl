GET http://localhost:8000/secret
x-secret: barbar
x-secret: BarBar
{
  "query": "{{a}}"
}
HTTP 200
[Asserts]
jsonpath "$.value" == "baz"
