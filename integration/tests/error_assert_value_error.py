from tests import app

@app.route("/error-assert-value")
def error_assert_value():
    return '{ "values": [1,2,3]}'

