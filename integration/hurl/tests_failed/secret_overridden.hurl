# We check that variable can't override secret because a secret becoming
# a public variable will leak it previous secret value.
GET http://localhost:8000/hello
x-header: a_secret_value
[Options]
variable: name=a_public_value
HTTP 200
