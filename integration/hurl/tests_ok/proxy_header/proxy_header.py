from app import app
from flask import request


@app.route("/proxy_header")
def proxy_header():
    assert request.headers["From-Proxy"] == "Hello"
    assert request.headers["Foo"] == "Bar"
    return ""


@app.route("/proxy_header_direct")
def proxy_header_direct():
    assert "From-Proxy" not in request.headers
    assert "Foo" not in request.headers
    return ""
