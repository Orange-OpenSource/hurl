import json

from app import app
from flask import Response


@app.route("/error-format-long/html")
def error_format_html():
    return "<html><head><title>Test</title></head></html>"


@app.route("/error-format-long/json")
def error_format_json():
    data = {
        "books": [
            {
                "name": "Dune",
                "author": "Franck Herbert",
            },
            {
                "name": "Les Misérables",
                "author": "Victor Hugo",
            },
        ]
    }
    return Response(json.dumps(data), mimetype="application/json")


@app.route("/error-format-long/rfc-7807")
def error_format_problem_json():
    data = {
        "type": "https://example.com/probs/out-of-credit",
        "title": "You do not have enough credit.",
        "detail": "Your current balance is 30, but that costs 50.",
        "instance": "/account/12345/msgs/abc",
        "balance": 30,
        "accounts": ["/account/12345", "/account/67890"],
    }
    return Response(json.dumps(data), mimetype="application/problem+json")
