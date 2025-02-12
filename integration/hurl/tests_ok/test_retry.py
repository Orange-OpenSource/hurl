from http import HTTPStatus

from app import app
from flask import Response

flaky_state = {"count": 0}

@app.route("/flaky/set/<count>", methods=["POST"])
def set_flaky(count):
    flaky_state["count"] = int(count)
    return Response(status=HTTPStatus.OK, mimetype="application/json")


@app.route("/flaky")
def get_flaky():
    if flaky_state["count"] > 0:
       flaky_state["count"] = flaky_state["count"] - 1
       data = {"error": "500", "message": "internal server error"}
       return data, HTTPStatus.INTERNAL_SERVER_ERROR

    data = {"message": "success"}
    return data
