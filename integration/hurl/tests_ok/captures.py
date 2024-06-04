from app import app
from flask import make_response, request


@app.route("/captures")
def captures():
    resp = make_response()
    resp.data = "Hello world!"
    resp.headers["Header1"] = "value1"
    resp.headers["Header2"] = "Hello Bob!"
    return resp


@app.route("/captures-check")
def captures_check():
    assert request.args.get("param1") == "value1"
    assert request.args.get("param2") == "Bob"
    return ""


@app.route("/captures-xml")
def captures_xml():
    return (
        "<!DOCTYPE html>"
        "<html>"
        "<head>"
        '<meta charset="utf-8">'
        "<title>title</title>"
        "</head>"
        "<body>"
        "<p>Lorem ipsum dolor sit amet</p>"
        "<p>Sed ut perspiciatis unde omnis</p>"
        "</body>"
        "</html>"
    )


@app.route("/captures-json")
def captures_json():
    return (
        "{"
        '"a_null":null,'
        '"an_object":{"id": "123"},'
        '"a_list":[1,2,3],'
        '"an_integer":1,'
        '"a_big_integer":10000000000000000365,'
        '"a_float":1.1,'
        '"a_bool":true,'
        '"a_string":"hello",'
        '"a_date_like_string":"2012-04-23T18:25:43.511Z"'
        "}"
    )
