# On can use the dedicated [MultipartFormData] section
POST http://localhost:8000/multipart-form-data
[MultipartFormData]
key1: value1
upload1: file,data.txt;
upload2: file,data.html;
upload3: file,data.txt; text/html
HTTP 200


POST http://localhost:8000/multipart-form-data
[Multipart]
key1: value1
upload1: file,data.txt;
upload2: file,data.html;
upload3: file,data.txt; text/html
HTTP 200


# Or, as an alternative to the [MultipartFormData] section, we can just use
# a multiline string body, constructing boundaries by hand (see https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST).
# The downside of this method is that we must inline the files content (for the moment).
POST http://localhost:8000/multipart-form-data
Content-Type: multipart/form-data; boundary="boundary"
```
--boundary
Content-Disposition: form-data; name="key1"

value1
--boundary
Content-Disposition: form-data; name="upload1"; filename="data.txt"
Content-Type: text/plain

Hello World!
--boundary
Content-Disposition: form-data; name="upload2"; filename="data.html"
Content-Type: text/html

<div>Hello <b>World</b>!</div>
--boundary
Content-Disposition: form-data; name="upload3"; filename="data.txt"
Content-Type: text/html

Hello World!
--boundary--
```
HTTP 200
