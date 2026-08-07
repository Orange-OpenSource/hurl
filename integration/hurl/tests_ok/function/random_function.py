import re

from app import app
from flask import request


@app.route("/random-function")
def random_function():
    assert request.args.get("bool") in ["true", "false"]

    email = request.args.get("email")
    assert re.match(r"^[^@\s]+@[^@\s]+$", email)

    value = int(request.args.get("int"))
    assert 10 <= value <= 99

    string = request.args.get("string")
    assert re.match(r"^[A-Za-z0-9]{32}$", string)

    for name in ["first-name", "last-name", "full-name", "word"]:
        assert len(request.args.get(name)) > 0

    return ""
