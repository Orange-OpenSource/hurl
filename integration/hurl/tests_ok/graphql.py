from app import app
from flask import make_response, request


@app.route("/graphql", methods=["POST"])
def graphql():
    assert request.headers["Content-Type"] == "application/json"
    body_in = request.data.decode("utf-8")
    responses = {
        r'{"query":"{\n  allFilms {\n    films {\n      title\n      director\n      releaseDate\n    }\n  }\n}"}': r'{"data":{"allFilms":{"films":[{"title":"A New Hope","director":"George Lucas","releaseDate":"1977-05-25"},{"title":"The Empire Strikes Back","director":"Irvin Kershner","releaseDate":"1980-05-17"},{"title":"Return of the Jedi","director":"Richard Marquand","releaseDate":"1983-05-25"},{"title":"The Phantom Menace","director":"George Lucas","releaseDate":"1999-05-19"},{"title":"Attack of the Clones","director":"George Lucas","releaseDate":"2002-05-16"},{"title":"Revenge of the Sith","director":"George Lucas","releaseDate":"2005-05-19"}]}}}',
        r'{"query":"query Query {\n  allFilms {\n    films {\n      title\n      director\n      releaseDate\n    }\n  }\n}"}': r'{"data":{"allFilms":{"films":[{"title":"A New Hope","director":"George Lucas","releaseDate":"1977-05-25"},{"title":"The Empire Strikes Back","director":"Irvin Kershner","releaseDate":"1980-05-17"},{"title":"Return of the Jedi","director":"Richard Marquand","releaseDate":"1983-05-25"},{"title":"The Phantom Menace","director":"George Lucas","releaseDate":"1999-05-19"},{"title":"Attack of the Clones","director":"George Lucas","releaseDate":"2002-05-16"},{"title":"Revenge of the Sith","director":"George Lucas","releaseDate":"2005-05-19"}]}}}',
        r'{"query":"query Person($id: ID!) {\n  person(id: $id) {\n    name\n  }\n}","variables":{"id":"cGVvcGxlOjQ="}}': r'{"data":{"person":{"name":"Darth Vader"}}}',
    }
    data = responses[body_in]
    resp = make_response(data)
    resp.headers["Content-Type"] = "application/json"
    return resp
