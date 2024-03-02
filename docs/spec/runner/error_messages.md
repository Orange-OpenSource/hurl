# Error messages

## Multiline string assert errors

### Motivation
Currently, the errors reported with a multiline-string body is "acceptable" for small content.

```
error: Assert body value
  --> test.hurl:4:1
   |
   | GET http://localhost:8000/test
   | ...
 4 | Hi World!
   | ^ actual value is <Hello World!
>
   |
```

But quickly it becomes unreadable when the body grows.

For example, expecting the following response
```
{
  "first_name": "John",
  "last_name": "Smith",
  "is_alive": true,
  "age": 27,
  "address": {
    "street_address": "21 2nd Street",
    "city": "New York",
    "state": "NY",
    "postal_code": "10021-3100"
  },
  "phone_numbers": [
    {
      "type": "home",
      "number": "212 555-1234"
    },
    {
      "type": "office",
      "number": "646 555-4567"
    }
  ],
  "children": [
    "Catherine",
    "Thomas",
    "Trevor"
  ],
  "spouse": null
}
```

and the server returning an error in the age (28 instead of 27), we got the following error message.
```
error: Assert body value
  --> test.hurl:4:1
   |
   | GET http://localhost:8000/test
   | ...
 4 | {
   | ^ actual value is <{
  "first_name": "John",
  "last_name": "Smith",
  "is_alive": true,
  "age": 28,
  "address": {
    "street_address": "21 2nd Street",
    "city": "New York",
    "state": "NY",
    "postal_code": "10021-3100"
  },
  "phone_numbers": [
    {
      "type": "home",
      "number": "212 555-1234"
    },
    {
      "type": "office",
      "number": "646 555-4567"
    }
  ],
  "children": [
    "Catherine",
    "Thomas",
    "Trevor"
  ],
  "spouse": null
}
>
   |
```


### Using diff output

A diff output (similar to git diff) could be used instead to display the error.

```
error: Assert body value
  --> test.hurl:4:1
   |
   | GET http://localhost:8000/test
   | ...
 4 | @@ -2,7 +2,7 @@
   |    "first_name": "John",
   |    "last_name": "Smith",
   |    "is_alive": true,
   | -  "age": 27,
   | +  "age": 28,
   |    "address": {
   |    "street_address": "21 2nd Street",
   |    "city": "New York",
   |
```

With coloring, il will be even more readable
```diff
-  "age": 27,
+  "age": 28,
```



The error message on the first simple example is now:

```
error: Assert body value
  --> test.hurl:4:1
   |
   | GET http://localhost:8000/test
   | ...
 4 | @@ -1 +1 @@
   | -Hello World!
   | +Hi World!
   |
```


### Detecting Whitespace

[![whitespace](trailing_space.png)]

The color is necessary here to see the additional whitespace.
We could also add a cli option to show explicitly "invisible characters".





