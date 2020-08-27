from tests import app

@app.route("/error-assert-base64")
def error_assert_base64():
    return 'line1\nline2\r\nline3\n'