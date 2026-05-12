from app import app
from flask import Response


@app.route("/fail_with_body")
def fail_with_body():
    return Response(
        '{"fruits":[{"id":1,"name":"Apple","quantity":12,"isFresh":true},{"id":2,"name":"Banana","quantity":8,"isFresh":true},{"id":3,"name":"Orange","quantity":15,"isFresh":false},{"id":4,"name":"Grape","quantity":25,"isFresh":true},{"id":5,"name":"Mango","quantity":6,"isFresh":false},{"id":6,"name":"Pineapple","quantity":3,"isFresh":true},{"id":7,"name":"Strawberry","quantity":18,"isFresh":true},{"id":8,"name":"Blueberry","quantity":20,"isFresh":false},{"id":9,"name":"Watermelon","quantity":2,"isFresh":true},{"id":10,"name":"Kiwi","quantity":9,"isFresh":false}]}',
        mimetype="application/json",
    )
