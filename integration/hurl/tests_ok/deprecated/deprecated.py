from app import app
from flask import Response


@app.route("/deprecated")
def deprecated():
    resp = Response("Hi!\n")
    resp.headers.add("X-Custom-Header", "value1")
    resp.headers.add("X-Custom-Header", "value2")
    resp.headers.add("X-Custom-Header", "value3")
    return resp
