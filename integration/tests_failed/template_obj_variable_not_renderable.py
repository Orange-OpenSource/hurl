from app import app

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