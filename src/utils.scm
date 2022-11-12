(define-module (utils)
               #:use-module ((srfi srfi-1)  #:select  (remove))
               #:use-module ((ice-9 ports)  #:select  (call-with-input-file))
               #:use-module ((ice-9 rdelim) #:select  (read-line))
               #:export     (call-and-set! set-all!))

(define-public words
  (λ (xs)
     (remove string-null? (string-split xs #\space))))

(define-public first-line-of
  (λ (file-path)
     (call-with-input-file file-path read-line)))

(define-public starts-with?
  (λ (str sub)
     (string=? sub (substring/read-only str 0 (string-length sub)))))

(define-public in-range?
  (λ (x lower upper)
     (and (>= x lower) (<= x upper))))

(define-syntax call-and-set!
  (syntax-rules (->)
    ((_ proc (new -> old))
     (let ([value (proc new old)])
       (begin (set! old new) value)))))

(define-syntax set-all!
  (syntax-rules ()
    ((_ (old new))
     (set! old new))
    ((_ (old new) expr ...)
     (begin (set-all! (old new)) (set-all! expr ...)))))
