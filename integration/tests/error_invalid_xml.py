from tests import app

@app.route("/error-invalid-xml")
def error_invalid_xml():
    return ''