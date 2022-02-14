from app import app
from flask import request


@app.route("/form-params", methods=["POST"])
def form_params():
    assert request.form["param1"] == "value1"
    assert request.form["param2"] == ""
    assert request.form["param3"] == "a=b"
    assert request.form["param4"] == "a%3db"
    assert request.form["values[0]"] == "0"
    assert request.form["values[1]"] == "1"
    return ""
