from app import app


@app.route("/error-xpath-to-json")
def error_xpath_to_json():
    return "<html><head><title>Test</title></head></html>"
