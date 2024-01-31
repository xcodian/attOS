global start

section .text
bits 32
%include "src/init/util.asm"

start:
  ; zero data segment
  xor ax, ax
  mov ds, ax

  ; print hello world
  mov esi, s_hello
  mov eax, 1
  call print

  

_guard:
  mov esi, s_halting
  mov eax, 2
  call print

  hlt

; strings
s_hello db 'atos: hello from main.asm', 0x00
s_halting db 'atos: halting', 0x00