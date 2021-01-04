section .data
memory times 30000 db 0
section .text
global _start
_start:
mov rsi,memory
mov rdi,0
mov rax,60
syscall