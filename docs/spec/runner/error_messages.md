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
  --> test.hurl:8:1
   |
   | GET http://localhost:8000/test
   | ...
   |    "first_name": "John",
   |    "last_name": "Smith",
   |    "is_alive": true,
 8 | -  "age": 27,
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
 4 | -Hello World!
   | +Hi World!
   |
```


### Detecting Whitespace

[![whitespace](trailing_space.png)]

The color is necessary here to see the additional whitespace.
We could also add a cli option to show explicitly "invisible characters".


## Display more or less context lines before first change

### Change at line 1

Standard unified diff

```
@@ -1,4 +1,4 @@
-{
+[
   "first_name": "John",
   "last_name": "Smith",
   "is_alive": true,
```

Hurl Output

```
error: Assert body value
  --> test.hurl:4:1
   |
   | GET http://localhost:8000/test
 4 | -{
   | +[  
   |    "first_name": "John",
   |    "last_name": "Smith",
   |    "is_alive": true,
```


### Change at line 2


Standard unified diff

```
@@ -1,5 +1,5 @@
 {
-  "first_name": "John",
+  "first_name": "Bob",
   "last_name": "Smith",
   "is_alive": true,
   "age": 27,
```

Hurl Output

```
error: Assert body value
  --> test.hurl:5:1
   |
   | GET http://localhost:8000/test
   |  {
 5 |-   "first_name": "John",
   |+   "first_name": "Bob",
   |    "is_alive": true,
   |    "age": 27,
```


### Change at line 3

Standard unified diff

```
@@ -1,6 +1,6 @@
 {
   "first_name": "John",
-  "last_name": "Smith",
+  "last_name": "Smiths",
   "is_alive": true,
   "age": 27,
   "address": {
```

Hurl Output

```
error: Assert body value
  --> test.hurl:6:1
   |
   | GET http://localhost:8000/test
   | {
   |   "first_name": "John",
6  |-  "last_name": "Smith",
   |+  "last_name": "Smiths",
   |   "is_alive": true,
   |   "age": 27,
   |   "address": {
```



## Deletion


Standard unified diff

```
@@ -1,6 +1,5 @@
 {
   "first_name": "John",
-  "last_name": "Smith",
   "is_alive": true,
   "age": 27,
   "address": {
```

Hurl Output

```
error: Assert body value
  --> test.hurl:6:1
   |
   | GET http://localhost:8000/test
   | {
   |   "first_name": "John",
6  |-  "last_name": "Smith",
   |   "is_alive": true,
   |   "age": 27,
   |   "address": {
```


## Addition

Standard unified diff

```
@@ -1,5 +1,6 @@
 {
   "first_name": "John",
+  "middle_name": "Bob",
   "last_name": "Smith",
   "is_alive": true,
   "age": 27,
```

Hurl Output

```
error: Assert body value
  --> test.hurl:5:1
   |
   | GET http://localhost:8000/test
   | {
5  |   "first_name": "John",
   |+  "middle_name": "Bob",
   |   "last_name": "Smith",
   |   "is_alive": true,
   |   "age": 27,
   |   "address": {
```


## Display several diff Hunks


Standard Unified Diff

```
@@ -2,7 +2,7 @@
   "first_name": "John",
   "last_name": "Smith",
   "is_alive": true,
-  "age": 27,
+  "age": 28,
   "address": {
     "street_address": "21 2nd Street",
     "city": "New York",
@@ -22,7 +22,7 @@
   "children": [
     "Catherine",
     "Thomas",
-    "Trevor"
+    "Bob"
   ],
   "spouse": null
 }
```

```
error: Assert body value
  --> test.hurl:8:1
   |
   | GET http://localhost:8000/test
   | ...
   |   "first_name": "John",
   |-  "last_name": "Smith",
   |   "is_alive": true,
 8 |-  "age": 27,
   |+  "age": 28,
   |   "address": {
   |     "street_address": "21 2nd Street",
   |     "city": "New York",
   | ...
   |   "children": [
   |     "Catherine",
   |     "Thomas",
28 |-    "Trevor"
   |+    "Bob"
   |   ],
   |   "spouse": null
   | }  
  
```