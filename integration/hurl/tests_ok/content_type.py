from app import app
from flask import request


@app.route("/content-type-json", methods=["POST"])
def content_type_json():
    assert request.headers["Content-Type"] == "application/json"
    return ""


@app.route("/content-type-vnd-json", methods=["POST"])
def content_type_vnd_json():
    assert request.headers["Content-Type"] == "application/vnd.api+json"
    return ""


@app.route("/content-type-form", methods=["POST"])
def content_type_form():
    assert request.headers["Content-Type"] == "application/x-www-form-urlencoded"
    return ""


@app.route("/content-type-multipart", methods=["POST"])
def content_type_multipart():
    assert "multipart/form-data" in request.headers["Content-Type"]
    return ""


@app.route("/content-type-xml", methods=["POST"])
def content_type_implicit_xml():
    assert request.headers["Content-Type"] == "application/xml"
    return ""
