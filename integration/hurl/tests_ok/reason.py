from app import app


@app.route("/reason-french")
def reason_french():
    return "Bonjour", "200 Succès"
