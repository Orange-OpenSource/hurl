# In each request, we sent a multiline body and get
# the same body as response. Request body is tested server side
# and we assert the response here.

POST http://localhost:8000/multilines/plain-text
```
line1
line2
line3
```
HTTP 200
# Different ways of testing body response:
# with explicit asserts:
[Asserts]
body == "line1\nline2\nline3\n"
body == ```
line1
line2
line3
```
# Or we can just test the body (implicit assert):
```
line1
line2
line3
```


POST http://localhost:8000/multilines/json
```json
{
  "foo": "bar"
  "baz": 123456
}
```
HTTP 200
# Different ways of testing body response:
# with explicit asserts:
[Asserts]
body == "{\n  \"foo\": \"bar\"\n  \"baz\": 123456\n}\n"
body == ```json
{
  "foo": "bar"
  "baz": 123456
}
```
# Or we can just test the body (implicit assert):
```json
{
  "foo": "bar"
  "baz": 123456
}
```



POST http://localhost:8000/multilines/xml
```xml
<?xml version="1.0"?>
<catalog>
    <book id="bk101">
        <author>Gambardella, Matthew</author>
        <title>XML Developer's Guide</title>
        <genre>Computer</genre>
        <price>44.95</price>
        <publish_date>2000-10-01</publish_date>
        <description>An in-depth look at creating applications
        with XML.</description>
    </book>
</catalog>
```
HTTP 200
# Different ways of testing body response:
# with explicit asserts:
[Asserts]
body == "<?xml version=\"1.0\"?>\n<catalog>\n    <book id=\"bk101\">\n        <author>Gambardella, Matthew</author>\n        <title>XML Developer's Guide</title>\n        <genre>Computer</genre>\n        <price>44.95</price>\n        <publish_date>2000-10-01</publish_date>\n        <description>An in-depth look at creating applications\n        with XML.</description>\n    </book>\n</catalog>\n"
body == ```xml
<?xml version="1.0"?>
<catalog>
    <book id="bk101">
        <author>Gambardella, Matthew</author>
        <title>XML Developer's Guide</title>
        <genre>Computer</genre>
        <price>44.95</price>
        <publish_date>2000-10-01</publish_date>
        <description>An in-depth look at creating applications
        with XML.</description>
    </book>
</catalog>
```
# Or we can just test the body (implicit assert):
```xml
<?xml version="1.0"?>
<catalog>
    <book id="bk101">
        <author>Gambardella, Matthew</author>
        <title>XML Developer's Guide</title>
        <genre>Computer</genre>
        <price>44.95</price>
        <publish_date>2000-10-01</publish_date>
        <description>An in-depth look at creating applications
        with XML.</description>
    </book>
</catalog>
```



POST http://localhost:8000/multilines/graphql
```graphql
{
  hero {
    name
    # Queries can have comments!
    friends {
      name
    }
  }
}
```
HTTP 200
# Different ways of testing body response:
# with explicit asserts:
[Asserts]
body == "{\"query\":\"{\\n  hero {\\n    name\\n    # Queries can have comments!\\n    friends {\\n      name\\n    }\\n  }\\n}\"}"
body == ```graphql
{
  hero {
    name
    # Queries can have comments!
    friends {
      name
    }
  }
}
```
# Or we can just test the body (implicit assert):
```graphql
{
  hero {
    name
    # Queries can have comments!
    friends {
      name
    }
  }
}
```
