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
cmp byte [rsi],0
je .end_1
.while_1:
inc rsi
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
cmp byte [rsi],0
je .end_2
.while_2:
inc rsi
inc byte [rsi]
inc byte [rsi]
inc rsi
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc rsi
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
inc rsi
inc byte [rsi]
dec rsi
dec rsi
dec rsi
dec rsi
dec byte [rsi]
cmp byte [rsi],0
jne .while_2
.end_2:
inc rsi
inc byte [rsi]
inc rsi
inc byte [rsi]
inc rsi
dec byte [rsi]
inc rsi
inc rsi
inc byte [rsi]
cmp byte [rsi],0
je .end_3
.while_3:
dec rsi
cmp byte [rsi],0
jne .while_3
.end_3:
dec rsi
dec byte [rsi]
cmp byte [rsi],0
jne .while_1
.end_1:
inc rsi
inc rsi
mov rdx,1
mov rdi,1
mov rax,1
syscall
inc rsi
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
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
mov rdx,1
mov rdi,1
mov rax,1
syscall
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
inc rsi
inc rsi
mov rdx,1
mov rdi,1
mov rax,1
syscall
dec rsi
dec byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
dec rsi
mov rdx,1
mov rdi,1
mov rax,1
syscall
inc byte [rsi]
inc byte [rsi]
inc byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
dec byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
inc rsi
inc rsi
inc byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
inc rsi
inc byte [rsi]
inc byte [rsi]
mov rdx,1
mov rdi,1
mov rax,1
syscall
mov rdi,0
mov rax,60
syscall
