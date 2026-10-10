; Founder-4 public ancestor. SPEC §16 (original §57).
; Survival, local gathering, day/night waiting and self-copy only.
; Every constant and loop here is ordinary genome content and may mutate.
;
; registers: r0..r7 effect/READ arguments, r8..r11 temporaries,
;            r12 = constant 1, r13 = wait counter (SPEC D7), r14/r15 = last receipt
.entry 0
.point 64
.structural 2048
.weights 4,2,2,2,1,1,1,1

; work memory
.equ M_CURSOR 1
.equ M_RAWLEN 2
.equ M_PAGES 3
.equ M_IDX 4
.equ M_DIR 5
.equ M_REPAIR 6
.equ M_KIND 8
.equ M_LIGHT 9
.equ M_TARGET 10
.equ M_DONE 11
.equ M_WAITED 12
; position table mem[16 + 2i] = dx, mem[17 + 2i] = dy for i = self, front, right, back, left

; pending receipt kinds
.equ K_GATHER 1
.equ K_COPY 2
.equ K_ALLOC 3
.equ K_RESIZE 4
.equ K_FINISH 5
.equ K_STEP 6
.equ K_REOPEN 7
.equ K_CANCEL 8
.equ K_ENERGY 9
.equ K_RELEASE 10
.equ K_DRIVE 11

.equ OUT_ENERGY 5
.equ FAULT_QUERYLIMIT 5

.module 0
init:
    CONST r12, 1
    CONST r9, -1
    CONST r8, 19          ; front dy = -1
    STORE r9, r8
    CONST r8, 20          ; right dx = 1
    STORE r12, r8
    CONST r8, 23          ; back dy = 1
    STORE r12, r8
    CONST r8, 24          ; left dx = -1
    STORE r9, r8
    CONST r8, M_WAITED    ; check light on the first pass
    STORE r12, r8

top:
    BRNZ r13, wait_tick
    ; row 0: unconsumed receipt
    CONST r8, M_KIND
    LOAD r9, r8
    BRNZ r9, receipt

rows:
    ; row 2: light check every 32 ticks or right after a wait
    CONST r8, M_WAITED
    LOAD r9, r8
    BRNZ r9, do_light
    CONST r0, 0
    CONST r3, 2           ; Self.age_lo
    READ r9
    CONST r10, 31
    AND r9, r10
    BRNZ r9, row3
do_light:
    CONST r9, 0
    CONST r8, M_WAITED
    STORE r9, r8
    CONST r0, 11          ; Surface
    CONST r1, 0
    CONST r2, 0
    CONST r3, 4           ; current_light
    READ r9
    CONST r8, M_LIGHT
    STORE r9, r8
    CONST r10, 8192
    LTU r9, r10
    BRNZ r9, wait32

row3:
    ; row 3: energy < 4096 -> wait 8
    CONST r0, 0
    CONST r3, 0
    READ r9
    CONST r10, 4096
    LTU r9, r10
    BRNZ r9, wait8

    ; row 4: inventory G < 24 -> gather G
    CONST r3, 1
    CONST r4, 3
    READ r9
    CONST r10, 24
    LTU r9, r10
    BRNZ r9, gather_g

    CONST r3, 6           ; Self.embryo_state
    READ r11
    BRNZ r11, has_embryo

    ; row 5: no embryo and S < 2 -> gather S
    CONST r3, 1
    CONST r4, 7
    READ r9
    CONST r10, 2
    LTU r9, r10
    BRNZ r9, gather_s

    ; row 6: E >= 49152 and light >= 8192 -> Allocate
    CONST r3, 0
    READ r9
    CONST r10, 49152
    LTU r9, r10
    BRNZ r9, row15
    CONST r8, M_LIGHT
    LOAD r9, r8
    CONST r10, 8192
    LTU r9, r10
    BRNZ r9, row15
    CONST r9, K_ALLOC
    CONST r8, M_KIND
    STORE r9, r8
    REPRO 0
    JUMP top

