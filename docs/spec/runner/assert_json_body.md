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




## Example Data

We will use this expected JSON below:


    20    {
    21      "first_name": "John",
    22      "last_name": "Smith",
    23      "is_alive": true,
    24      "age": 22,
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


## Possible Cases


### case 1 - int value mismatch
 
Expected value
   
   24    "age": 22


Actual Value

        "age": 20


Explicit jsonpath assert error

    24 |   "age": 22,
       |          ^^ value mismatch at $.age
       |   actual:   int <20>
       |   expected: int <22>


Explicit jsonpath assert error

     5 | jsonpath "$.age" == 22
       |   actual:   int <20>
       |   expected: int <22>


### case 2 - missing expected key


    23 |   "is_alive": true,
       |    ^^^^^^^^  Missing expected key $.is_alive 
       

Explicit jsonpath assert error

     5 | jsonpath "$.is_alive" == true
       |   actual: none
       | expected: boolean <true>


### case 3 - unexpected actual key

    20 |  {
       |  ...
    47 |  }
       |  ^ Unexpected actual key <country> at $.country


The line number matches the line for which it could be added in the source Hurl file.

Explicit jsonpath assert error

      5 | jsonpath "$.country" not exists
        |   actual:   string <spain>
        |   expected: not something
        |


### case 4 - mismatch value in array of strings

Expected array

    41      "children": [
    42        "Catherine",
    43        "Thomas",
    44        "Trevor"
    45      ]

Actual array
           "children": [
              "Thomas",
              "Trevor"
           ]

Explicit jsonpath assert error

    42 |        "Catherine",  
       |        ^^^^^^^^^^^ value mismatch at $.children[0]
       |  actual:   string <Thomas>
       |  expected: string <Catherine>



Explicit jsonpath assert error

      5 | jsonpath "$.children[0]" == "Catherine"
        |   actual:   string <Thomas>
        |   expected: string <Catherine>
        |




### case 5 - mismatch value in array of objects


Expected array

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

Actual array

            "phone_numbers": [
              {
                "type": "home",
                "number": : "210 555-1234"      
              },
              {
                "type": "office",
                "number": "646 555-4567"
              }
            ],

Explicit jsonpath assert error


    34 |        "number": : "212 555-1234"   
       |                    ^^^^^^^^^^^^^^ value mismatch at $.phone_numbers[0].number
       |  actual:   string <210 555-1234>
       |  expected: string <212 555-1234>


Explicit jsonpath assert error

      5 | jsonpath "$.phone_numbers[0].number" == "212 555-1234"
        |   actual:   string <210 555-1234>
        |   expected: string <212 555-1234>
        |



### case 6 - missing expected array element

Expected array

    41      "children": [
    42        "Catherine",
    43        "Thomas",
    44        "Trevor"
    45      ]

Actual array
           "children": [
              "Catherine",
              "Thomas",
           ]

Assert JSON Body Error

    44  |    "Trevor" 
        |    ^^^^^^^^ Missing expected array element at $.children[2] 
        |  actual: nothing
        |  expected string <Trevor>


Explicit jsonpath assert error

      5 | jsonpath "$.children[2]" == "Trevor"
        |   actual:   nothing
        |   expected: string <Trevor>
        |



### case 7 - unexpected array element

Expected array

    41      "children": [
    42        "Catherine",
    43        "Thomas",
    44        "Trevor"
    45      ]

Actual array
           "children": [
              "Catherine",
              "Thomas",
              "Trevor",
              "Bob"
           ]


Assert JSON Body Error

    45 |   ]
       |   ^ unexpected actual array element at $.children[3]
       |  actual:  string <Bob>   
       |  expected: nothing


Explicit jsonpath assert error

      5 | jsonpath "$.children[3]" not exist
        |   actual:   string <Bob>
        |   expected: nothing
        |


### case 8 - type mismatch 

 
Expected value
   
   24    "age": 22


Actual Value

        "age": "22"


Explicit jsonpath assert error

    24 |   "age": 22,
       |          ^^ value mismatch at $.age
       |   actual:   string <22>
       |   expected: int <22>


Explicit jsonpath assert errors

     5 | jsonpath "$.age" isNumber
       |   actual:   string <22>
       |   expected: number

     6 | jsonpath "$.age" == 22
       |   actual:   string <22>
       |   expected: int <22>



## Additional

Note that the generated errors do not fully enable to reconstruct the actual JSON. That's the reason why we are not going to call it a JSON Diff.

We initially wanted to produce such a diff, similar to [jd](https://github.com/josephburnett/jd) Format.

    jd object1.json object2.json       
    @ ["age"]
    - 20
    + 22

We found that a modified field value was quite readable, but array item additional and deletion was to hard to understand in the Hurl output format.

  
