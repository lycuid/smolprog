(define-module (logger volume))

(define-public default "vol: ?")
(define-public fifo-path (string-append (getenv "XDG_RUNTIME_DIR") "/pipe/volume"))
(define-public fmt "~
<ScrlU:Shift=volume 5%+>~
  <ScrlD:Shift=volume 5%->~
    <ScrlU=volume 1%+>~
      <ScrlD=volume 1%->~
        <BtnL=volume toggle> ~a  </BtnL>~
      </ScrlD>~
    </ScrlU>~
  </ScrlD>~
</ScrlU>~
<Box:Left=#171717:2> </Box>~
")
