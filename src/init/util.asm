%ifndef UTIL_ASM
%define UTIL_ASM

;; function print
;; - esi: string ptr
;; - eax: line offset
;; does not require a stack to function
print:
  ; zero counter
  xor ecx, ecx
  ; set up line offset
  imul eax, 160
  mov edx, 0xb8000
  add edx, eax

  .loop:
    ; load next byte
    lodsb
    ; check if this is last char, if so, bailout
    cmp al, 0
    jne $+1
    ret
    ; write to video memory
    mov [edx + (ecx * 2)], al
    ; increment counter and loop 
    inc ecx
    jmp .loop

%endif