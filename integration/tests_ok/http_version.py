from app import app
from flask import request


@app.route("/http_version/10")
def http_10():
    version = request.environ["SERVER_PROTOCOL"]
    assert version == "HTTP/1.0"
    return "HTTP/1.0"


@app.route("/http_version/11")
def http_11():
    version = request.environ["SERVER_PROTOCOL"]
    assert version == "HTTP/1.1"
    return "HTTP/1.1"
