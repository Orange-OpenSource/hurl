import json

from app import app
from flask import Response

DATA = {
    "first_name": "John",
    "last_name": "Smith",
    "is_alive": True,
    "age": 22,
    "address": {
        "street_address": "21 2nd Street",
        "city": "New York",
        "state": "NY",
        "postal_code": "10021-3100",
    },
    "phone_numbers": [
        {"type": "home", "number": "212 555-1234"},
        {"type": "office", "number": "646 555-4567"},
    ],
    "children": ["Catherine", "Thomas", "Trevor"],
    "spouse": None,
}


@app.route("/assert-body-json/ok")
def assert_body_json():
    return Response(
        json.dumps(DATA),
        mimetype="application/json",
    )


@app.route("/assert-body-json/age_modified")
def assert_body_json_age_modified():
    return Response(
        json.dumps({**DATA, "age": 20}),
        mimetype="application/json",
    )


@app.route("/assert-body-json/is_alive_deleted")
def assert_body_json_is_alive_deleted():
    data = {**DATA}
    del data["is_alive"]
    return Response(
        json.dumps(data),
        mimetype="application/json",
    )


@app.route("/assert-body-json/new_country")
def assert_body_json_new_country():
    return Response(
        json.dumps({**DATA, "country": "spain"}),
        mimetype="application/json",
    )
