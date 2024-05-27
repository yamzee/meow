; cat out the contents of a file
global _start          ;expose entrypoint

section .text          ;start code
maxRead equ 1024

_start:
;open the file, move to mem
mov rax, 2             ;open file, move to rax
mov rdi, [rsp+16]      ;pass into rdi, rsp (16 is args) rsp has arg count
mov rsi, 0             ;0 to flags
mov rdx, 0             ;0 to mode
syscall

;take file, push to stdout
mov rsi, rax           ;pass opened file descriptor to input
mov rdi, 1             ;output to rdi, 1 is stdout
mov rax, 40            ;sendfile = 40
mov rdx, 0             ;no offset
mov r10, maxRead       ;count, max filesize
syscall

;exiting
mov rax, 60            ;exit syscall
mov rdi, 0             ;return 0
syscall
