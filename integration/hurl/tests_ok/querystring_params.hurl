GET http://localhost:8000/querystring-params
[QueryStringParams]
param1: value1
param2:
param3: a=b
param4: 1,2,3
$top: 5
HTTP 200


GET http://localhost:8000/querystring-params
[Query]
param1: value1
param2:
param3: a=b
param4: 1,2,3
$top: 5
HTTP 200


# Same version as raw
GET http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3db&param4=1,2,3&$top=5
HTTP 200


# Combine version
GET http://localhost:8000/querystring-params?param1=value1
[QueryStringParams]
param2:
param3: a=b
param4: 1,2,3
$top: 5
HTTP 200


GET http://localhost:8000/querystring-params?param1=value1
[Query]
param2:
param3: a=b
param4: 1,2,3
$top: 5
HTTP 200


# Encoding slash
GET http://localhost:8000/querystring-params-encoded?value1=/&value2=%2F
[QueryStringParams]
value3: /
HTTP 200


GET http://localhost:8000/querystring-params-encoded?value1=/&value2=%2F
[Query]
value3: /
HTTP 200
