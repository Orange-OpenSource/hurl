import re

from app import app
from flask import request


@app.route("/function")
def function():
    uuid_pattern = "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
    assert re.match(uuid_pattern, request.args.get("uuid"))
    return ""
