from flask import request, make_response
from tests import app


@app.route("/cookie-storage/assert-that-cookie1-is-valueA")
def cookiestorage_assert_that_cookie1_is_valuea():
    assert request.cookies['cookie1'] == 'valueA'
    return ''

@app.route("/cookie-storage/assert-that-cookie1-is-not-in-session")
def cookiestorage_assert_that_cookie1_is_not_in_session():
    assert'cookie1' not in request.cookies
    return ''



