from app import app
from flask import request


@app.route("/cookie-storage/assert-that-cookie1-is-valueA")
def cookie_storage_assert_that_cookie1_is_valuea():
    assert request.cookies["cookie1"] == "valueA"
    return ""


@app.route("/cookie-storage/assert-that-cookie1-is-not-in-session")
def cookie_storage_assert_that_cookie1_is_not_in_session():
    assert "cookie1" not in request.cookies
    return ""


@app.route("/cookie-storage/noop")
def cookie_storage_noop():
    return ""
