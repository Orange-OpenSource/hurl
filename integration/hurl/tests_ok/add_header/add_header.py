from app import app
from flask import request


@app.route("/add-header")
def add_header():
    assert request.headers.get("header-b") == "baz"
    assert request.headers.get("header-c") == "qux"
    return ""


@app.route("/add-header-with-aggregation")
def add_header_with_aggregation():
    assert request.headers.get("header-a") == "foo"
    assert request.headers.get("header-b") == "baz"
    assert request.headers.get("header-c") == "qux"
    return ""


@app.route("/add-header-with-duplicate")
def add_header_with_duplicate():
    assert request.headers.get("header-b") == "bar,baz"
    assert request.headers.get("header-c") == "qux"
    assert request.get_json()["message"] == "hi!"
    return ""
