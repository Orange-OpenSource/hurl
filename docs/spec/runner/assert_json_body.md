# JSON body Asserts

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



## Using Hurl jsonpath error semantic

"Standard" Hurl jsonpath errors can be generated when the actual JSON is differnt from the received one.

For example

      --> test.hurl:4:0
      |
      | GET http://localhost:8000/modify
      | ...
    4 | jsonpath "$.age"
      |   actual:   int <20>
      |   expected: int <22>
      |

For arrays, when they differs in size, we will only display the errors on the count of elements.

      error: Assert JSON Body
        --> test.hurl:41:0
        |
        | GET http://localhost:8000/add_json
        | ...
     31 | jsonpath "$.phone_numbers" count == 2
        |   actual:   integer <3>
        |   expected: integer <2>


Using our message `expected: not something` on a specific array element when a item has been added or deleted was not easy to understand.



## Example

We will use this expected JSON below:


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
    34          "number": : "212 555-1234"      
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

### case 1 - age modified
 

    24 | jsonpath "$.age"
       |   actual:   int <20>
       |   expected: int <22>


### case 2 - is_alive field deleted

    23 | jsonpath "$.is_alive"
       |   actual:   not something
       |   expected: true

### case 3 - new country field added

    47 | jsonpath "$.country"
       |   actual:   spain
       |   expected: not something

The line number matches the line for which it could be added in the source Hurl file.


### case 4 - first phone number modified
 
    34 | jsonpath "$.phone_numbers[0].number" == "212 555-1234"
       |   actual:   string <210 555-1234>
       |   expected: string <212 555-1234>


### case 5 - deleting a phone number

     31 | jsonpath "$.phone_numbers" count == 2
        |   actual:   integer <2>
        |   expected: integer <3>

The line number match the start of the array in the source Hurl file.


### case 6 - adding a phone number

        | 
     31 | jsonpath "$.phone_numbers" count == 2
        |   actual:   integer <3>
        |   expected: integer <2>



The line number matches the start of the array in the source Hurl file.


## Additional

Note that the generated errors do not fully enable to reconstruct the actual JSON. That's the reason why we are not going to call it a JSON Diff.

We initially wanted to produce such a diff, similar to [jd](https://github.com/josephburnett/jd) Format.

    jd object1.json object2.json       
    @ ["age"]
    - 20
    + 22

We found that a modified field value was quite readable, but array item additional and deletion was to hard to understand in the Hurl output format.

  
