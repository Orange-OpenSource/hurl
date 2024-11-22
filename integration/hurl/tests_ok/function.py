import re

from app import app
from flask import request


@app.route("/function")
def function():
    uuid_pattern = "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
    assert re.match(uuid_pattern, request.args.get("uuid"))

    # check date with at least millisecond precision
    # TODO: depends currently on the OS / should be normalized to the same value
    date_pattern = "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}.[0-9]{6}"
    assert re.match(date_pattern, request.args.get("now"))

    return ""
