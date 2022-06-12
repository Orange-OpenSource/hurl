" Vim syntax file
" Language: Hurl (https://hurl.dev)

if exists("b:current_syntax")
  finish
endif

syntax match Method  "^GET"
syntax match Method  "^POST"
syntax match Method  "^PUT"
syntax match Method  "^DELETE"
syntax match Method  "^CONNECT"
syntax match Method  "^OPTIONS"
syntax match Method  "^TRACE"
syntax match Method  "^PATCH"

syntax match Comment "#.*$"
syntax match EscapeNumberSign  "\\#"
syntax match EscapeQuote  "\\\"" 
syntax match Section "\[[A-Za-z]*\]"
syntax match Number  "\s[0-9]*"

syntax region String   start='"' end='"'  contains=EscapeQuote
syntax region String   start='```' end='```'
syntax region Json     start='{' end='}' contains=Template
syntax region Template start='{{' end='}}'


" colors
highlight Comment             ctermfg=grey
highlight Method              ctermfg=yellow
highlight Section             ctermfg=magenta
highlight String              ctermfg=green
highlight Json                ctermfg=green
highlight Number              ctermfg=lightblue
highlight EscapeNumberSign    ctermfg=white
highlight EscapeQuote         ctermfg=green
highlight Template            ctermfg=red


