from app import app
from flask import make_response


@app.route("/assert-header")
def assert_header():
    resp = make_response()
    resp.headers["Header1"] = "value1"
    resp.headers["ETag"] = '"33a64df551425fcc55e4d42a148795d9f25f89d4"'
    resp.set_cookie("cookie1", "value1")
    resp.set_cookie("cookie2", "value2")
    resp.set_cookie("cookie3", "value3")
    return resp
