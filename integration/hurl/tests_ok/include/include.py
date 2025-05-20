from app import app
from flask import Response


@app.route("/include")
def include():
    response = Response("Hello")
    response.headers["x-foo"] = "bar"
    response.headers["X-BAR"] = "baz"
    return response
