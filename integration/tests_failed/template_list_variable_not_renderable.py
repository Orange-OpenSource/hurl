from app import app


@app.route("/get-list")
def get_list():
    return '{"values":[1,2,3]}'

@app.route("/get-object")
def get_object():
    return """
        {
            "values": {
                "a": 1,
                "b": "two"
            }
        }
    """