from app import app
from flask import request


@app.route("/empty-value-header")
def empty_value_header():
    assert request.headers.get("X-Custom") == ""
    return ""
