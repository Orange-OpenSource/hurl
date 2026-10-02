from app import app
from flask import make_response


def read_file(filename):
    with open(filename, "rb") as data:
        yield from data


@app.route("/large/html")
def large_html():
    data = read_file("tests_ok/parse_cache/parse_cache.html.gz")
    resp = make_response(data)
    resp.headers["Content-Type"] = "text/html; charset=utf-8"
    resp.headers["Content-Encoding"] = "gzip"
    return resp


@app.route("/large/json")
def large_json():
    data = read_file("tests_ok/parse_cache/parse_cache.json.gz")
    resp = make_response(data)
    resp.headers["Content-Type"] = "application/json"
    resp.headers["Content-Encoding"] = "gzip"
    return resp
