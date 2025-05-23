# coding=utf-8
from app import app
from flask import request


@app.route("/key-template/header")
def key_template_header():
    assert request.headers["name"] == "value"
    return ""


@app.route("/key-template/querystring")
def key_template_querystring():
    assert request.args.get("name") == "value"
    return ""


@app.route("/key-template/form", methods=["POST"])
def key_template_form():
    assert request.form["name"] == "value"
    return ""


@app.route("/key-template/multipart-form-data", methods=["POST"])
def key_template_multipart_form_data():
    assert request.form["name"] == "value"
    upload = request.files["file"]
    assert upload.filename == "data.txt"
    assert upload.content_type == "text/plain"
    assert upload.read() == b"Hello World!"
    return ""


@app.route("/key-template/cookie")
def key_template_cookie():
    assert request.cookies["name"] == "value"
    return ""


@app.route("/key-template/capture")
def key_template_capture():
    return "Hello"
