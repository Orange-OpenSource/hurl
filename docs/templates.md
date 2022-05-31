# Templates

## Variables

In Hurl file, you can generate value using two curly braces, i.e `{{my_variable}}`. For instance, if you want to reuse a
value from an HTTP response in the next entries, you can capture this value in a variable and reuse it in a template.

```hurl
GET https://example.org

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"

# Do the login !
POST https://acmecorp.net/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}

HTTP/1.1 302
```

In this example, we capture the value of the [CSRF token] from the body a first response, and inject it
as a header in the next POST request.

```hurl
GET https://example.org/api/index
HTTP/* 200
[Captures]
index: body

GET https://example.org/api/status
HTTP/* 200
[Asserts]
jsonpath "$.errors[{{index}}].id" == "error"
```

In this second example, we capture the body in a variable `index`, and reuse this value in the query
`jsonpath "$.errors[{{index}}].id"`.

## Types

Variable are typed, and can be either string, bool, number, `null` or collections. Depending on the variable type,
templates can be rendered differently. Let's say we have captured an integer value into a variable named
`count`:

```hurl
GET https://sample/counter
HTTP/* 200
[Captures]
count: jsonpath "$.results[0]"
```

The following entry:

```hurl
GET https://sample/counter/{{count}} 
HTTP/* 200
[Asserts]
jsonpath "$.id" == "{{count}}"
```

will be rendered at runtime to:

```hurl
GET https://sample/counter/458 
HTTP/* 200
[Asserts]
jsonpath "$.id" == "458"
```

resulting in a comparison between the [JSONPath] expression and a string value.

On the other hand, the following assert:

```hurl
GET https://sample/counter/{{count}} 
HTTP/* 200
[Asserts]
jsonpath "$.index" == {{count}}
```

will be rendered at runtime to:

```hurl
GET https://sample/counter/458 
HTTP/* 200
[Asserts]
jsonpath "$.index" == 458
```

resulting in a comparison between the [JSONPath] expression and an integer value.

So if you want to use typed values (in asserts for instances), you can use `{{my_var}}`.
If you're interested in the string representation of a variable, you can surround the variable with double quotes
, as in `"{{my_var}}"`.

> When there is no possible ambiguities, like using a variable in an url, or
> in a header, you can omit the double quotes. The value will always be rendered
> as a string.

## Injecting Variables

Variables can also be injected in a Hurl file:

- by using [`--variable` option]
- by using [`--variables-file` option]
- by defining environment variables, for instance `HURL_foo=bar`

Lets' see how to inject variables, given this `test.hurl`:

```hurl
GET https://{{host}}/{{id}}/status
HTTP/1.1 304

GET https://{{host}}/health
HTTP/1.1 200
```

### `variable` option

Variable can be defined with command line option:

```shell
$ hurl --variable host=example.net --variable id=1234 test.hurl
``` 


### `variables-file` option

We can also define all injected variables in a file:

```shell
$ hurl --variables-files vars.env test.hurl
``` 

where `vars.env` is

```
host=example.net
id=1234
```

### Environment variable

Finally, we can use environment variables in the form of `HURL_name=value`:

```shell
$ export HURL_host=example.net
$ export HURL_id=1234 
$ hurl test.hurl
``` 



## Templating Body

Using templates with [JSON body] or [XML body] is not currently supported in Hurl.
Besides, you can use templates in [raw string body] with variables to send a JSON or XML body:

~~~hurl
PUT https://example.org/api/hits
Content-Type: application/json
```
{
    "key0": "{{a_string}}",
    "key1": {{a_bool}},
    "key2": {{a_null}},
    "key3": {{a_number}}
}
```
~~~

Variables can be initialized via command line:

```shell
$ hurl --variable a_string=apple --variable a_bool=true --variable a_null=null --variable a_number=42 test.hurl
```

Resulting in a PUT request with the following JSON body:

```
{
    "key0": "apple",
    "key1": true,
    "key2": null,
    "key3": 42
}
```

[`--variable` option]: /docs/man-page.md#variable
[`--variables-file` option]: /docs/man-page.md#variables-file
[CSRF token]: https://en.wikipedia.org/wiki/Cross-site_request_forgery
[JSONPath]: /docs/asserting-response.md#jsonpath-assert
[JSON body]: /docs/request.md#json-body
[XML body]: /docs/request.md#xml-body
[raw string body]: /docs/request.md#raw-string-body
