global start32

section .text
bits 32
extern start64
%include "src/init/util.asm"

start32:
  ; set up stack
  mov esp, stack_top

  call check_cpuid
  call check_long_mode

  mov esi, s_long_mode_avail
  mov eax, 1
  call print

  ; create and load page tables
  call setup_paging
  
  ; jump into 64 bit code
  lgdt [gdt64.pointer]
  jmp gdt64.code_segment:start64

  jmp _guard

check_cpuid:
  pushfd          ; push FLAGS onto stack
  pop eax         ; load FLAGS into eax
  mov ecx, eax    ; copy to ecx
  
  ; try setting FLAGS
  xor eax, 1 << 21
  push eax
  popfd

  pushfd
  pop eax

  xor eax, ecx
  jz .err_no_cpuid
  ret
.err_no_cpuid:
  mov esi, s_err_no_cpuid
  mov eax, 1
  call print
  hlt

check_long_mode:
  mov eax, 0x80000000
  cpuid 
  cmp eax, 0x80000001 
  jb .err_no_long_mode

  mov eax, 0x80000001
  cpuid
  test edx, 1 << 29
  jz .err_no_long_mode
  ret
.err_no_long_mode:
  mov esi, s_err_no_long_mode
  mov eax, 1
  call print
  hlt

setup_paging:
  ; add a reference to the L3 table in the L4 table
  mov eax, page_table_l3
  or eax, 0b11
  mov [page_table_l4], eax
  ; add a reference to the L2 table in the L3 table
  mov eax, page_table_l2
  or eax, 0b11
  mov [page_table_l3], eax

  ; make a bunch of huge pages
  mov ecx, 0
  .loop:
    ; calculate entry address
    mov eax, 0x200000 ; 2Mib
    mul ecx

    ; set entry flags
    or eax, 0b10000011 ; present, writeable, huge page

    ; add entry into L2 table
    mov [page_table_l2 + ecx * 8], eax

    inc ecx
    cmp ecx, 512 ; check if whole L2 table is mapped
    jne .loop
  
  ; point the cpu to L4 page table
  mov eax, page_table_l4
  mov cr3, eax

  ; enable PAE
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax

  ; trigger long mode switch
  mov ecx, 0xc0000080
  rdmsr
  or eax, 1 << 8
  wrmsr

  ; enable paging
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax

  ret

_guard:
  mov esi, s_halting
  mov eax, 2
  call print
  hlt

section .bss
align 4096
page_table_l4:
  resb 4096
page_table_l3:
  resb 4096
page_table_l2:
  resb 4096
stack_bottom:
  resb 4096 * 4
stack_top:

section .data
s_long_mode_avail   db '[ ok ] 64-bit mode is supported by CPU', 0x00
s_halting           db 'main32: halting', 0x00
s_err_no_cpuid      db '[ !! ] cpuid is unsupported on this processor', 0x00
s_err_no_long_mode  db '[ !! ] 64-bit long mode is unsupported on this processor', 0x00

section .rodata
gdt64:
  ; zero entry
  dq 0
.code_segment: equ $ - gdt64
  dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
  dw $ - gdt64 - 1
  dq gdt64