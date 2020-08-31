from flask import request, make_response
from tests import app


@app.route("/cookies/set-request-cookie1-valueA")
def set_request_cookie1_value1():
    assert request.cookies['cookie1'] == 'valueA'
    return ''


@app.route("/cookies/set-session-cookie2-valueA")
def set_session_cookie2_valuea():
    resp = make_response()
    resp.set_cookie('cookie2', 'valueA')
    return resp


@app.route("/cookies/set-request-cookie2-valueB")
def set_request_cookie2_valueb():
    assert request.cookies['cookie2'] == 'valueB'
    return ''


@app.route("/cookies/send-cookie2-value1")
def send_cookie2_value1():
    assert'cookie1' not in request.cookies
    assert request.cookies['cookie2'] == 'value1'
    return ''


@app.route("/cookies/send-cookie2-value2")
def send_cookie2_value2():
    assert request.cookies['cookie2'] == 'value2'
    return ''


@app.route("/cookies/delete-cookie2")
def delete_cookie2():
    resp = make_response()
    resp.set_cookie('cookie2', '', max_age=0)
    return resp


@app.route("/cookies/assert-that-cookie1-is-not-in-session")
def assert_that_cookie1_is_not_in_session():
    assert'cookie1' not in request.cookies
    return ''


@app.route("/cookies/assert-that-cookie2-is-not-in-session")
def assert_that_cookie2_is_not_in_session():
    assert'cookie2' not in request.cookies
    return ''


@app.route("/cookies/assert-that-cookie2-is-valueA")
def assert_that_cookie2_is_valuea():
    assert request.cookies['cookie2'] == 'valueA'
    return ''

@app.route("/cookies/assert-that-cookie2-is-valueB")
def assert_that_cookie2_is_valueb():
    assert request.cookies['cookie2'] == 'valueB'
    return ''

@app.route("/cookies/set-session-cookie2-valueA-subdomain")
def set_session_cookie2_valuea_subdomain():
    resp = make_response()
    resp.set_cookie('cookie2', 'valueA', domain='myshop.orange.localhost')
    return resp

@app.route("/cookies/set-session-cookie2-valueA-subdomain2")
def set_session_cookie2_valuea_subdomain2():
    resp = make_response()
    resp.set_cookie('cookie2', 'valueA', domain='orange.localhost')
    return resp


# Set-Cookie: LSID=; Path=/accounts; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly
# Set-Cookie: HSID=AYQEVn…DKrdst; Domain=.localhost; Path=/; Expires=Wed, 13 Jan 2021 22:23:01 GMT; HttpOnly
# Set-Cookie: SSID=Ap4P…GTEq; Domain=.localhost; Path=/; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly
@app.route("/cookies/set")
def set_cookies():
    resp = make_response()
    resp.set_cookie('LSID', 'DQAAAKEaem_vYg', path='/accounts', secure=True, httponly=True, expires='Wed, 13 Jan 2021 22:23:01 GMT')
    resp.set_cookie('HSID', 'AYQEVnDKrdst', domain='.localhost', path='/', expires='Wed, 13 Jan 2021 22:23:01 GMT', httponly=True)
    resp.set_cookie('SSID', 'Ap4PGTEq', domain='.localhost', path='/', expires='Wed, 13 Jan 2021 22:23:01 GMT', secure=True, httponly=True)
    return resp




