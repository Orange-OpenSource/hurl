from app import app


@app.route("/filter")
def filter():
    return """{
  "file": "5L2g5aW95LiW55WM",
  "list": [1,2,3],
  "message": "Hello Bob!",
  "url": "https://mozilla.org/?x=шеллы",
  "encoded_url": "https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B",
  "text": "a > b && a < c",
  "escaped_html": [
    "a &gt; b &amp;&amp; a &lt; c",
    "Foo &#xA9; bar &#x1D306; baz &#x2603; qux",
    "&#65 foo"
  ],
  "id": "123",
  "score": 1.6,
  "ips": "192.168.2.1, 10.0.0.20, 10.0.0.10",
  "json": "{\\"message\\": \\"Hello\\"}",
  "pi": "3.141592653589793",
  "ten": 10
}"""
