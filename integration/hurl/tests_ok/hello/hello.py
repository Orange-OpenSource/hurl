from app import app
from flask import request


@app.route("/hello")
def hello():
    assert len(request.headers) == 3
    assert request.headers["Host"] == "localhost:8000"
    assert request.headers["Accept"] == "*/*"
    assert "User-Agent" in request.headers
    assert len(request.data) == 0
    return "Hello World!"
