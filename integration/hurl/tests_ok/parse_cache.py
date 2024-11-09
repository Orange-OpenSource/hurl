from app import app
from flask import make_response


@app.route("/large/html")
def large_html():
    data = open("tests_ok/parse_cache.html.gz", "rb")
    resp = make_response(data)
    resp.headers["Content-Type"] = "text/html; charset=utf-8"
    resp.headers["Content-Encoding"] = "gzip"
    return resp


@app.route("/large/json")
def large_json():
    data = open("tests_ok/parse_cache.json.gz", "rb")
    resp = make_response(data)
    resp.headers["Content-Type"] = "application/json"
    resp.headers["Content-Encoding"] = "gzip"
    return resp
