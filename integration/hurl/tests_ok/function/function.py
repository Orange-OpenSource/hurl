import re

from app import app
from flask import request


@app.route("/function")
def function():
    uuid = request.args.get("uuid")
    uuid_pattern = "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
    assert re.match(uuid_pattern, uuid)

    # check UTC date yyyy-mm-ddYhh:mm:ss.xxxxxxZ
    date_str = request.args.get("now")
    date_pattern = "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}.[0-9]{6}Z$"
    assert re.match(date_pattern, date_str)

    return ""
