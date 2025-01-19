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
jsonpath "$.value" matches /\d{8}/


GET http://localhost:8000/get-token
HTTP 200
[Captures]
token: header "X-Token" redact
[Asserts]
variable "token" matches /secret/


GET http://localhost:8000/another-secret
x-token: {{token}}
HTTP 200

