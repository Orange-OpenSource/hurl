# Testing trailing whitespace
# Response '<p>Hello</p>\n\n'

# Testing with a multiline string
# check exactly the whitespace
# It should produce an assert error
GET http://localhost:8000/error-assert-newline
HTTP 200
```
<p>Hello</p>
```

# TODO
# Testing with XML
# Ignore trailing whitespace (semantic comparison)
# It should not produce an error
# GET http://localhost:8000/error-assert-newline
# HTTP 200
# <p>Hello</p>
