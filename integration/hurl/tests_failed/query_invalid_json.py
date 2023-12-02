from app import app


@app.route("/error-query-invalid-json")
def error_query_invalid_json():
    return "Hello World!"
