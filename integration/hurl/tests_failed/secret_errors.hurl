# A public variable can override a secret variable, previous secrets are kept
# for the lifetime of the Hurl file.
GET http://localhost:8000/hello
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
