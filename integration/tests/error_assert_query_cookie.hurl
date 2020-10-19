GET http://localhost:8000/error-assert-query-cookie
HTTP/1.0 200

[Asserts]

cookie "cookie1[Secure]" not exists
cookie "cookie1[Secure]" equals false      # This is not valid, Secure attribute exists or not but does have a value
cookie "cookie1[Secure]" not equals true

cookie "cookie2[Secure]" exists
cookie "cookie2[Secure]" equals true       # This is not valid, Secure attribute exists or not but does have a value
cookie "cookie2[Secure]" not equals true   # This is not valid, Secure attribute exists or not but does have a value



