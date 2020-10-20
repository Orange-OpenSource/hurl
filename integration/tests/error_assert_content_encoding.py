from tests import app
from flask import Response

@app.route("/error/content-encoding")
def error_assert_content_encoding():
    headers = {'Content-Encoding': 'unknown'}
    return Response('Hello', headers=headers)