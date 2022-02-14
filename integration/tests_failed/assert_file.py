from app import app


@app.route("/error-assert-file")
def error_assert_file():
    return "Hello"
