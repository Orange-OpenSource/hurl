" Vim syntax file
" Language: Hurl (https://hurl.dev)

if exists("b:current_syntax")
  finish
endif

syntax keyword method GET POST PUT DELETE CONNECT OPTIONS TRACE PATCH LINK UNLINK PURGE LOCK UNLOCK PROPFIND VIEW nextgroup=url skipwhite
syntax match url "\S\+" contained
syntax match version "HTTP" nextgroup=status skipwhite
syntax match version "HTTP/1\.0" nextgroup=status skipwhite
syntax match version "HTTP/1\.1" nextgroup=status skipwhite
syntax match version "HTTP/2" nextgroup=status skipwhite
syntax match version "HTTP/\*" nextgroup=status skipwhite
syntax match status "[0-9]\+" contained
syntax match comment "#.*$"
syntax match section "\[QueryStringParams\]"
syntax match section "\[FormParams\]"
syntax match section "\[MultipartFormData\]"
syntax match section "\[Cookies\]"
syntax match section "\[Captures\]"
syntax match section "\[Asserts\]"
syntax match section "\[Options\]"

syntax keyword operator == != > >= < <= not
syntax keyword query status url header cookie body jsonpath xpath regex variable duration sha256 md5 bytes
syntax keyword predicate startsWith endsWith matches exists includes isInteger isFloat isBoolean isString isCollection
syntax match predicate "contains"
syntax keyword filter count regex urlEncode urlDecode htmlEscape htmlUnescape
syntax match escapeNumberSign "\\#"
syntax match escapeQuote "\\\""
syntax region string start='"' end='"'  contains=escapeQuote
syntax region string start='```' end='```'
syntax include @jsonSyntax syntax/json.vim
syntax region json start='{' end='}' contains=@jsonSyntax contained
syntax region template start='{{' end='}}'


highlight def link comment Comment
highlight def link method Statement
highlight def link url Underlined
highlight def link version Statement
highlight def link status Number
highlight def link section Statement
highlight def link operators Operator
highlight def link string String
highlight def link query Identifier
highlight def link filter Operator
highlight def link predicate Operator
highlight def link template Identifier
highlight def link escapeQuote SpecialChar
highlight def link escapeNumberSign SpecialChar

let b:current_syntax = "hurl"
