GET http://localhost:8000/default-headers
HTTP/1.0 200

GET http://localhost:8000/default-headers
User-Agent: hurl/1.0
Host: localhost:8000  # comment
HTTP/1.0 200


GET http://localhost:8000/default-headers
User-Agent: hurl/1.0
Host: localhost:8000  # comment
HTTP/1.0 200

GET http://localhost:8000/custom-headers
Fruit: Raspberry
Fruit: Apple
Fruit: Banana
Fruit: Grape
Color: Green
HTTP/1.0 200

GET http://localhost:8000/custom-headers-utf8
Beverage: café  # send the utf8 string - expected to be decoded as ascii in the server side
HTTP/1.0 200

GET http://localhost:8000/custom-headers-value
Id: \#123       # send a hash in the value
HTTP/1.0 200

GET http://localhost:8000/custom-headers-quote
Header1: '
HTTP/1.0 200

GET http://localhost:8000/response-headers
HTTP/1.0 200
Beverage: cafe  # TBC send utf8
