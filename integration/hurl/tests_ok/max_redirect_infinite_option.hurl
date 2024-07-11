GET http://localhost:8000/redirect-infinite/60
[Options]
max-redirs: -1
location: true
HTTP 200


GET http://localhost:8000/redirect-infinite/60
[Options]
max-redirs: -1
location: false
HTTP 302


# Test max-redirs option is OK with template
GET http://localhost:8000/redirect-infinite/60
[Options]
variable: infinite=-1
max-redirs: {{infinite}}
location: true
HTTP 200

