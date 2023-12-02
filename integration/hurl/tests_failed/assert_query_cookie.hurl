GET http://localhost:8000/error-assert-query-cookie
HTTP 200

[Asserts]

cookie "cookie1[Secure]" not exists
cookie "cookie1[Secure]" == false      # This is not valid, Secure attribute exists or not but does have a value
cookie "cookie1[Secure]" not == true

cookie "cookie2[Secure]" exists
cookie "cookie2[Secure]" == true       # This is not valid, Secure attribute exists or not but does have a value
cookie "cookie2[Secure]" not == true   # This is not valid, Secure attribute exists or not but does have a value



