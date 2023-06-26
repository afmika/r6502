.segment "HEADER"
.byte "SOME HEADER TEST NESROM"

.segment "CODE"
; this is a test program
y = $ff
.byte "AB", 67, y
start:
.db "DE"
.dw "AB", $ffff, 'A', 'A'+3, y
; LDA #$a ; non official
LDA ($ff), y ; official
.db "HELLO WORLD"

.segment "VECTORS"
.byte "SOME VECTORS"

.segment "CHARS"
.byte "SOME CHARS"
; .res 1234s