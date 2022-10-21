(define-module (logger memory)
               #:use-module ((ice-9 format) #:select (format))
               #:use-module ((ice-9 popen)  #:select (open-input-pipe close-pipe))
               #:use-module ((ice-9 match)  #:select (match))
               #:use-module ((ice-9 rdelim) #:select (read-line))
               #:use-module ((utils)        #:select (in-range? starts-with? words)))

(define-public interval 1)
(define-public runner
  (λ ()
     (format #f
       "<BtnL=notify_max_mem> ~a  </BtnL><Box:Left=#171717:2> </Box>"
       (calculate))))

(define calculate
  (λ ()
     (match (get-memory-usage)
            ((_ used _ shared _ _)
             (let ([mem (quotient (+ used shared) 1024)])
               (cond
                 [(in-range? mem 1 500)       (format #f "  ~4d MiB" mem)]
                 [(in-range? mem 501 1000)    (format #f "  <Fg=#ffdd59>~4d</Fg> MiB" mem)]
                 [(in-range? mem 1001 +inf.0) (format #f "  <Fg=#cc6666>~,2f</Fg> GiB" (/ mem 1024.))]
                 [#t                          "  ?"]))))))

(define get-memory-usage
  (λ ()
     (let* ([pipe    (open-input-pipe "free")]
            [return  (λ (val) (begin (close-pipe pipe) val))])
       (let get-line ([line (read-line pipe)])
         (cond
           [(eof-object? line)        (return "")]
           [(starts-with? line "Mem") (return (map string->number (cdr (words line))))]
           [#t                        (get-line (read-line pipe))])))))
