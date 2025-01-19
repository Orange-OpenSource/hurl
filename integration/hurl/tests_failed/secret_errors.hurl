# We check that variable can't override secret because a secret becoming
# a public variable will leak it previous secret value.
GET http://localhost:8000/hello
x-header: a_secret_value
[Options]
variable: name=a_public_value
HTTP 200

# Secrets must be strings
GET http://localhost:8000/hello
HTTP 200
[Captures]
token1: header "Date" toDate "%a, %d %b %Y %H:%M:%S GMT%Z" redact


# Entries that created new secrets with `redact` can't be log in normal mode (vs --test mode)
GET http://localhost:8000/hello
[Options]
verbose: true
HTTP 200
[Captures]
token2: header "Date" redact
