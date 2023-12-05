# \ud800 is not a valid code point
POST https://localhost:8000/hello
{
 "foo": "\ud800\ud800"
}
HTTP 200