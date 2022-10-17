(define-module (logger sessions)
               #:use-module ((ice-9 popen)  #:select (open-input-pipe close-pipe))
               #:use-module ((ice-9 rdelim) #:select (read-line)))

(define-public interval 1)
(define-public runner
  (λ ()
     (format #f
       "<BtnL=notify_tmux_ls> ~
          <Fg=#9b59b6>  ~a</Fg>  ~
        </BtnL>~
        <Box:Left=#171717:2> </Box>"
       (calculate))))

(define calculate
  (λ ()
     (let ([port (open-input-pipe "tmux ls")])
       (let get-line ([line (read-line port)])
         (if (eof-object? line)
           (begin (close-pipe port) 0)
           (+ 1 (get-line (read-line port))))))))
