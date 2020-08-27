GET http://localhost:8000/querystring-params
[QueryStringParams]
param1: value1
param2:
param3: a=b
param4: 1,2,3
HTTP/1.0 200


# same version as raw
GET http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3db&param4=1,2,3

HTTP/1.0 200


# combine version
GET http://localhost:8000/querystring-params?param1=value1
[QueryStringParams]
param2:
param3: a=b
param4: 1,2,3
HTTP/1.0 200


# encoding slash
GET http://localhost:8000/querystring-params-encoded?value1=/&value2=%2F
[QueryStringParams]
value3:/
HTTP/1.0 200
