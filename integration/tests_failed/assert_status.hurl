# We add various explicit asserts on the body, and
# implicit asserts on the response headers.
# As the status code is not correct, those asserts
# should not be tested (the status code is a "stronger"
# assert than the others).

GET http://localhost:8000/not_found

HTTP 200
x-baz: xxx
[Asserts]
duration < 0
jsonpath "$.foo" startsWith "something"
jsonpath "$.bar" not exists
header "x-bar" exists
