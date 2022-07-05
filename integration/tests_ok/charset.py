from app import app
from flask import request, make_response


@app.route("/charset/default")
def charset_default():
    return "<p>Hello World!</p>"


@app.route("/charset/uppercase")
def charset_uppercase():
    resp = make_response("<p>Hello World!</p>")
    resp.headers["Content-Type"] = "text/html; charset=UTF-8"
    return resp
