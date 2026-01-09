from app import app
from flask import make_response, request


@app.route("/no_cookie_store/set")
def no_cookie_store_set():
    resp = make_response()
    resp.set_cookie(
        "foo",
        "1234",
        path="/no_cookie/check",
        expires="Mon, 12 Jan 2026 22:23:01 GMT",
    )
    return resp


@app.route("/no_cookie_store/check")
def no_cookie_store_check():
    assert "foo" not in request.cookies
    return ""


@app.route("/no_cookie_store/request_with_cookie")
def no_cookie_store_request_with_cookie():
    assert request.cookies["color"] == "blue"
    return ""
