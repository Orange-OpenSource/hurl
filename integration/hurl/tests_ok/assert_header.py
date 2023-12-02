from app import app
from flask import make_response, Response


@app.route("/assert-header")
def assert_header():
    headers = {
        "Header1": "value1",
        "ETag": '"33a64df551425fcc55e4d42a148795d9f25f89d4"',
        "Expires": "Wed, 21 Oct 2015 07:28:00 GMT",
        "x-fruit": ["Banana", "Lemon", "Grape", "Strawberry"],
    }

    resp = Response("", headers=headers)
    resp.set_cookie("cookie1", "value1")
    resp.set_cookie("cookie2", "value2")
    resp.set_cookie("cookie3", "value3")
    return resp
