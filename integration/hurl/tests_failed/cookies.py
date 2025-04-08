# coding=utf-8
from app import app
from flask import make_response


@app.route("/cookies/error")
def cookies_error():
    resp = make_response()
    resp.set_cookie(
        "cookie1",
        "value1",
        path="/",
        secure=True,
        httponly=True,
        expires="???",
    )
    return resp
