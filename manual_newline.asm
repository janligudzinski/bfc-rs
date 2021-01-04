; This is the manually-written ASM file that demonstrates what newline.bf should translate into.
section .data
memory times 30000 db 0
section .text
global _start
_start:
mov rsi,memory ; this is our data pointer

inc byte [rsi] ; increment the value found at the memory address pointed to by our pointer
; The "byte" is necessary, as we don't have a type system in assembly
; so the assembler doesn't know what's supposed to be at some memory address, that's our job
; This is why smart people invented higher-level programming languages.
inc byte [rsi] ; All these are our "+" commands.
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]

mov rdx,1
mov rdi,1
mov rax,1
syscall ; that was our "."

mov rdi,0
mov rax,60
syscall ;exit