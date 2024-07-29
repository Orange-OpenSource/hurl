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
 4 | `Hello World!`
   | ^^^^^^^^^^^^^^ actual value is <Hi World!>
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
The context line from the input source file will be displayed like for the asserts errors.

```
error: Assert body value
  --> test.hurl:8:1
   |
   | GET http://localhost:8000/test
   | ...
 8 |   "age": 27,
   |   -  "age": 27,
   |   +  "age": 28,
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
 4 | `Hello World!`
   |   -Hello World!
   |   +Hi World!
   |
```


### Detecting Whitespace

[![whitespace](trailing_space.png)]

The color is necessary here to see the additional whitespace.
We could also add a cli option to show explicitly "invisible characters".


## Change

### Change at line 1

Standard unified diff (without context)

```
--- old.txt	2024-07-29 10:03:27.267387991 +0200
+++ new_line1.txt	2024-07-29 10:18:55.523048872 +0200
@@ -1 +1 @@
-{
+[
```

Hurl Output

```
error: Assert body value
  --> test.hurl:4:1
   |
   | GET http://localhost:8000/test
 4 | {
   |   -{
   |   +[  
```


### Change at line 2


Standard unified diff (without context)

```
@@ -2 +2 @@
-  "first_name": "John",
+  "first_name": "Bob",
```

Hurl Output

```
error: Assert body value
  --> test.hurl:5:1
   |
   | GET http://localhost:8000/test
   | ...
 5 |   "first_name": "John",
   |   -  "first_name": "John",
   |   +   "first_name": "Bob",
```


### Change at line 3

Standard unified diff (without context)

```
@@ -3 +3 @@
-  "last_name": "Smith",
+  "last_name": "Smiths",
```

Hurl Output

```
error: Assert body value
  --> test.hurl:6:1
   |
   | GET http://localhost:8000/test
   | ...
6  |   "last_name": "Smith",
   |   -  "last_name": "Smith",
   |   +  "last_name": "Smith",
```



## Deletion


Standard unified diff (without context)

```
@@ -3 +2,0 @@
-  "last_name": "Smith",
```

Hurl Output

```
error: Assert body value
  --> test.hurl:6:1
   |
   | GET http://localhost:8000/test
   | ...
6  |   "last_name": "Smith"
   |   - "last_name": "Smith"
```


## Addition

Standard unified diff (without context)

```
@@ -2,0 +3 @@
+  "middle_name": "Bob",
```

Hurl Output

```
error: Assert body value
  --> test.hurl:5:1
   |
   | GET http://localhost:8000/test
   | ...
5  |   "first_name": "John",
   |   +  "middle_name": "Bob",
```


## Display several diff Hunks


Standard unified diff (without context)

```
@@ -5 +5 @@
-  "age": 27,
+  "age": 28,
@@ -25 +25 @@
-    "Trevor"
+    "Bob"
```

```
error: Assert body value
  --> test.hurl:8:1
   |
   | GET http://localhost:8000/test
   | ...
 8 |   "age": 27,
   |   -  "age": 27,
   |   +  "age": 28,
   | ...
28 |     "Trevor"
   |   -    "Trevor"
   |   +    "Bob"
   |
```