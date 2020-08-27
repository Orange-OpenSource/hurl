from tests import app

@app.route("/assert-base64")
def assert_base64():
    return 'line1\nline2\r\nline3\n'