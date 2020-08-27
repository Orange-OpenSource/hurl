#!/bin/bash
./gen_manpage.py hurl.md > hurl.1
./gen_manpage.py hurlfmt.md > hurlfmt.1
./gen_doc.py hurl.md > ../../hurl-dev/sites/hurl.dev/_docs/man-page.md


