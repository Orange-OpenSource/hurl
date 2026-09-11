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


@app.route("/assert-body-json/value_mismatch_age")
def assert_body_json_value_mismatch_age():
    return Response(
        json.dumps({**DATA, "age": 20}),
        mimetype="application/json",
    )


@app.route("/assert-body-json/missing_key_is_alive")
def assert_body_json_missing_key_is_alive():
    data = {**DATA}
    del data["is_alive"]
    return Response(
        json.dumps(data),
        mimetype="application/json",
    )


@app.route("/assert-body-json/unexpected_key_country")
def assert_body_json_unexpected_key_country():
    return Response(
        json.dumps({**DATA, "country": "spain"}),
        mimetype="application/json",
    )


@app.route("/assert-body-json/array_value_mismatch_phone_number")
def assert_body_json_array_value_mismatch_phone_number():
    phone_numbers = [
        {**DATA["phone_numbers"][0], "number": "210 555-1234"},
        DATA["phone_numbers"][1],
    ]
    return Response(
        json.dumps({**DATA, "phone_numbers": phone_numbers}),
        mimetype="application/json",
    )


@app.route("/assert-body-json/missing_array_element_children")
def assert_body_json_missing_array_element_children():
    return Response(
        json.dumps({**DATA, "children": DATA["children"][:2]}),
        mimetype="application/json",
    )


@app.route("/assert-body-json/unexpected_array_element_children")
def assert_body_json_unexpected_array_element_children():
    return Response(
        json.dumps({**DATA, "children": [*DATA["children"], "Bob"]}),
        mimetype="application/json",
    )


@app.route("/assert-body-json/type_mismatch_age")
def assert_body_json_type_mismatch_age():
    return Response(
        json.dumps({**DATA, "age": "22"}),
        mimetype="application/json",
    )


@app.route("/assert-body-json/array_value_mismatch_children")
def assert_body_json_array_value_mismatch_children():
    return Response(
        json.dumps({**DATA, "children": DATA["children"][1:]}),
        mimetype="application/json",
    )
