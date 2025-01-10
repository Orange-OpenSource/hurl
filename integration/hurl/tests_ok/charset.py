from app import app
from flask import make_response


@app.route("/charset/default")
def charset_default():
    return "<p>café</p>"


@app.route("/charset/utf8/uppercase-value")
def charset_uppercase_value():
    resp = make_response("<p>café</p>")
    resp.headers["Content-Type"] = "text/html; charset=UTF-8"
    return resp


@app.route("/charset/utf8/many-keys")
def charset_uppercase_many_keys():
    resp = make_response("<p>café</p>")
    resp.headers["Content-Type"] = (
        "text/plain; version=0.0.4; charset=utf-8; escaping=values"
    )
    return resp


@app.route("/charset/latin1")
def charset_latin1():
    resp = make_response("<p>café</p>".encode("latin1"))
    resp.headers["Content-Type"] = "text/html; charset=latin1"
    return resp


@app.route("/charset/latin1/uppercase-key")
def charset_latin1_uppercase_key():
    resp = make_response("<p>café</p>".encode("latin1"))
    resp.headers["Content-Type"] = "text/html; CHARSET=latin1"
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
