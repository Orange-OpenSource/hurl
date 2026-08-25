from app import app
from flask import make_response


@app.route("/evil-headers")
def evil_header():
    response = make_response("Hello world!")
    # An evil value
    response.headers.set("X-Foo", "<script>alert('Hello')</script>")
    # An evil name
    response.headers.set("<script>alert('Hello')</script>", "foo")
    return response
