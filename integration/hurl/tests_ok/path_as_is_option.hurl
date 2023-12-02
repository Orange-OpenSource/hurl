# Dot segments are removed by default
# the sequence "any/../" evaluate to nothing
# therefore the url sent is http://localhost:8000/hello
GET http://localhost:8000/any/../hello
HTTP 200
`Hello World!`

# Setting the option path-as-is
# the path is sent as it is (provided in the url)
GET http://localhost:8000/path-as-is/../resource
[Options]
path-as-is: true
HTTP 200


