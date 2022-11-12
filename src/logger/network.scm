(define-module (logger network)
               #:use-module ((ice-9 match)  #:select (match))
               #:use-module ((ice-9 format) #:select (format))
               #:use-module ((utils)        #:select (first-line-of call-and-set! set-all!)))

(define-public interval 1)
(define-public runner
  (λ ()
     (format #f
       "<BtnL=xdotool key super+ctrl+n> ~a  </BtnL><Box:Left=#171717:2> </Box>"
       (calculate))))

(define *net-dir* "/sys/class/net")
(define *old-interface* "")
(define *old-rx* 0)
(define *old-tx* 0)

(define calculate
  (λ ()
     (let ([interface (get-active-interface)])
       (if (string-null? interface)
         "net: ?" ;; internet not connected.
         (let ([interface-changed? (not (string=? *old-interface* interface))])
           (match (get-network-bytes interface)
                  ((new-rx new-tx)
                   (begin
                     (when interface-changed?
                       (set-all! (*old-rx* new-rx) (*old-tx* new-tx) (*old-interface* interface)))
                     (let* ([proc (λ (new old) (/ (- new old) 1024.))]
                            [rx   (call-and-set! proc (new-rx -> *old-rx*))]
                            [tx   (call-and-set! proc (new-tx -> *old-tx*))])
                       (format #f "~a:  ~,2f kib/s  ~,2f kib/s" interface rx tx))))))))))

(define get-active-interface
  (λ ()
     (let* ([dir     (opendir *net-dir*)]
            [return  (λ (val) (begin (closedir dir) val))])
       (let next-entry ([entry (readdir dir)])
         (if (eof-object? entry)
           (return "")
           (let ([file (format #f "~a/~a/operstate" *net-dir* entry)])
             (if (and (file-exists? file) (string=? "up" (first-line-of file)))
               (return entry)
               (next-entry (readdir dir)))))))))

(define get-network-bytes
  (λ (interface) ;; assuming the 'interface' is valid (no unnecessary strcmp).
     (let ([rx-file (format #f "~a/~a/statistics/rx_bytes" *net-dir* interface)]
           [tx-file (format #f "~a/~a/statistics/tx_bytes" *net-dir* interface)])
       (list
         (string->number (first-line-of rx-file))
         (string->number (first-line-of tx-file))))))
