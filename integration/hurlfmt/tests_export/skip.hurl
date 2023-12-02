GET http://localhost:8000/skip/init
HTTP 200
`0`

GET http://localhost:8000/skip/increment
[Options]
skip: true
HTTP 200
`whatever`

GET http://localhost:8000/skip/get
HTTP 200
`0`

GET http://localhost:8000/skip/increment
[Options]
skip: false
HTTP 200
`1`

GET http://localhost:8000/skip/get
HTTP 200
`1`
