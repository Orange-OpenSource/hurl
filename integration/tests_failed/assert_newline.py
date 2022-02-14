from app import app


@app.route("/error-assert-newline")
def error_assert_newline():
    return "<p>Hello</p>\n\n"
