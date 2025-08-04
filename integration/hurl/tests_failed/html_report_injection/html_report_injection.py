from app import app


@app.route("/inline-script")
def inline_script():
    return "<script>alert('Hi')</script>"
