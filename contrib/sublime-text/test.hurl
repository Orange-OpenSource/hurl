# This is a comment
GET https://example.org/api/items?q={{search}}
User-Agent: hurl/1.0
Accept: application/json

HTTP 200
[Captures]
item_id: jsonpath "$.items[0].id"
csrf: header "X-CSRF" split "," nth 0

# Create a new item, with options
POST https://example.org/api/items
Content-Type: application/json
[Options]
retry: 3
insecure: true
delay: 500ms
{
  "name": "widget",
  "price": 9.99,
  "active": true,
  "tags": ["a", "b"],
  "owner": {{user_id}},
  "meta": null
}

HTTP 201
[Asserts]
status == 201
jsonpath "$.name" startsWith "wid"
jsonpath "$.price" > 5
jsonpath "$.count" isNumber
header "Location" matches "/items/[0-9]+"
jsonpath "$.tags" count == 2
variable "token" exists
body contains "widget"
redirects count == 0
duration < 1000

# Form parameters
POST https://example.org/login
[FormParams]
username: alice
password: {{secret}}

# Multipart upload
POST https://example.org/upload
[MultipartFormData]
file: file,data.bin; application/octet-stream

# Basic auth
GET https://example.org/private
[BasicAuth]
alice: {{password}}

# Oneline and multiline bodies
PUT https://example.org/raw
`inline oneline body {{x}}`

POST https://example.org/graphql
```graphql
query { user(id: {{id}}) { name } }
```

# XML body
POST https://example.org/soap
```xml
<request><id>{{id}}</id></request>
```

# Inline encoded body
POST https://example.org/bin
hex,deadbeef;
