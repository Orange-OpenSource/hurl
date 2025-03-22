# JSON Diff

##  Motivation

Currently, JSON response are compared from a textual perspective and not a semantic one.
There are 2 main drawbacks.

First, 2 equivalent JSON can produce an error if they have different formatting spacing or different field ordering.

Example:

Hurl file

    test.hurl
    GET http://localhost:8000/json
    {
      "greeting": "Hello"
    }

If the response returns JSON on one line `{"greeting":"Hello"}`


    $ hurl test.hurl
    error: Assert body value
      --> /tmp/test.hurl:3:1
      |
      | GET http://localhost:8000/greeting
      | ...
    3 | {
      |   -{
      |   -  "greeting": "Hello"
      |   -}
      |   +{"greeting":"Hello"}
      | }


Second, when they are really different, the error message will show the diff, but will also be polluted due to different field ordering.

Example:

Hurl file

    test.hurl
    GET http://localhost:8000/bob
    {
      "name": "Bob",
      "age": 22
    }


    $ hurl test.hurl
    error: Assert body value
      --> /tmp/test2.hurl:4:1
      |
      | GET http://localhost:8000/bob
      | ...
    4 |   "name": "Bob",
      |   -  "name": "Bob",
      |   -  "age": 22
      |   -}
      |   +  "age": 20,
      |   +  "name": "Bob"
      |   +}
      |   +


## Using the native jd diff output

