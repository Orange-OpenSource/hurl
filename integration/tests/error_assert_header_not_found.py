from tests import app

@app.route("/error-assert-header-not-found")
def error_assert_header_not_found():
    return 'Hello World!'

