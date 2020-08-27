from tests import app
from flask import make_response, request
from io import BytesIO


@app.route('/bytes')
def bytes():
    result = BytesIO()
    result.write(b'\xff')
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = 'application/octet-stream'
    return resp



