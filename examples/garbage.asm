; this is a test program
y = $ff
.byte "AB", 67, y
start:
.db "DE"
.dw "AB", $ffff, 'A', 'A'+3, y
LDA #$ff01
.db "HELLO WORLD"