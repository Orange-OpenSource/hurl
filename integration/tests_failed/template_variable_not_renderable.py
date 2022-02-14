from app import app


@app.route("/get-list")
def get_list():
    return '{"values":[1,2,3]}'
