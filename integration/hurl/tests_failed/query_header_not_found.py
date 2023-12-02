from app import app


@app.route("/error-query-header-not-found")
def error_query_header_not_found():
    return "Hello World!"
