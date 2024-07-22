#!/bin/bash
set -Eeuo pipefail

/usr/bin/expect <<EOD
spawn hurl --no-color --verbose --interactive tests_ok/interactive.hurl
expect -exact "\r
Interactive mode\r
\r
Next request:\r
\r
GET http://localhost:8000/hello?name=Alice\r
\r
Press Q (Quit) or C (Continue)\[?25l\r
"
send -- "c"
expect -exact "\r
Interactive mode\r
\r
Next request:\r
\r
GET http://localhost:8000/hello?name=Bob\r
\r
Press Q (Quit) or C (Continue)\[?25l\r
"
send -- "c"
expect eof
EOD