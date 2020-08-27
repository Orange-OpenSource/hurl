from tests import app
from flask import Response

@app.route("/error-invalid-jsonpath")
def error_invalid_jsonpath():
    return Response('{"success":false,"errors":[{"id":"error1"},{"id":"error2"}]}', mimetype='application/json')
