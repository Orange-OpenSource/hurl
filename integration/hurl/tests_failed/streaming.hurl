# With a streaming endpoint,
# Hurl is hanging without outputting anything
#
# curl outputs when it received data
# $ curl http://localhost:8000/streaming
# 0
# 1
# 2
# 3
# ....
#
# The max-time option must be used to make Hurl exit.
GET http://localhost:8000/streaming

