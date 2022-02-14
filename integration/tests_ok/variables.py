from flask import request
from app import app
import json


@app.route("/variables", methods=["POST"])
def variables():
    assert request.headers["Content-Type"] == "application/json"
    assert request.headers["Name"] == "Jennifer"
    assert request.headers["Age"] == "30"
    assert request.headers["Female"] == "true"
    assert request.headers["Id"] == "123"
    assert request.headers["Height"] == "1.7"
    assert request.headers["A_null"] == "null"

    s = request.data.decode("utf-8")
    data = json.loads(s)
    assert data["name"] == "Jennifer"
    assert data["age"] == 30
    assert data["female"] == True
    assert data["id"] == "123"
    assert data["height"] == 1.7
    assert data["a_null"] is None
    return ""
