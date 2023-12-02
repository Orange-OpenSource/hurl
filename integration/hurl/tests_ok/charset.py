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


@app.route("/charset/latin1")
def charset_latin1():
    resp = make_response("<p>café</p>".encode("latin1"))
    resp.headers["Content-Type"] = "text/html; charset=latin1"
    return resp


@app.route("/charset/gb2312")
def charset_gb2312():
    resp = make_response("<p>你好世界</p>".encode("gb2312"))
    resp.headers["Content-Type"] = "text/html; charset=gb2312"
    return resp


@app.route("/charset/cp1256")
def charset_cp1256():
    resp = make_response("<p>مرحبا بالعالم</p>".encode("cp1256"))
    resp.headers["Content-Type"] = "text/html; charset=cp1256"
    return resp
