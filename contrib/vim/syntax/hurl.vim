" Vim syntax file
" Language: Hurl (https://hurl.dev)

if exists("b:current_syntax")
  finish
endif

syntax keyword method GET POST PUT DELETE CONNECT OPTIONS TRACE PATCH
syntax keyword operators == != > >= <= sp lt
syntax match http "^HTTP/*"
syntax keyword hurlKeywords jsonpath xpath null header count

syntax match comment "#.*$"
syntax match escapeNumberSign  "\\#"
syntax match escapeQuote  "\\\""
syntax match section "\[[A-Za-z]*\]"
syntax match number  "\s[0-9]*"

syntax region string   start='"' end='"'  contains=escapeQuote
syntax region string   start='```' end='```'
syntax include @jsonSyntax syntax/json.vim
syntax region json     start='{' end='}' contains=@jsonSyntax contained
syntax region template start='{{' end='}}'


hi def link comment Comment
hi def link method Keyword
hi def link http Keyword
hi def link section Label
hi def link operators Operator
hi def link string String
hi def link number Number
hi def link hurlKeywords Keyword
hi def link template Identifier
hi def link escapeQuote SpecialChar
hi def link escapeNumberSign SpecialChar

let b:current_syntax = "hurl"
