from tests import app
from flask import make_response
from io import BytesIO


@app.route('/error-assert/match-utf8')
def error_assert_match_utf8():
    result = BytesIO()
    result.write(b'\xff')
    data = result.getvalue()
    resp = make_response(data)
    return resp


