from app import app
from flask import Response


@app.route("/assert-json")
def assert_json():
    return Response(
        """{
  "count": 5,
  "success": false,
  "errors": [{"id":"error1"},{"id":"error2"}],
  "failures": [{"id":"failure1"}],
  "warnings": [],
  "message": "Bob says \\"Hello\\"",
  "duration": 1.5,
  "tags": ["test"],
  "nullable": null,
  "profile-id": "123abc",
  "dates": [
    "2022-10-31T09:00:00.594Z",
    "2024-03-20T11:23:56.773+02:00"
  ],
  "empty": {}
}""",
        mimetype="application/json",
    )


@app.route("/assert-json/index")
def assert_json_index():
    return "1"


@app.route("/assert-json/list")
def assert_json_list():
    return Response(
        """[
  { "id": 1, "name": "Bob"},
  { "id": 2, "name": "Bill"}
]""",
        mimetype="application/json",
    )


@app.route("/assert-json/filter")
def assert_json_filter():
    return Response(
        """{
    "fruit": [
        {
            "name": "apple",
            "price": {
                "US": 100,
                "UN": 110
            }
        },
        {
            "name": "grape",
            "price": {
                "US": 200,
                "UN": 150
            }
        }
    ]
}""",
        mimetype="application/json",
    )


@app.route("/assert-json/filter-by-name")
def assert_json_filter_ny_name():
    return Response(
        """{
  "main": {
    "items": [
      {
        "id": 1,
        "name": "car"
      },
      {
        "id": 2,
        "name": "bike",
        "items": [
          {
            "id": 4,
            "name": "wheel"
          }
        ]
      },
      {
        "id": 3,
        "name": "plane"
      }
    ]
  },
  "more": {
    "items": [
      {
        "id": 5,
        "name": "scooter"
      }
    ]
  }
}""",
        mimetype="application/json",
    )


@app.route("/assert-json/big-number")
def assert_json_big_number():
    return Response(
        """{
  "big_integer": 1000000000000000000000
}""",
        mimetype="application/json",
    )
