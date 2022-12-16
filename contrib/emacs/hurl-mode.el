;;; hurl-mode.el --- Major mode for hurl  -*- lexical-binding: t; -*-

;; Homepage: https://github.com/Orange-OpenSource/hurl
;; Keywords: Hurl, shell

;; Package-Version: 0.1.0
;; Package-Requires: ((emacs "24"))

;; SPDX-License-Identifier: Apache License 2.0

;;; Commentary:

;; A very basic version of major mode for hurl shell.
;; Current features:
;;
;;  - keyword highlight
;;

;;; Code:

(require 'cl-lib)
(eval-when-compile (require 'subr-x))

(defgroup hurl nil
  "Hurl shell support."
  :group 'languages)

(defcustom hurl-indent-offset 4
  "Default indentation offset for Hurl."
  :group 'hurl
  :type 'integer
  :safe 'integerp)

(defvar hurl-enable-auto-indent nil
  "Controls auto-indent feature.
If the value of this variable is non-nil, whenever a word in
`hurl-auto-indent-trigger-keywords' is typed, it is indented instantly.")

(unless (fboundp 'setq-local)
  (defmacro setq-local (var val)
    "Set variable VAR to value VAL in current buffer."
    `(set (make-local-variable ',var) ,val)))

;;; Syntax highlighting
;;; To get the commands, use `help commands | get name | | save --raw tmp'
(defconst hurl-builtins
  (list
   "GET"
   "POST"
   "PUT"
   "DELETE"
   "CONNECT"
   "OPTIONS"
   "TRACE"
   "PATCH"
   "header"
   "status"
   "jsonpath"
   ))

(defconst hurl-keywords
  (list
   "exists"
   "contains"
   "not"
   "=="
   ))

;;; Add `hurl-builtin' and `hurl-keywords' to
;;; font-lock
(defconst hurl-font-lock-keywords-1
  (list

   ;; Builtins
   `( ,(rx-to-string `(and
                       symbol-start
                       (eval `(or ,@hurl-builtins))
                       symbol-end)
                     t)
      .
      font-lock-builtin-face)

   ;; Keywords
   `( ,(rx-to-string `(and
                       symbol-start
                       (eval `(or ,@hurl-keywords))
                       symbol-end)
                     t)
      .
      font-lock-keyword-face)))

(defvar hurl-mode-syntax-table
  (let ((table (make-syntax-table text-mode-syntax-table)))
    (modify-syntax-entry ?\# "<" table)
    (modify-syntax-entry ?\n ">" table)
    (modify-syntax-entry ?\" "\"\"" table)
    (modify-syntax-entry ?\' "\"'" table)
    (modify-syntax-entry ?\\ "\\" table)
    (modify-syntax-entry ?$ "'" table)
    table)
  "Syntax table for `hurl-mode'.")


;;; Mode definition

;;;###autoload
(define-derived-mode hurl-mode prog-mode "Hurl"
  "Major mode for editing hurl shell files."
  :syntax-table hurl-mode-syntax-table
  (setq-local font-lock-defaults '(hurl-font-lock-keywords-1))
  (setq-local comment-start "# ")
  (setq-local comment-start-skip "#+[\t ]*"))

;;;###autoload
;;; Specify major mode by file extension .hurl
(add-to-list 'auto-mode-alist '("\\.hurl\\'" . hurl-mode))

(provide 'hurl-mode)

;;; hurl-mode.el ends here
