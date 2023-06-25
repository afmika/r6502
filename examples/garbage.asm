; this is a test program
y = $ff
.byte "AB", 67, y
start:
.db "DE"
.dw "AB", $ffff, 'A', 'A'+3, y
; LDA #$a ; non official
LDA ($ff), y ; official
.db "HELLO WORLD"