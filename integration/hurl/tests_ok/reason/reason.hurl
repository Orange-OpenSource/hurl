# This file tests an HTTP endpoint that returns a localized HTTP reason phrase.
# Some HTTP clients doesn't like it, curl does not care.
GET http://localhost:8000/reason-french
HTTP 200
