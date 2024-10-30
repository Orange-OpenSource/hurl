import json
from io import BytesIO

from app import app
from flask import Response, make_response


@app.route("/error-format-long/html")
def error_format_html():
    return "<html><head><title>Test</title></head></html>"


@app.route("/error-format-long/json")
def error_format_json():
    data = {
        "books": [
            {
                "name": "Dune",
                "author": "Franck Herbert",
            },
            {
                "name": "Les Misérables",
                "author": "Victor Hugo",
            },
        ]
    }
    return Response(json.dumps(data), mimetype="application/json")


@app.route("/error-format-long/rfc-7807")
def error_format_problem_json():
    data = {
        "type": "https://example.com/probs/out-of-credit",
        "title": "You do not have enough credit.",
        "detail": "Your current balance is 30, but that costs 50.",
        "instance": "/account/12345/msgs/abc",
        "balance": 30,
        "accounts": ["/account/12345", "/account/67890"],
    }
    return Response(json.dumps(data), mimetype="application/problem+json")


@app.route("/error-format-long/fhir")
def error_format_fhir_json():
    data = {
        "resourceType": "Practitioner",
        "id": "example",
        "text": {
            "status": "generated",
            "div": '<div xmlns="http://www.w3.org/1999/xhtml">\n      <p>Dr Adam Careful is a Referring Practitioner for Acme Hospital from 1-Jan 2012 to 31-Mar\n        2012</p>\n    </div>',
        },
        "identifier": [{"system": "http://www.acme.org/practitioners", "value": "23"}],
        "active": True,
        "name": [{"family": "Careful", "given": ["Adam"], "prefix": ["Dr"]}],
        "address": [
            {
                "use": "home",
                "line": ["534 Erewhon St"],
                "city": "PleasantVille",
                "state": "Vic",
                "postalCode": "3999",
            }
        ],
        "qualification": [
            {
                "identifier": [
                    {
                        "system": "http://example.org/UniversityIdentifier",
                        "value": "12345",
                    }
                ],
                "code": {
                    "coding": [
                        {
                            "system": "http://terminology.hl7.org/CodeSystem/v2-0360/2.7",
                            "code": "BS",
                            "display": "Bachelor of Science",
                        }
                    ],
                    "text": "Bachelor of Science",
                },
                "period": {"start": "1995"},
                "issuer": {"display": "Example University"},
            }
        ],
    }
    return Response(json.dumps(data), mimetype="application/fhir+json")


@app.route("/error-format-long/bytes")
def error_format_bytes():
    result = BytesIO()
    result.write(b"\xde\xad\xbe\xef")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp


@app.route("/error-format-long/csv")
def error_format_csv():
    data = """\
"Year","Score","Title"
1968,86,"Greetings"
1970,17,"Bloody Mama"
1970,73,"Hi, Mom!"
1971,40,"Born to Win"
1973,98,"Mean Streets"
1973,88,"Bang the Drum Slowly"
1974,97,"The Godfather, Part II"
1976,41,"The Last Tycoon"
1976,99,"Taxi Driver"
1977,47,"1900"
1977,67,"New York, New York"
1978,93,"The Deer Hunter"
1980,97,"Raging Bull"
1981,75,"True Confessions"
1983,90,"The King of Comedy"
1984,89,"Once Upon a Time in America"
1984,60,"Falling in Love"
1985,98,"Brazil"
1986,65,"The Mission"
1987,00,"Dear America: Letters Home From Vietnam"
1987,80,"The Untouchables"
1987,78,"Angel Heart"
1988,96,"Midnight Run"
1989,64,"Jacknife"
1989,47,"We're No Angels"
1990,88,"Awakenings"
1990,29,"Stanley & Iris"
1990,96,"Goodfellas"
"""
    return Response(data, mimetype="text/csv")
