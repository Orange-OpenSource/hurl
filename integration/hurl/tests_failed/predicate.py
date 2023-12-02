from app import app


@app.route("/predicate/error/type")
def predicate_error_type():
    return """{
     "status": true, 
     "message": "0", 
     "count": 1, 
     "empty": "", 
     "number": 1.0,
     "list": [1,2,3] 
}"""
