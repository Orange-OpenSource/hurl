from tests import app

@app.route("/predicate/error/type")
def predicate_error_type():
    return '{ "status": true, "message": "0", "count": 1, "empty": "", "number": 1.0 }'