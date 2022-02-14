from app import app
from flask import request


@app.route("/hello")
def hello():
    assert "Content-Type" not in request.headers
    assert "Content-Length" not in request.headers
    assert len(request.data) == 0
    return "Hello World!"
