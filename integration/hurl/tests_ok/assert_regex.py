# coding=utf-8
from app import app


@app.route("/assert-regex")
def assert_regex():
    return "Hello World!"
