header_start:
  ; multiboot 2 header
  dd 0xe85250d6
  ; architecture = protected i386
  dd 0
  ; header length
  dd header_end - header_start
  ; checksum
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
  dw 0
  dw 0
  dd 8
header_end: