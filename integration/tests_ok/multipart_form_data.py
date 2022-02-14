#  curl -v -F key1=value1 -F upload1=@tests_ok/data.txt -Fupload2=@tests_ok/data.html http://localhost:8000/multipart-form-data
from app import app
from flask import request


@app.route("/multipart-form-data", methods=["POST"])
def multipart_form_data():

    assert request.form["key1"] == "value1"
    assert "Expect" not in request.headers

    upload1 = request.files["upload1"]
    assert upload1.filename == "data.txt"
    assert upload1.content_type == "text/plain"
    assert upload1.read() == b"Hello World!"

    upload2 = request.files["upload2"]
    assert upload2.filename == "data.html"
    assert upload2.content_type == "text/html"
    assert upload2.read() == b"<div>Hello <b>World</b>!</div>"

    upload3 = request.files["upload3"]
    assert upload3.filename == "data.txt"
    assert upload3.content_type == "text/html"
    assert upload3.read() == b"Hello World!"

    return ""
