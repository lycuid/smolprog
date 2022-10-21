#!/bin/sh
exec guile -L $(dirname $0) -e "(smolprog)" $0 $@
!#

(define-module (smolprog))

(use-modules ((ice-9 atomic))
             ((ice-9 threads))
             ((srfi srfi-43)  #:select (vector-map))
             ((srfi srfi-1)   #:select (fold-right))
             ((srfi srfi-18)  #:select (time->seconds seconds->time current-time thread-sleep!))
             ((utils)         #:select (first-line-of)))

(use-modules ((logger network)    #:prefix net:)
             ((logger cpu)        #:prefix cpu:)
             ((logger memory)     #:prefix mem:)
             ((logger volume)     #:prefix vol:)
             ((logger sessions)   #:prefix ses:)
             ((logger battery)    #:prefix bat:)
             ((logger date)       #:prefix dat:))

(define *procs*
  (vector (λ (i) (interval-runner  net:interval net:runner i))
          (λ (i) (interval-runner  cpu:interval cpu:runner i))
          (λ (i) (interval-runner  mem:interval mem:runner i))
          (λ (i) (fifo-runner      vol:fifo-file vol:default vol:fmt i))
          (λ (i) (interval-runner  ses:interval ses:runner i))
          (λ (i) (interval-runner  bat:interval bat:runner i))
          (λ (i) (interval-runner  dat:interval dat:runner i))))

(define *condvar* (make-condition-variable))
(define *mutex*   (make-mutex))
(define *running* (make-atomic-box #t))
(define *output*  (vector-map (const "") *procs*))

(define-syntax-rule (while-running body)
  (do ((loop #nil #nil))
    ((not (atomic-box-ref *running*))) body))

(define interval-runner
  (λ (interval runner index)
     (while-running
       (let ([time (seconds->time (+ interval (time->seconds (current-time))))])
         (with-mutex *mutex*
                     (let ([new (runner)]
                           [old (vector-ref *output* index)])
                       (unless (string=? new old)
                         (vector-set! *output* index new)
                         (signal-condition-variable *condvar*))))
         (thread-sleep! time)))))

(define fifo-runner
  (λ (fifo-file default fmt index)
     (begin
       (with-mutex *mutex*
                   (vector-set! *output* index (format #f fmt default))
                   (signal-condition-variable *condvar*))
       (while-running
         (let ([new (first-line-of fifo-file)])
           (with-mutex *mutex*
                       (let ([old (vector-ref *output* index)])
                         (unless (or (eof-object? new) (string=? new old))
                           (vector-set! *output* index (format #f fmt new))
                           (signal-condition-variable *condvar*)))))))))

(define-public (main ...)
  (let ([threads (vector-map (λ (i proc) (make-thread proc i)) *procs*)])
    (begin
      (while-running
        (with-mutex *mutex*
                    (begin
                      (wait-condition-variable *condvar* *mutex*)
                      (format (current-output-port) "~a~%~!"
                              (fold-right string-append "" (vector->list *output*))))))
      (map join-thread threads))))
