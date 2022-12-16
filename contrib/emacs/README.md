<div align="center">
  <h1>Emacs Hurl</h1>

Emacs major mode for Hurl.

</div>

---

## Features

- Keyword highlight

## Installation

### Doom Emacs

in `packages.el`

``` lisp
(package! hurl-mode :recipe
  (:host github
   :repo "Orange-OpenSource/hurl"
   :files ("contrib/emacs/*.el")))
```

### straight.el

``` lisp
(straight-use-package
 '(hurl-mode
   :type git :host github :repo "Orange-OpenSource/hurl" :files ("contrib/emacs/*.el")))

```
