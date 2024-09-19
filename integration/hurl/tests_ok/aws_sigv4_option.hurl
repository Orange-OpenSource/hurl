POST http://localhost:8000/aws-sigv4
[Options]
aws-sigv4: aws:amz:eu-central-1:hurltest
user: someAccessKeyId:someSecretKey
[FormParams]
test: test
HTTP 200


POST http://localhost:8000/aws-sigv4
[Options]
aws-sigv4: aws:amz:eu-central-1:hurltest
user: someAccessKeyId:someSecretKey
[Form]
test: test
HTTP 200
