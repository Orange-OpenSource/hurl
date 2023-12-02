GET http://localhost:8000/~user
HTTP 200
`user`

GET http://localhost:8000/%7Euser
HTTP 200
`user`

# TODO: add single quote (needs to be escaped in curl command-line)
GET http://localhost:8000/!$&()*+,;=:@[]
HTTP 200
`weird`

