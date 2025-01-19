from app import app
from flask import request


@app.route("/header-option")
def header_option():
    assert (
        request.headers.get("key")
        == "from-header-syntax,from-cli,from-option-1,from-option-2,from-variable"
    )
    assert request.headers.get("another-key") == "another-from-option"
    assert request.headers.get("key-from-variable") == "value-from-variable"
    return ""
