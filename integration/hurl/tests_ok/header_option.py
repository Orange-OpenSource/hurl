from app import app
from flask import request


@app.route("/header-option")
def header_option():
    assert (
        request.headers.get("test")
        == "from-header-syntax,from-cli,from-option-1,from-option-2"
    )
    assert request.headers.get("another-test") == "from-option"
    return ""
