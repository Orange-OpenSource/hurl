# coding=utf-8
from tests import app


@app.route("/assert-regex")
def assert_regex():
    return 'Hello World!'

