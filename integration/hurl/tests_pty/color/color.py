from app import app
from flask import Response


@app.route("/pty/color")
def pty_color():
    return Response(
        """{
  "store": {
    "book": [
      {
        "published": true,
        "title": "Sayings of the Century",
        "price": 8.95,
              "ratings": [
      ],
        "notes": {
        }
      },
      {
        "published": false,
        "title": "Sword of Honour",
        "price": 12.99,
              "ratings": [
      ],

        "notes": {
        }
      }
    ]
  }
}""",
        mimetype="application/json",
    )
