from app import app
from flask import request


@app.route("/no-header")
def no_header():
    assert "User-Agent" not in request.headers
    assert "Accept" not in request.headers
    return "Hello World!"
