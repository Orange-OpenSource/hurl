from tests import app
from flask import make_response

@app.route("/assert-header")
def assert_header():
    resp = make_response()
    resp.headers['Header1'] = 'value1'
    resp.set_cookie('cookie1', 'value1')
    resp.set_cookie('cookie2', 'value2')
    resp.set_cookie('cookie3', 'value3')
    return resp


