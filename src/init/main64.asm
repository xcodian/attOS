section .text
bits 64

global start64
extern kernel_main

%include "src/init/util.asm"

start64:
  mov ax, 0
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

  mov esi, s_hello64
  mov eax, 2
  call print
  
  jmp kernel_main

  hlt

section .data
s_hello64    db '[ ok ] in long mode, jmp kernel_main YOLOOOO', 0x00