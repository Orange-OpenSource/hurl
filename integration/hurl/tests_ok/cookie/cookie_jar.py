from app import app
from flask import make_response


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
    return resp
