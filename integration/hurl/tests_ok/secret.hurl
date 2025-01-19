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
# When run in normal mode (vs --test mode), log on standard error are logged immediately. Variables that are
# redacted may have leaked their value in the logged network call (from headers, libcurl etc...). So, if there
# are captured redacted variables in a entry, we do not authorize to set verbosity to prevent any chance of leaking.
# NOTE: in test mode, there is no risk of leaks even in verbose mode, as standard error logs are buffered and redacted
# before being printed.
[Options]
verbose: false
HTTP 200
[Captures]
token: header "X-Token" redact
[Asserts]
variable "token" matches /secret/


GET http://localhost:8000/another-secret
x-token: {{token}}
HTTP 200

