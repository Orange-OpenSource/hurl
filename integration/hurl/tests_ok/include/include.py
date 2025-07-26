from app import app
from flask import Response


@app.route("/include")
def include():
    response = Response("Hello")
    # Waitress will capitalize these response headers to X-Foo and X-Bar
    response.headers["x-foo"] = "bar"
    response.headers["X-BAR"] = "baz"
    return response