The native [jd](https://github.com/josephburnett/jd) 
format use a "context" to specify the node for which the diff occurs.

### Scalar example

    string1.json 
    "Hello"

    string2.json 
    "Hi"

    $ jd string1.json string2.json 
    @ []
    - "Hello"
    + "Hi"

The context is empty (directly under the root)

### Object example

    object1.json
    {
      "name": "Bob",
      "age": 22
    }

    object2.json
    {
      "age": 22,
      "name": "Bob"
    }

    jd object1.json object2.json       
    @ ["age"]
    - 20
    + 22

The context is the `age` field in the root object.

### List example

    list1.json 
    [
      {
        "name": "Bob",
        "age": 20
      },
      {
        "name": "Bruce",
        "age": 17
      }
    ]

    list2.json 
    [
      {
        "name": "Bob",
        "age": 22
      },
      {
        "name": "Bruce",
        "age": 17
      }
    ]

    $ jd list1.json list2.json 
    @ [0,"age"]
    - 20
    + 22

The context is the first element of the root array, then the `age` field.
    

### Deeper diff example

20    {
21      "first_name": "John",
22      "last_name": "Smith",
23      "is_alive": true,
24      "age": 27,
25      "address": {
26        "street_address": "21 2nd Street",
27        "city": "New York",
28        "state": "NY",
29        "postal_code": "10021-3100"
30      },
31      "phone_numbers": [
32        {
33          "type": "home",
34          "number": "212 555-1234"   
35        },
36        {
37          "type": "office",
38          "number": "646 555-4567"
39        }
40      ],
41      "children": [
42        "Catherine",
43        "Thomas",
44        "Trevor"
45      ],
46      "spouse": null
47    }


The diff produces the following output:

    @ ["phone_numbers",0,"number"]
    - "212 555-1234"
    + "212 555-1233"

The context consists of 3 levels:
   - field `phone_numbers` in the root object
   - first element of the array
   - field `number` in the object


### Combining several diffs

Each diff has its own context. For example, if both numbers have been changed:

    @ ["phone_numbers",0,"number"]
    - "212 555-1234"
    + "212 555-1233"
    @ ["phone_numbers",1,"number"]
    - "646 555-4567"
    + "646 555-4568"


## Using jsonpath in the context

   The context could also be defined as a jsonpath expression.

   For example  `@ [$.phone_numbers[0].number]` could be used instead of `@ ["phone_numbers",0,"number"]`.


## Using Jsonpath-like error


Let' take the following Hurl file (expected JSON)

    1  GET http://localhost:8000/bob
    2  {
    3    "name": "Bob",
    4    "age": 22,
    5    "hobbies": ["biking", "swimming"]
    6  }
    7


### Case Modified field

Response from server (actual)

    {
      "name": "Bob",
      "age": 20,
      "hobbies": ["biking", "swimming"]
    }

Error

      --> /home/fab/tmp/test_jsondiff/modify.hurl:4:0
      |
      | GET http://localhost:8000/modify
      | ...
    4 | jsonpath "$.age"
      |   actual:   int <20>
      |   expected: int <22>
      |

### Case Deleted field

Response from server (actual)

    {
      "name": "Bob",
      "hobbies": ["biking", "swimming"]
    }

Error

    error: Assert failure
      --> /home/fab/tmp/test_jsondiff/delete.hurl:4:0
      |
      | GET http://localhost:8000/delete
      | ...
    4 | jsonpath "$.age
      |   actual:   
      |   expected: integer <22>
      |

A deleted field is characterised by the absence of an actual value.


### Case New field

Response from server (actual)

    {
      "name": "Bob",
      "age": 22,
      "hobbies": ["biking", "swimming"]
      "country": "Spain"
    }

Error

    error: Assert failure
      --> /home/fab/tmp/test_jsondiff/add.hurl:2:0
      |
      | GET http://localhost:8000/add
      | ...
    2 | jsonpath "$.country
      |   actual:   string <Spain>
      |   expected: 
      |

An additional field can be characterised by an absence of expected value.
The error source line will match the enclosing context (parent)


### Case new array element

Response from server (actual)

    {
      "name": "Bob",
      "age": 20,
      "hobbies": ["biking", "guitar", "swimming"]
    }


Error

    error: Assert failure
      --> /home/fab/tmp/test_jsondiff/add_array_item.hurl:5:0
      |
      | GET http://localhost:8000/add_array_item
      | ...
    5 | jsonpath "$.hobbies[1]"
      |   actual:   string <guitar>
      |   expected: 
      |

An additional value is characterized by an absence of expected value.
This can be confusing though, as the jsonpath expression "$.hobbies[1]" is valid from the JSON source, and point to the next element.

In this case, the standard diff representation is less ambigous.

    error: Assert failure
      --> /home/fab/tmp/test_jsondiff/add_array_item.hurl:5:0
      |
      | GET http://localhost:8000/add_array_item
      | ...
    5 | jsonpath "$.hobbies[1]"
      |   + "guitar" 
      |


# Using exactly the jsonpath assert semantic

The JSON body will be treated as it was defined with jsonpath asserts.
for example, for the list of phone numbers:

    [Asserts]
    jsonpath "$.phone_numbers[0].type" == "home"
    jsonpath "$.phone_numbers[0].number" == "212 555-1234"
    jsonpath "$.phone_numbers[1].type" == "office"
    jsonpath "$.phone_numbers[1].number" == "646 555-4567"
    jsonpath "$.phone_numbers[2].type" not exists
    jsonpath "$.phone_numbers[2].number" not exists



## First phone number modified

    error: Assert JSON Body
      --> test_jsondiff.hurl:14:21
      |
      | GET http://localhost:8000/modify_json
      | ...
   34 | jsonpath "$.phone_numbers[0].number" == "212 555-1234"
      |   actual:   string <210 555-1234>
      |   expected: string <212 555-1234>
      |




## First phone number deleted

    error: Assert JSON Body
      --> test_jsondiff.hurl:25:0
      |
      | GET http://localhost:8000/delete_json
      | ...
   33 | jsonpath "$.phone_numbers[0].type" == "home"
      |   actual:   string <office>
      |   expected: string <home>
      | ...
   34 | jsonpath "$.phone_numbers[0].number" == "212 555-1234"
      |   actual:   string <646 555-4567>
      |   expected: string <212 555-1234>
      |
   37 | jsonpath "$.phone_numbers[1].type" == "office"
      |   actual:   none
      |   expected: string <office>
      |
   38 | jsonpath "$.phone_numbers[1].number" == "646 555-4567"
      |   actual:   none
      |   expected: string <646 555-4567>
      |




## Phone number added as the second element


    error: Assert JSON Body
      --> test_jsondiff.hurl:37:0
      |
      | GET http://localhost:8000/add_json
      | ...
   37 | jsonpath "$.phone_numbers[1].type" == "office"
      |   actual:   string <mobile>
      |   expected: string <office>
      | ...
   38 | jsonpath "$.phone_numbers[1].number" == "646 555-4567"
      |   actual:   string <111 222-3333>
      |   expected: string <646 555-4567>
      |
   39 | jsonpath "$.phone_numbers[2].type" not exists
      |   actual:   string <office>
      |   expected: not something
      |
   39 | jsonpath "$.phone_numbers[2].number" not exists
      |   actual:   string <646 555-4567>
      |   expected: not something
      |

## Using assert on array length when it is different

Only check array elements if they have the same length.

    error: Assert failure
      --> test_jsondiff.hurl:41:0
      |
      | GET http://localhost:8000/add_json
      | ...
   31 | jsonpath "$.phone_numbers" count == 2
      |   actual:   integer <3>
      |   expected: integer <2>
  