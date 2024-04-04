from app import app


@app.route("/assert-body")
def assert_body():
    return "line1\nline2\nline3\n"


@app.route("/assert-body-with-crlf")
def assert_body_with_crlf():
    return "line1\nline2\r\nline3\n"
