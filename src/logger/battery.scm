(define-module (logger battery)
               #:use-module ((srfi srfi-43)) ;; vector lib.
               #:use-module ((ice-9 format) #:select (format))
               #:use-module ((utils)        #:select (first-line-of call-and-set!)))

(define-public interval 1)
(define-public runner
  (λ ()
     (let ([capacity (get-battery-capacity)])
       (format #f " ~a ~3d  " (get-battery-status capacity) capacity))))

(define *battery-symbols* #(" " " " " " " " " "))
(define *current* 0)

(define get-battery-capacity
  (λ ()
     (string->number
       (first-line-of "/sys/class/power_supply/BAT0/capacity"))))

(define get-battery-status
  (λ (capacity)
     (let* ([status   (first-line-of "/sys/class/power_supply/BAT0/status")]
            [sym-len  (vector-length *battery-symbols*)]
            [index    (call-and-set! values ((remainder (+ 1 *current*) sym-len) -> *current*))])
       (cond
         [(string=? "Charging"    status) (vector-ref *battery-symbols* index)]
         [(string=? "Discharging" status) (vector-ref *battery-symbols*
                                                      (quotient (* capacity sym-len) 100))]
         [#t                              (vector-ref *battery-symbols* (- sym-len 1))]))))
