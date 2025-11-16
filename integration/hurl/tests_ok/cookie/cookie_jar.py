from app import app
from flask import make_response, request


@app.route("/cookie-jar")
def set_cookie_jar():
    resp = make_response()
    resp.set_cookie(
        "LSID",
        "DQAAAKEaem_vYg",
        path="/accounts",
        httponly=True,
        expires="Thu, 13 Jan 2078 22:23:01 GMT",
    )
    resp.set_cookie(
        "HSID",
        "AYQEVnDKrdst",
        domain="localhost",
        path="/",
        expires="Thu, 13 Jan 2078 22:23:01 GMT",
        httponly=True,
    )
    resp.set_cookie(
        "SSID",
        "Ap4PGTEq",
        domain="localhost",
        path="/",
        expires="Thu, 13 Jan 2078 22:23:01 GMT",
        httponly=True,
    )
    resp.set_cookie(
        "foo",
        "a b c",
        domain="localhost",
        path="/",
        expires="Thu, 13 Jan 2068 10:10:01 GMT",
        httponly=False,
    )
    return resp


@app.route("/cookie-jar/hello")
def cookie_jar_hello():
    cookies = request.cookies
    assert len(cookies) == 3
    assert cookies["HSID"] == "AYQEVnDKrdst"
    assert cookies["SSID"] == "Ap4PGTEq"
    assert cookies["foo"] == "a b c"
    return "Hello World!"
