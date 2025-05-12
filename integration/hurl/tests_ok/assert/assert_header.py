from app import app
from flask import Response, redirect


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


@app.route("/assert-header-location-http")
def assert_header_location_http():
    return redirect("http://localhost:8000")


@app.route("/assert-header-location-custom-scheme")
def assert_header_location_custom():
    return redirect("market://details?id=com.example.package")


@app.route("/assert-header-location-xxx")
def assert_header_location_xxx():
    return redirect("xxx")
