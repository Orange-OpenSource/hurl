GET http://localhost:8000/redirect/7
[Options]
location: true
max-redirs: 5
HTTP 200


# max-redirs should work in template also
GET http://localhost:8000/redirect/7
[Options]
variable: max=5
location: true
max-redirs: {{max}}
HTTP 200
