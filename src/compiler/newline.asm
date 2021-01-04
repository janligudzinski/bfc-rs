section .data
memory times 30000 db 0
section .text
global _start
_start:
mov rsi,memory
inc byte [rsi]
inc byte [rsi]
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
syscall
mov rdi,0
mov rax,60
syscall