GET http://localhost:8000/default-headers
HTTP 200

GET http://localhost:8000/default-headers
User-Agent: hurl/1.0
Host: localhost:8000  # Comment
HTTP 200

GET http://localhost:8000/default-headers
User-Agent: hurl/1.0
Host: localhost:8000  # Comment
HTTP 200

GET http://localhost:8000/custom-headers
Fruit: Raspberry
Fruit: Apple
Fruit: Banana
Fruit: Grape
Color: Green
HTTP 200

GET http://localhost:8000/custom-headers
# Headers name/value can use templates
{{key}}: Raspberry
{{key}}: Apple
{{key}}: Banana
{{key}}: {{foo}}
Color: {{color}}
[Options]
variable: key=Fruit
variable: color=Green
variable: foo=Grape
HTTP 200

GET http://localhost:8000/custom-headers-utf8
Beverage: café  # Send the utf8 string - expected to be decoded as ascii in the server side
HTTP 200

GET http://localhost:8000/custom-headers-value
Id: \#123       # Send a hash in the value
HTTP 200

GET http://localhost:8000/custom-headers-quote
Header1: '
HTTP 200

GET http://localhost:8000/response-headers
HTTP 200
Beverage: cafe  # TBC send utf8
