from app import app
from flask import request


@app.route("/fileroot", methods=["GET", "POST"])
def fileroot():
    data = '{"user":"bob","age":21}'
    if request.method == "POST":
        s = request.data.decode("utf-8")
        assert s == data
        return ""
    else:
        return data
