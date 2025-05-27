from app import app


@app.route("/no-output")
def no_output():
    return app.response_class(headers={"date": "DATE1"}, response="Hello world!\n")
