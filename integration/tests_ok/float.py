from app import app
from flask import Response


@app.route("/float")
def float():
    return "[ -2.2, 0.0, 0.0000000000000001, 0.000000000000001, 0.333, 0.3333333333333333, 0.333333333333333333, 1.0, 1.001, 1.07, 1.070, 1.1, 1.5 ]"
