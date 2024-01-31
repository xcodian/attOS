ENTRY(start32)

SECTIONS {
  . = 1M;

  .boot : 
  {
    KEEP(*(.multiboot_header))
  }

  .text : 
  {
    *(.text)
  }

  .bss : 
  {
    *(.bss)
  }

  .data : 
  {
    *(.data)
  }

  .rodata : 
  {
    *(.rodata)
  }
}