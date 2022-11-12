(define-module (logger cpu)
               #:use-module ((srfi srfi-1)  #:prefix iter:)
               #:use-module ((ice-9 format) #:select (format))
               #:use-module ((utils)        #:select (first-line-of call-and-set! in-range? words)))

(define-public interval 1)
(define-public runner
  (λ ()
     (format #f
       "<BtnL=notify_max_cpu> ~a  </BtnL><Box:Left=#171717:2> </Box>"
       (calculate))))

(define *old-total* 0)
(define *old-used*  0)

(define calculate
  (λ ()
     (let* ([cpu (get-cpu-usage)]
            [fmt (cond
                   [(in-range? cpu 0 24)    "  ~3d%"]
                   [(in-range? cpu 25 64)   "  <Fg=#ffdd59>~3d</Fg>%"]
                   [(in-range? cpu 65 100)  "  <Fg=#cc6666>~3d</Fg>%"]
                   [#t                      "  ?"])])
       (format #f fmt cpu))))

(define get-cpu-usage
  (λ ()
     (let* ([line       (first-line-of "/proc/stat")]
            [vals       (cdr (words line))]
            [new-total  (apply + (map string->number (iter:take vals 7)))]
            [new-used   (apply + (map string->number (iter:take vals 3)))]
            [total      (call-and-set! - (new-total -> *old-total*))]
            [used       (call-and-set! - (new-used  -> *old-used*))])
       (quotient (* used 100) (max 1 total)))))
