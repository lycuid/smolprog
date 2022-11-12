(define-module (logger date)
               #:use-module ((ice-9 format) #:select (format))
               #:use-module (srfi srfi-19))

(define-public interval 1)
(define-public runner
  (λ ()
     (format #f
       "<Box:Top|Bottom|Left|Right=#089CAC>~
          <Bg=#171717>~
            <Fn=1> ~a </Fn>~
          </Bg>~
        </Box>"
       (date->string (current-date) "~a, ~b ~d ~H:~M:~S"))))
