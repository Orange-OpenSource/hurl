import uuid
from http import HTTPStatus

from app import app
from flask import Response

jobs = {}


@app.route("/jobs", methods=["POST"])
def new_job():
    job_id = uuid.uuid4().hex
    job = {"id": job_id, "state": "RUNNING", "count": 0}
    jobs[job_id] = job

    data = {"id": job_id, "state": "RUNNING"}
    return data, HTTPStatus.CREATED


@app.route("/jobs/<job_id>")
def get_job(job_id):
    job = jobs.get(job_id)
    if not job:
        data = {"error": "404", "message": "job not found"}
        return data, HTTPStatus.NOT_FOUND

    if job["state"] == "RUNNING":
        job["count"] = job["count"] + 1
        if job["count"] >= 5:
            job["state"] = "COMPLETED"

    data = {"id": job_id, "state": job["state"]}
    return data


@app.route("/jobs/<job_id>", methods=["DELETE"])
def delete_job(job_id):
    jobs.pop(job_id)
    return Response(status=HTTPStatus.OK, mimetype="application/json")
