# Create a new job
POST http://localhost:8000/jobs
HTTP 201
[Captures]
job_id: jsonpath "$.id"
[Asserts]
jsonpath "$.state" == "RUNNING"


# Pull job status until it is completed
GET http://localhost:8000/jobs/{{job_id}}
[Options]
retry: 10
retry-interval: 100ms
HTTP 200
[Asserts]
jsonpath "$.state" == "COMPLETED"


# Delete the job
DELETE http://localhost:8000/jobs/{{job_id}}
HTTP 200


GET http://localhost:8000/jobs/{{job_id}}
HTTP 404
