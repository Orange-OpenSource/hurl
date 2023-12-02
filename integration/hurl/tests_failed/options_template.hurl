# Invalid type at runtime

GET http://localhost:8000/unused
[Options]
variable: redirect=10
location: {{redirect}}
