GET http://localhost:8000/secret?q={{c}}
x-secret: secret2
x-secret: {{b}}
x-secret: Secret2
[Cookies]
X-SECRET: {{b}}
{
  "query": "{{a}}"
}
HTTP 200
[Captures]
value: jsonpath "$.value"
[Asserts]
jsonpath "$.value" == "12345678"
