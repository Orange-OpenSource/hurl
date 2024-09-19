# In various body requests (JSON, XML, form etc...) there is
# no need to provide a Content-Type header, Hurl will infer the right
# content type header depending of the request. That said, user has always
# control over this header and can explicitly override it.

POST http://localhost:8000/content-type-json
{
    "name": "Bob",
    "age": 30
}
HTTP 200


# JSON body using multilines strings:
POST http://localhost:8000/content-type-json
```json
{
    "name": "Bob",
    "age": 30
}
```
HTTP 200


# Without language hints:
POST http://localhost:8000/content-type-json
Content-Type: application/json
```
{
    "name": "Bob",
    "age": 30
}
```
HTTP 200


POST http://localhost:8000/content-type-json
```graphql
{
  project(name: "GraphQL") {
    tagline
  }
}
```
HTTP 200


POST http://localhost:8000/content-type-json
Content-Type: application/json
{
    "name": "Bob",
    "age": 30
}
HTTP 200


POST http://localhost:8000/content-type-vnd-json
Content-Type: application/vnd.api+json
{
    "name": "Bob",
    "age": 30
}
HTTP 200


# "content-type vs "Content-Type": explicit content-type header must override implicit
# content type even if name of the header is in lowercase
POST http://localhost:8000/content-type-vnd-json
content-type: application/vnd.api+json
{
    "name": "Bob",
    "age": 30
}
HTTP 200


POST http://localhost:8000/content-type-form
[FormParams]
field1: foo
field2: bar
field2: baz
HTTP 200


POST http://localhost:8000/content-type-form
[Form]
field1: foo
field2: bar
field2: baz
HTTP 200


POST http://localhost:8000/content-type-multipart
[MultipartFormData]
field1: foo
field2: bar
field2: baz
HTTP 200


POST http://localhost:8000/content-type-multipart
[Multipart]
field1: foo
field2: bar
field2: baz
HTTP 200


POST http://localhost:8000/content-type-xml
<note>
    <to>Tove</to>
    <from>Jani</from>
    <heading>Reminder</heading>
    <body>Don't forget me this weekend!</body>
</note>
HTTP 200


# XML body using multilines strings:
POST http://localhost:8000/content-type-xml
```xml
<note>
    <to>Tove</to>
    <from>Jani</from>
    <heading>Reminder</heading>
    <body>Don't forget me this weekend!</body>
</note>
```
HTTP 200


# XML body using multilines strings:
POST http://localhost:8000/content-type-xml
Content-Type: application/xml
```
<note>
    <to>Tove</to>
    <from>Jani</from>
    <heading>Reminder</heading>
    <body>Don't forget me this weekend!</body>
</note>
```
HTTP 200
