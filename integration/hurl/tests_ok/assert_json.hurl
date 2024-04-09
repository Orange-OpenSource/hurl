GET http://localhost:8000/assert-json
[Options]
variable: one=1
variable: two=2
variable: three=3
variable: four=4
variable: five=5
variable: foo=abc
insecure: false
HTTP 200
[Captures]
count: jsonpath "$.count"
[Asserts]
jsonpath "$.count" == 5
jsonpath "$.count" == 5.0
jsonpath "$.count" == {{five}}
jsonpath "$.count" != 4
jsonpath "$.count" != {{four}}
jsonpath "$.count" not == 4
jsonpath "$.count" > 1
jsonpath "$.count" > {{one}}
jsonpath "$.count" > 1.0
jsonpath "$.success" == false
jsonpath "$.success" != null
jsonpath "$.success" exists
jsonpath "$.success" isBoolean
jsonpath "$.errors" count == 2
jsonpath "$.errors" isCollection
jsonpath "$.failures" count == 1
jsonpath "$.failures" isCollection
jsonpath "$.warnings" count == 0
jsonpath "$.warnings" isEmpty
jsonpath "$.toto" not exists
jsonpath "$.failures" exists
jsonpath "$.warnings" exists
jsonpath "$.errors[0]" exists
jsonpath "$.errors[0]" isCollection
jsonpath "$.errors[0].id" == "error1"
jsonpath "$.errors[0].id" isString
jsonpath "$.errors[0]['id']" == "error1"
jsonpath "$.errors[*].id" includes "error1"
jsonpath "$.errors[0,1].id" includes "error1"
jsonpath "$.errors[:2].id" count == 2
jsonpath "$.errors[0:].id" count == 2
jsonpath "$.errors[?(@.id)].id" nth 0 == "error1"
jsonpath "$.errors[?(@.id=='error1')].id" nth 0 == "error1"
jsonpath "$.failures[*].id" includes "failure1"
jsonpath "$.duration" == 1.5
jsonpath "$.duration" <= 2.0
jsonpath "$.duration" < 2
jsonpath "$.duration" < {{two}}
jsonpath "$.duration" isFloat
jsonpath "$.duration" isNumber
jsonpath "$.duration" not isInteger
jsonpath "$.nullable" == null
jsonpath "$.tags[0]" == "test"
jsonpath "$.tags[0]" > "te"
jsonpath "$.tags[0]" < "testabc"
jsonpath "$.tags[0]" <= "test"
jsonpath "$.tags[0]" > "{{foo}}"
jsonpath "$.profile-id" == "123abc"
jsonpath "$['profile-id']" == "123abc"
jsonpath "$.errors[0]" not isEmpty
jsonpath "$.empty" isEmpty
jsonpath "$.*" count == 11
jsonpath "$.errors..*" count == 4
jsonpath "$..id" count == 3
jsonpath "$.dates[0]" isIsoDate
jsonpath "$.dates[1]" isIsoDate
jsonpath "$.tags[0]" not isIsoDate
jsonpath "$.tags[0]" not isNumber


# FIXME do we accept count filter on object?
# jsonpath "$.empty" count == 0
{
  "count": 5,
  "success": false,
  "errors": [{"id":"error1"},{"id":"error2"}],
  "failures": [{"id":"failure1"}],
  "warnings": [],
  "duration": 1.5,
  "tags": ["test"],
  "nullable": null,
  "profile-id": "123abc",
  "dates": [
    "2022-10-31T09:00:00.594Z",
    "2024-03-20T11:23:56.773+02:00"
  ],
  "empty": {}
}


GET http://localhost:8000/assert-json/index
HTTP 200
[Captures]
index: body


GET http://localhost:8000/assert-json
HTTP 200
[Asserts]
jsonpath "$.errors[{{index}}].id" == "error2"
jsonpath "$.tags" includes "test"
jsonpath "$.tags" not includes "prod"
jsonpath "$.tags" not includes null


GET http://localhost:8000/assert-json/list
HTTP 200
[Asserts]
jsonpath "$" count == 2
jsonpath "$.[0].name" == "Bob"
jsonpath "$[0].name" == "Bob"
jsonpath "$.[?(@.id == 1)].name" nth 0 == "Bob"
jsonpath "$[?(@.id == 1)].name" nth 0 == "Bob"


GET http://localhost:8000/assert-json/filter
HTTP 200
[Asserts]
jsonpath "$.fruit[?(@.price.US==200)].name" nth 0 == "grape"
jsonpath "$.fruit.[?(@.US == 100)]" count == 0   # To be clarified with spec: the first fruit is return by jsonpath.com
{
    "fruit": [
        {
            "name": "apple",
            "price": {
                "US": 100,
                "UN": 110
            }
        },
        {
            "name": "grape",
            "price": {
                "US": 200,
                "UN": 150
            }
        }
    ]
}


GET http://localhost:8000/assert-json/filter-by-name
HTTP 200
[Asserts]

jsonpath "$..*[?(@.name)]" count == 5
jsonpath "$..*[?(@.name=='car')].id" nth 0 == 1
jsonpath "$..*[?(@.name=='wheel')].id" nth 0 == 4

jsonpath "$..items[?(@.name)]" count == 5
jsonpath "$..items[?(@.name=='car')].id" nth 0 == 1
jsonpath "$..items[?(@.name=='wheel')].id" nth 0 == 4

jsonpath "$.main.items[?(@.name)]" count == 3
jsonpath "$.main.items[?(@.name=='car')].id" nth 0 == 1
jsonpath "$.main.items[?(@.name=='wheel')].id" isEmpty

jsonpath "$.main..items[?(@.name)]" count == 4
jsonpath "$.main..items[?(@.name=='car')].id" nth 0 == 1
jsonpath "$.main..items[?(@.name=='wheel')].id" nth 0 == 4

{
  "main": {
    "items": [
      {
        "id": 1,
        "name": "car"
      },
      {
        "id": 2,
        "name": "bike",
        "items": [
          {
            "id": 4,
            "name": "wheel"
          }
        ]
      },
      {
        "id": 3,
        "name": "plane"
      }
    ]
  },
  "more": {
    "items": [
      {
        "id": 5,
        "name": "scooter"
      }
    ]
  }
}


GET http://localhost:8000/assert-json/big-number
HTTP 200
[Asserts]
jsonpath "$.big_integer" == 1000000000000000000000
{
  "big_integer": 1000000000000000000000
}
