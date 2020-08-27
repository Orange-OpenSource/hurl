from tests import app

@app.route("/error-file-read-access")
def error_file_read_access():
    return ''