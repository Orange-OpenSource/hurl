POST http://localhost:8000/verbosity
X_POWERED_BY: Hurl
{"foo": "bar"}
HTTP 200


POST http://localhost:8000/verbosity
X_POWERED_BY: Hurl
[Options]
verbosity: brief
{"foo": "bar"}
HTTP 200


POST http://localhost:8000/verbosity
X_POWERED_BY: Hurl
[Options]
verbosity: verbose
{"foo": "bar"}
HTTP 200


POST http://localhost:8000/verbosity
X_POWERED_BY: Hurl
[Options]
verbosity: debug
{"foo": "bar"}
HTTP 200