has_embryo:
    ; raw length and pages from Self.inherited_len
    CONST r3, 7
    READ r9
    CONST r8, M_RAWLEN
    STORE r9, r8
    CONST r10, 127
    ADD r9, r10
    CONST r10, 7
    SHR r9, r10
    CONST r8, M_PAGES
    STORE r9, r8
    MOV r10, r11
    SUB r10, r12
    BRNZ r10, not_writing
    ; row 7: Writing and capacity < 128 * pages -> Resize
    CONST r0, 8
    CONST r3, 2
    READ r10
    MOV r8, r9
    CONST r4, 7
    SHL r8, r4
    LTU r10, r8
    BRNZ r10, do_resize
    ; row 8: cursor < raw length -> COPY
    CONST r8, M_CURSOR
    LOAD r10, r8
    CONST r8, M_RAWLEN
    LOAD r9, r8
    MOV r8, r10
    LTU r8, r9
    BRNZ r8, do_copy
    ; row 9: Finish(raw length)
    MOV r5, r9
    CONST r9, K_FINISH
    CONST r8, M_KIND
    STORE r9, r8
    REPRO 2
    JUMP top

do_resize:
    MOV r5, r9
    CONST r9, K_RESIZE
    CONST r8, M_KIND
    STORE r9, r8
    REPRO 1
    JUMP top

do_copy:
    MOV r6, r9
    SUB r6, r10
    CONST r8, 4
    MOV r11, r8
    LTU r11, r6
    BRNZ r11, copy4
    JUMP copy_go
copy4:
    MOV r6, r8
copy_go:
    CONST r0, 6           ; OwnCode, offset 0
    CONST r1, 8           ; Embryo, offset 0
    MOV r4, r10
    MOV r5, r10
    CONST r7, 0
    CONST r3, 0
    CONST r9, K_COPY
    CONST r8, M_KIND
    STORE r9, r8
    COPY 0
    JUMP top

not_writing:
    SUB r10, r12
    BRNZ r10, not_sealed
    ; row 10: SealedWork -> Step
    CONST r9, K_STEP
    CONST r8, M_KIND
    STORE r9, r8
    REPRO 3
    JUMP top

not_sealed:
    SUB r10, r12
    BRNZ r10, is_ready
    ; rows 11-12: Invalid -> Reopen once, then Cancel
    CONST r8, M_REPAIR
    LOAD r9, r8
    BRNZ r9, do_cancel
    CONST r9, K_REOPEN
    CONST r8, M_KIND
    STORE r9, r8
    REPRO 4
    JUMP top
do_cancel:
    CONST r9, K_CANCEL
    CONST r8, M_KIND
    STORE r9, r8
    REPRO 6
    JUMP top

is_ready:
    CONST r0, 8
    CONST r3, 4           ; embryo energy
    READ r9
    CONST r10, 16384
    MOV r11, r9
    LTU r11, r10
    BRNZ r11, invest
    ; row 14: Release toward the remembered direction
    CONST r8, M_DIR
    LOAD r5, r8
    CONST r9, K_RELEASE
    CONST r8, M_KIND
    STORE r9, r8
    REPRO 5
    JUMP top

invest:
    ; row 13: E >= 32768 -> transfer min(1024, deficit)
    SUB r10, r9
    CONST r0, 0
    CONST r3, 0
    READ r9
    CONST r11, 32768
    LTU r9, r11
    BRNZ r9, row15
    CONST r5, 1024
    MOV r11, r10
    LTU r11, r5
    BRNZ r11, inv_small
    JUMP inv_go
inv_small:
    MOV r5, r10
inv_go:
    CONST r0, 0           ; Self
    CONST r1, 8           ; Embryo
    CONST r3, 15          ; energy channel
    CONST r9, K_ENERGY
    CONST r8, M_KIND
    STORE r9, r8
    TRANSFER 0
    JUMP top

; ---- rows 4-5: local gathering over self, front, right, back, left ----
gather_g:
    CONST r10, 3
    CONST r11, 4
    JUMP gather
gather_s:
    CONST r10, 7
    CONST r11, 2
gather:
    CONST r8, M_TARGET
    LOAD r9, r8
    XOR r9, r10
    BRNZ r9, gather_reset
gather_check:
    CONST r8, M_DONE
    LOAD r9, r8
    BRNZ r9, row15
gather_loop:
    CONST r8, M_IDX
    LOAD r9, r8
    CONST r8, 5
    EQ r8, r9
    BRNZ r8, gather_exhausted
    MOV r8, r9
    ADD r8, r9
    CONST r7, 16
    ADD r8, r7
    LOAD r1, r8
    ADD r8, r12
    LOAD r2, r8
    CONST r0, 1           ; Loose
    CONST r3, 0
    MOV r4, r10
    READ r8
    BRNZ r8, gather_take
    CONST r0, 0
    CONST r3, 5           ; Self.fault (read and clear)
    READ r7
    CONST r6, FAULT_QUERYLIMIT
    EQ r6, r7
    BRNZ r6, gather_wait
    ADD r9, r12
    CONST r7, M_IDX
    STORE r9, r7
    JUMP gather_loop
