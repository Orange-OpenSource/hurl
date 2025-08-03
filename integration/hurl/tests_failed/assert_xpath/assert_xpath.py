from app import app


@app.route("/error-assert-xpath")
def error_assert_xpath():
    return "<html><head><title>Test</title></head></html>"
