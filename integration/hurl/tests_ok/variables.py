import json

from app import app
from flask import request


@app.route("/variables", methods=["POST"])
def variables():
    assert request.headers["Content-Type"] == "application/json"
    assert request.headers["Name"] == "Jennifer"
    assert request.headers["Age"] == "30"
    assert request.headers["Female"] == "true"
    assert request.headers["Id"] == "123"
    assert request.headers["Height"] == "1.7"
    assert request.headers["A-null"] == "null"
    assert request.headers["Country"] == "Italy"
    assert request.headers["Planet"] == "The Earth"
    assert request.headers["Galaxy"] == "Milky Way"
    assert request.headers["BigInt"] == "9223372036854775808"

    s = request.data.decode("utf-8")
    data = json.loads(s)
    assert data["name"] == "Jennifer"
    assert data["age"] == 30
    assert data["female"] is True
    assert data["id"] == "123"
    assert data["height"] == 1.7
    assert data["a_null"] is None
    assert data["country"] == "Italy"
    assert data["planet"] == "The Earth"
    assert data["galaxy"] == "Milky Way"
    assert data["big_int"] == 9223372036854775808
    return ""


@app.route("/variable/country")
def country():
    return "Italy"


@app.route("/variable/planet")
def planet():
    return "The Earth"
