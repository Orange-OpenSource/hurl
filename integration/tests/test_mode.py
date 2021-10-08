from tests import app
from flask import request

@app.route("/test-mode")
def test_mode():
    assert 'Content-Type' not in request.headers
    assert 'Content-Length' not in request.headers
    assert len(request.data) == 0
    return 'Hello World!'

