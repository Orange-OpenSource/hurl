from tests import app
from flask import Response

@app.route("/error-assert-decompress")
def error_assert_decompress():
    headers = {}
    headers['Content-Encoding'] = 'gzip'
    return Response('Hello', headers=headers)