# An unnamed simple query
POST http://localhost:8000/graphql
```graphql
{
  allFilms {
    films {
      title
      director
      releaseDate
    }
  }
}
```
HTTP 200
[Asserts]
jsonpath "$.data.allFilms.films" count == 6
jsonpath "$.data.allFilms.films[0].title" == "A New Hope"
jsonpath "$.data.allFilms.films[0].director" == "George Lucas"
jsonpath "$.data.allFilms.films[0].releaseDate" == "1977-05-25"
jsonpath "$.data.allFilms.films[0].openingCrawl" not exists
jsonpath "$.data.allFilms.films[1].title" == "The Empire Strikes Back"
jsonpath "$.data.allFilms.films[2].title" == "Return of the Jedi"


# Equivalent syntax by posting a JSON body
POST http://localhost:8000/graphql
Content-Type: application/json
{"query":"{\n  allFilms {\n    films {\n      title\n      director\n      releaseDate\n    }\n  }\n}"}
HTTP 200
[Asserts]
jsonpath "$.data.allFilms.films" count == 6


# Full syntax for query
POST http://localhost:8000/graphql
```graphql
query Query {
  allFilms {
    films {
      title
      director
      releaseDate
    }
  }
}
```
HTTP 200
[Asserts]
jsonpath "$.data.allFilms.films" count == 6


# Query with variables:
POST http://localhost:8000/graphql
```graphql
query Person($id: ID!) {
  person(id: $id) {
    name
  }
}

variables {
  "id": "cGVvcGxlOjQ="
}
```
HTTP 200
[Asserts]
jsonpath "$.data.person.name" == "Darth Vader"


# Hurl variables can also be used:
POST http://localhost:8000/graphql
[Options]
variable: id=cGVvcGxlOjQ=
```graphql
query Person($id: ID!) {
  person(id: $id) {
    name
  }
}

variables {
  "id": "{{id}}"
}
```
HTTP 200
[Asserts]
jsonpath "$.data.person.name" == "Darth Vader"
