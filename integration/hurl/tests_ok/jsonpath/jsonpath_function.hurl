# Check JSONPPath Functions

# Functions do not raise runtime errors
# For example, an invalid regex used in the match function will not raise an error but simply return no match

GET http://localhost:8000/jsonpath/function
HTTP 200
[Asserts]

# length function
jsonpath "$.items[?length(@.name) > 3]" count == 2
jsonpath "$.items[?length(@.tags) >= 2]" count == 2
jsonpath "$.items[?length(@.tags) >= $.two]" count == 2
jsonpath "$.items[?length(@) >= 2]" count == 3
jsonpath "$.items[?length(@.heavy) >= 2]" not exists

# count function
jsonpath "$.items[?count(@.tags) == 1]" count == 3

# match function
jsonpath "$.items[?match(@.name, '^ca.*')].name" == "car"
jsonpath "$.items[?match(@.name, $.pattern)].name" == "car"
jsonpath "$.items[?match(@.unknow, '^ca.*')]" not exists           # field not found
jsonpath "$.items[?match(@.name, $.unknown)]" not exists           # pattern not found
jsonpath "$.items[?match(@.name, $.two)].name" not exists          # regex should be defined as a string
jsonpath "$.items[?match(@.name, '[]')].name" not exists           # invalid regex

# search function
jsonpath "$.items[?search(@.name, 'ca')].name" == "car"
jsonpath "$.items[?search(@.name, $.string)].name" == "car"
jsonpath "$.items[?search(@.unknown, 'ca')]" not exists            # field not found
jsonpath "$.items[?search(@.name, $.unknown)]" not exists          # pattern not found
jsonpath "$.items[?search(@.name, $.two)]" not exists              # regex should be defined as a string
jsonpath "$.items[?search(@.name, '[]')]" not exists               # invalid regex

# value function
jsonpath "$.items[?value(@.heavy) == true]" count == 2
jsonpath "$.items[?value(@.unknown) == true]" not exists

# nested function call
jsonpath "$.items[?length(value(@.name)) > 4].name" == "plane"

{
  "two": 2,
  "pattern": "ca.*",
  "string": "ca",
  "items": [
    {
      "id": 1,
      "name": "car",
      "tags": [
        "transport"
      ],
      "heavy": true
    },
    {
      "id": 2,
      "name": "bike",
      "tags": [
        "transport",
        "sport"
      ],
      "heavy": false
    },
    {
      "id": 3,
      "name": "plane",
      "tags": [
        "transport",
        "fast",
        "expensive"
      ],
      "heavy": true
    }
  ]
}
