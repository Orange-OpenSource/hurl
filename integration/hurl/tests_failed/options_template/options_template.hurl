# Invalid options values. We computed these options at runtime using
# variables, to bypass the parser check that is run before executing
# a Hurl file.

GET http://localhost:8000/unused
[Options]
variable: location=10
location: {{location}}
HTTP 200


GET http://localhost:8000/unused
[Options]
variable: max_redirect=-123
max-redirs: {{max_redirect}}
HTTP 200


GET http://localhost:8000/unused
[Options]
variable: max_redirect=abc
max-redirs: {{max_redirect}}
HTTP 200


GET http://localhost:8000/unused
[Options]
variable: count=-2
repeat: {{count}}
HTTP 200


GET http://localhost:8000/unused
[Options]
variable: count=foo
repeat: {{count}}
HTTP 200


GET http://localhost:8000/unused
[Options]
variable: count=-2
retry: {{count}}
HTTP 200


GET http://localhost:8000/unused
[Options]
variable: count=foo
retry: {{count}}
HTTP 200
