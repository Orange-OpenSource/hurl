GET http://localhost:8000/repeat/hello?name=A
[Options]
repeat: 4
output: -
HTTP 200


GET http://localhost:8000/repeat/hello?name=B
[Options]
repeat: 0   # repeat 0 is equivalent to skip
output: -
HTTP 200


GET http://localhost:8000/repeat/hello?name=C
[Options]
repeat: 3
output: -
HTTP 200


GET http://localhost:8000/repeat/hello?name=D
[Options]
repeat: 1
output: -
HTTP 200


GET http://localhost:8000/repeat/hello?name=E
[Options]
repeat: 2
output: -
HTTP 200
