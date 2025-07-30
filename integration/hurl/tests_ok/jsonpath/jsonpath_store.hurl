# Example from https://goessner.net/articles/JsonPath/
GET http://localhost:8000/json/store
HTTP 200
[Asserts]
jsonpath "$.store.book[*].author" count == 4                               # the authors of all books in the store
jsonpath "$.store.book[*].author" nth 0 == "Nigel Rees"
jsonpath "$.store.book[*].author" nth 1 == "Evelyn Waugh"
jsonpath "$.store.book[*].author" nth 2 == "Herman Melville"
jsonpath "$.store.book[*].author" nth 3 == "J. R. R. Tolkien"
jsonpath "$..author" count == 4	                                           # all authors
jsonpath "$..author" nth 0 == "Nigel Rees"
jsonpath "$..author" nth 1 == "Evelyn Waugh"
jsonpath "$..author" nth 2 == "Herman Melville"
jsonpath "$..author" nth 3 == "J. R. R. Tolkien"
jsonpath "$.store.*" count == 2                                            # all things in store, which are some books and a red bicycle.
jsonpath "$.store..price" count == 5                                       # the price of everything in the store.
jsonpath "$..book[2].title" nth 0 == "Moby Dick"                           # the third book
jsonpath "$..book[-1].title" nth 0 == "The Lord of the Rings"              # the last book
#jsonpath "$..book[(@.length-1)].title" nth 0 == "The Lord of the Rings"   # not supported yet
#jsonpath "$..book[*]" last jsonpath "$.title" == "The Lord of the Rings"   # alternative
jsonpath "$..book[-1:].title" nth 0 == "The Lord of the Rings"             # the last book in order.
jsonpath "$..book[0,1]" count == 2                                         # the first two books
jsonpath "$..book[0,1].title" nth 0 == "Sayings of the Century"
jsonpath "$..book[0,1].title" nth 1 == "Sword of Honour"
jsonpath "$..book[-4,-3].title" nth 0 == "Sayings of the Century"
jsonpath "$..book[-4, -3].title" nth 1 == "Sword of Honour"
jsonpath "$..book[:2]" count == 2
jsonpath "$..book[:2].title" nth 0 == "Sayings of the Century"
jsonpath "$..book[:2].title" nth 1 == "Sword of Honour"
jsonpath "$..book[?(@.isbn)]" count == 2                                   # filter all books with isbn number
jsonpath "$..book[?(@.isbn)].title" nth 0 == "Moby Dick"
jsonpath "$..book[?(@.isbn)].title" nth 1 == "The Lord of the Rings"
jsonpath "$..book[?(@.isbn)].not_exist" count == 0                         # querying field that does not exist return an empty list
jsonpath "$..book[?(@.price<10)]" count == 2                               # filter all books cheaper than 10
jsonpath "$..book[?(@.price<10)].title" nth 0 == "Sayings of the Century"
jsonpath "$..book[?(@.price<10)].title" nth 1 == "Moby Dick"
jsonpath "$.store.book[?(@.published==true)].title" nth 0 == "Moby Dick"   # filter on published books
jsonpath "$..book[?(@.author != 'Charles Dickens')]" count == 4            # filter all books for author other than Charles Dickens
jsonpath "$..book[?(@.author != 'Nigel Rees')]" count == 3                 # filter all books for author other than Nogen Rees
jsonpath "$..book[?(@.price != 10.00)]" count == 4                         # filter all books for price different from 10.00
jsonpath "$..book[?(@.price != 8.95)]" count == 3                          # filter all books for price different from 8.95
jsonpath "$..*" count == 31                                                # all members of JSON structure.
jsonpath "$..book[*].isbn" count == 2                                      # get list of isbn numbers
jsonpath "$..book[*].not_exist" count == 0                                 # querying field that does not exist return an empty list
jsonpath "$..book[4]" not exists                                           # out of bound
jsonpath "$..book[-5]" not exists                                          # out of bound
jsonpath "$.store[0]" not exists                                           # array indexing for an object
jsonpath "$.store[-1]" not exists                                          # array indexing for an object
body jsonpath "$.store.book[0].category" == "reference"                    # `jsonpath` is also a filter
#jsonpath "$.store.book" last jsonpath "$.title" == "The Lord of the Rings" # other uses of `jsonpath` filter
#jsonpath "$.store.book" nth 2 jsonpath "$.isbn" == "0-553-21311-3"         #
jsonpath "$.store.bicycle.price" == 19.95                                  #
#jsonpath "$.store" jsonpath "$.bicycle" jsonpath "$.price" == 19.95        #

{
  "store": {
    "book": [
      { "category": "reference",
        "published": false,
        "author": "Nigel Rees",
        "title": "Sayings of the Century",
        "price": 8.95
      },
      { "category": "fiction",
        "published": false,
        "author": "Evelyn Waugh",
        "title": "Sword of Honour",
        "price": 12.99
      },
      { "category": "fiction",
        "published": true,
        "author": "Herman Melville",
        "title": "Moby Dick",
        "isbn": "0-553-21311-3",
        "price": 8.99
      },
      { "category": "fiction",
        "published": false,
        "author": "J. R. R. Tolkien",
        "title": "The Lord of the Rings",
        "isbn": "0-395-19395-8",
        "price": 22.99
      }
    ],
    "bicycle": {
      "color": "red",
      "price": 19.95
    }
  }
}
