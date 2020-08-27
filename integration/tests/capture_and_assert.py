from tests import app


@app.route("/capture-and-assert")
def capture_and_assert():
    return 'Hello World!'