gather_reset:
    CONST r8, M_TARGET
    STORE r10, r8
    CONST r9, 0
    CONST r8, M_IDX
    STORE r9, r8
    CONST r8, M_DONE
    STORE r9, r8
    JUMP gather_check
gather_exhausted:
    CONST r8, M_DONE
    STORE r12, r8
    JUMP row15
gather_take:
    MOV r5, r8
    MOV r7, r11
    LTU r7, r5
    BRNZ r7, take_max
    JUMP take_go
take_max:
    MOV r5, r11
take_go:
    MOV r0, r9
    CONST r7, 4
    SHL r0, r7
    OR r0, r12            ; Loose at offset idx
    CONST r1, 0           ; Self inventory
    MOV r3, r10
    CONST r9, K_GATHER
    CONST r8, M_KIND
    STORE r9, r8
    TRANSFER 0
    JUMP top
gather_wait:
    YIELD
    JUMP top

; ---- row 0: receipt handling ----
receipt:
    CONST r10, 0
    STORE r10, r8
    CONST r10, K_COPY
    EQ r10, r9
    BRNZ r10, rc_copy
    CONST r10, K_GATHER
    EQ r10, r9
    BRNZ r10, rc_gather
    CONST r10, K_DRIVE
    EQ r10, r9
    BRNZ r10, rc_reset_search
    CONST r10, K_ALLOC
    EQ r10, r9
    BRNZ r10, rc_alloc
    CONST r10, K_REOPEN
    EQ r10, r9
    BRNZ r10, rc_reopen
    CONST r10, K_RELEASE
    EQ r10, r9
    BRNZ r10, rc_release
rc_common:
    CONST r10, OUT_ENERGY
    EQ r10, r14
    BRNZ r10, wait8
    JUMP rows
rc_copy:
    CONST r8, M_CURSOR
    LOAD r10, r8
    ADD r10, r15
    STORE r10, r8
    JUMP rc_common
rc_gather:
    MOV r10, r14
    CONST r11, 2
    LTU r10, r11
    BRNZ r10, rc_reset_search
    CONST r8, M_IDX
    LOAD r10, r8
    ADD r10, r12
    STORE r10, r8
    JUMP rc_common
rc_reset_search:
    CONST r10, 0
    CONST r8, M_IDX
    STORE r10, r8
    CONST r8, M_DONE
    STORE r10, r8
    JUMP rc_common
rc_alloc:
    BRNZ r14, rc_common
    CONST r10, 0
    CONST r8, M_CURSOR
    STORE r10, r8
    CONST r8, M_REPAIR
    STORE r10, r8
    JUMP rc_common
rc_reopen:
    BRNZ r14, rc_common
    CONST r10, 0
    CONST r8, M_CURSOR
    STORE r10, r8
    CONST r8, M_REPAIR
    STORE r12, r8
    JUMP rc_common
rc_release:
    BRNZ r14, rc_release_fail
    CONST r10, 0
    CONST r8, M_CURSOR
    STORE r10, r8
    CONST r8, M_DIR
    STORE r10, r8
    CONST r8, M_REPAIR
    STORE r10, r8
    JUMP rc_common
rc_release_fail:
    CONST r8, M_DIR
    LOAD r10, r8
    ADD r10, r12
    CONST r11, 3
    AND r10, r11
    STORE r10, r8
    JUMP rc_common

; ---- rows 15-16: wander when local sources are exhausted, otherwise wait ----
row15:
    CONST r8, M_DONE
    LOAD r9, r8
    BRNZ r9, try_drive
    JUMP wait8
try_drive:
    CONST r0, 0
    CONST r3, 0
    READ r9
    CONST r10, 8192
    LTU r9, r10
    BRNZ r9, wait8_reset
    CONST r8, M_LIGHT
    LOAD r9, r8
    LTU r9, r10
    BRNZ r9, wait8_reset
    RANDOM r5
    CONST r10, 3
    AND r5, r10
    CONST r9, K_DRIVE
    CONST r8, M_KIND
    STORE r9, r8
    DRIVE 0
    JUMP top
wait8_reset:
    CONST r9, 0
    CONST r8, M_IDX
    STORE r9, r8
    CONST r8, M_DONE
    STORE r9, r8
wait8:
    CONST r13, 8
    JUMP wait_tick
wait32:
    CONST r13, 32
wait_tick:
    SUB r13, r12
    YIELD
    BRNZ r13, wait_tick
    CONST r8, M_WAITED
    STORE r12, r8
    JUMP top
