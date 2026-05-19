from app import app
from flask import request


@app.route("/no-header")
def no_header():
    assert "User-Agent" not in request.headers
    assert "Accept" not in request.headers
    return "Hello World!"


@app.route("/no-header/foo-bar")
def no_header_foo_bar():
    headers = ["Host", "Accept", "User-Agent", "foo", "bar"]
    assert len(request.headers) == len(headers)
    for h in headers:
        assert h in request.headers
    return "Foo Bar"


@app.route("/no-header/foo-bar-baz")
def no_header_foo_bar_baz():
    headers = ["Host", "Accept", "User-Agent", "foo", "bar", "baz"]
    assert len(request.headers) == len(headers)
    for h in headers:
        assert h in request.headers
    return "Foo Bar Baz"
