# coding=utf-8
from tests import app


@app.route("/assert-xpath")
def assert_xpath():
    return '<data>café</data>'

