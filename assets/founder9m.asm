; Founder-9m public ancestor for E2 with habits in modules (SPEC §16f). Module 0 is Founder-7
; with three hooks, the same in every modular founder:
;   CALL 1 after taking G (r11 = G that was on that tile),
;   CALL 2 when lost (about to wander or sleep for want of S or G): r5 = a direction, or 4,
;   CALL 3 when rich and idle outside the morning.
; Modules 1..3 hold this founder's habit: learning from a public record (3), as Founder-9. Empty hooks just return.
; Recombination (tag kept, content replaced) can then move one founder's habit into another's
; genome; a Founder-9m that adopted another kind's code carries that code's hooks as donors.
;
; registers: r0..r7 effect/READ arguments, r8..r11 temporaries,
;            r12 = constant 1, r13 = wait counter, r14/r15 = last receipt
.entry 0
.point 64
.structural 2048
.weights 4,2,2,2,1,1,1,1

.equ M_CURSOR 1
.equ M_RAWLEN 2
.equ M_DIR 5
.equ M_REPAIR 6
.equ M_LIGHT 9
.equ M_PREV 10
.equ M_IDX 11
.equ M_WAITED 12
.equ M_GIDX 13
.equ M_LEARNED 14
.equ M_LEN 15
.equ M_SRC 26
.equ M_SEL 27
; position table mem[16 + 2i] = dx, mem[17 + 2i] = dy for i = self, front, right, back, left

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
    CONST r8, M_WAITED
    STORE r12, r8

top:
    BRNZ r13, wait_tick
    ; light check every 32 ticks or right after a wait
    CONST r8, M_WAITED
    LOAD r9, r8
    BRNZ r9, do_light
    CONST r0, 0
    CONST r3, 2           ; Self.age_lo
    READ r9
    CONST r10, 31
    AND r9, r10
    BRNZ r9, after_light
do_light:
    CONST r9, 0
    CONST r8, M_WAITED
    STORE r9, r8
    CONST r8, M_LIGHT
    LOAD r10, r8          ; previous reading
    CONST r0, 11          ; Surface
    CONST r1, 0
    CONST r2, 0
    CONST r3, 4           ; current_light
    READ r9
    STORE r9, r8
    CONST r8, M_PREV
    STORE r10, r8
    CONST r10, 8192
    LTU r9, r10
    BRNZ r9, wait32

after_light:
    CONST r0, 0
    CONST r3, 0           ; energy
    READ r9
    CONST r10, 8192
    LTU r9, r10
    BRNZ r9, wait32
    CONST r3, 6           ; embryo state
    READ r11
    BRNZ r11, embryo
    CONST r3, 1
    CONST r4, 7           ; inventory S
    READ r9
    CONST r10, 2
    LTU r9, r10
    BRNZ r9, gather_s
    CONST r4, 3           ; inventory G
    READ r9
    CONST r10, 30
    LTU r9, r10
    BRNZ r9, gather_g
    CONST r3, 0
    READ r9
    CONST r10, 50000
    LTU r9, r10
    BRNZ r9, wait32
    ; morning only: the last light reading is higher than the one before
    CONST r8, M_LIGHT
    LOAD r9, r8
    CONST r8, M_PREV
    LOAD r10, r8
    LTU r10, r9
    BRNZ r10, allocate
    CALL 3                ; hook: rich and idle
    JUMP wait32
allocate:
    REPRO 0
    BRNZ r14, wait32
    CONST r9, 0
    CONST r8, M_CURSOR
    STORE r9, r8
    CONST r8, M_REPAIR
    STORE r9, r8
    JUMP top

embryo:
    CONST r3, 7           ; inherited_len
    READ r10
    CONST r8, M_RAWLEN
    STORE r10, r8
    SUB r11, r12
    BRNZ r11, not_writing
    ; Writing: enough pages?
    MOV r9, r10
    CONST r8, 127
    ADD r9, r8
    CONST r8, 7
    SHR r9, r8
    CONST r0, 8
    CONST r3, 2           ; embryo capacity
    READ r11
    MOV r8, r9
    CONST r4, 7
    SHL r8, r4
    LTU r11, r8
    BRNZ r11, resize
    CONST r8, M_CURSOR
    LOAD r9, r8
    CONST r0, 6           ; OwnCode
    CONST r1, 8           ; Embryo
    CONST r3, 0
    CONST r6, 4           ; COPY copies the available prefix of 4
    CONST r7, 0
copy_loop:
    MOV r11, r9
    LTU r11, r10
    BRNZ r11, copy_go
    CONST r8, M_CURSOR
    STORE r9, r8
    MOV r5, r10
    REPRO 2               ; Finish(raw length)
    JUMP top
copy_go:
    MOV r4, r9
    MOV r5, r9
    COPY 0
    BRNZ r15, copy_ok
    CONST r8, M_CURSOR
    STORE r9, r8
    JUMP wait8
copy_ok:
    ADD r9, r15
    JUMP copy_loop
resize:
    MOV r5, r9
    REPRO 1
    BRNZ r14, wait8
    JUMP top

not_writing:
    SUB r11, r12
    BRNZ r11, not_sealed
step_loop:
    REPRO 3
    BRNZ r14, wait8
    CONST r0, 0
    CONST r3, 6
    READ r11
    CONST r8, 2
    EQ r11, r8
    BRNZ r11, step_loop
    JUMP top

not_sealed:
    SUB r11, r12
    BRNZ r11, ready
    ; Invalid: repair once by recopying, then cancel
    CONST r8, M_REPAIR
    LOAD r9, r8
    BRNZ r9, cancel
    STORE r12, r8
    CONST r9, 0
    CONST r8, M_CURSOR
    STORE r9, r8
    REPRO 4
    JUMP top
cancel:
    REPRO 6
    JUMP top

ready:
    CONST r0, 8
    CONST r3, 4           ; embryo energy
    READ r9
    CONST r10, 16384
    MOV r11, r9
    LTU r11, r10
    BRNZ r11, invest
    CONST r8, M_DIR
    LOAD r5, r8
    REPRO 5
    BRNZ r14, release_fail
    CONST r9, 0
    STORE r9, r8
    JUMP top
release_fail:
    ADD r5, r12
    CONST r9, 3
    AND r5, r9
    STORE r5, r8
    JUMP wait8

invest:
    SUB r10, r9           ; deficit
    CONST r0, 0
    CONST r3, 0
    READ r9               ; own energy
    CONST r11, 40000
    MOV r8, r9
    LTU r8, r11
    BRNZ r8, wait16
    SUB r9, r11           ; spare above 40000
    MOV r5, r10
    MOV r8, r9
    LTU r8, r5
    BRNZ r8, inv_spare
    JUMP inv_go
inv_spare:
    MOV r5, r9
inv_go:
    CONST r0, 0           ; Self
    CONST r1, 8           ; Embryo
    CONST r3, 15          ; energy channel
    TRANSFER 0
    JUMP top

; ---- S gathering over self, front, right, back, left; wander when exhausted ----
gather_s:
    CONST r8, M_IDX
    LOAD r9, r8
    CONST r10, 5
    EQ r10, r9
    BRNZ r10, wander
    MOV r8, r9
    ADD r8, r9
    CONST r10, 16
    ADD r8, r10
    LOAD r1, r8
    ADD r8, r12
    LOAD r2, r8
    CONST r0, 1           ; Loose
    CONST r3, 0
    CONST r4, 7           ; S
    READ r10
    BRNZ r10, take_s
    CONST r0, 0
    CONST r3, 5           ; Self.fault (read and clear)
    READ r10
    CONST r8, 5
    EQ r8, r10
    BRNZ r8, gs_yield
    ADD r9, r12
    CONST r8, M_IDX
    STORE r9, r8
    JUMP gather_s
gs_yield:
    YIELD
    JUMP top
take_s:
    MOV r0, r9
    CONST r10, 4
    SHL r0, r10
    OR r0, r12            ; Loose at offset idx
    CONST r1, 0           ; Self inventory
    CONST r3, 7
    CONST r5, 2
    TRANSFER 0
    CONST r9, 0
    CONST r8, M_IDX
    STORE r9, r8
    JUMP top
wander:
    CONST r9, 0
    CONST r8, M_IDX
    STORE r9, r8
    CALL 2                ; hook: lost, may name a direction in r5 (4 = none)
    CONST r10, 4
    EQ r10, r5
    BRNZ r10, wander_any
    DRIVE 0
    JUMP top
wander_any:
    RANDOM r5
    CONST r10, 3
    AND r5, r10
    DRIVE 0
    JUMP top

; ---- G scavenging over self, front, right, back, left; sleep when none is found ----
gather_g:
    CONST r8, M_GIDX
    LOAD r9, r8
    CONST r10, 5
    EQ r10, r9
    BRNZ r10, gg_none
    MOV r8, r9
    ADD r8, r9
    CONST r10, 16
    ADD r8, r10
    LOAD r1, r8
    ADD r8, r12
    LOAD r2, r8
    CONST r0, 1           ; Loose
    CONST r3, 0
    CONST r4, 3           ; G
    READ r10
    BRNZ r10, take_g
    CONST r0, 0
    CONST r3, 5           ; Self.fault (read and clear)
    READ r10
    CONST r8, 5
    EQ r8, r10
    BRNZ r8, gs_yield
    ADD r9, r12
    CONST r8, M_GIDX
    STORE r9, r8
    JUMP gather_g
gg_none:
    CONST r9, 0
    CONST r8, M_GIDX
    STORE r9, r8
    CALL 2                ; hook: lost
    CONST r10, 4
    EQ r10, r5
    BRNZ r10, wait32
    DRIVE 0
    JUMP top
take_g:
    MOV r11, r10          ; G seen on that tile, for the hook
    MOV r0, r9
    CONST r10, 4
    SHL r0, r10
    OR r0, r12            ; Loose at offset idx
    CONST r1, 0           ; Self inventory
    CONST r3, 3
    CONST r5, 8
    TRANSFER 0
    CONST r9, 0
    CONST r8, M_GIDX
    STORE r9, r8
    CALL 1                ; hook: found food
    JUMP top

; ---- waiting ----
wait8:
    CONST r13, 8
    JUMP wait_tick
wait16:
    CONST r13, 16
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
.module 1                 ; found food: nothing
    RET
.module 2                 ; lost: no direction
    CONST r5, 4
    RET
.module 3                 ; rich and idle: adopt a genome-sized record of another length (once)
    CONST r8, M_LEARNED
    LOAD r9, r8
    BRNZ r9, out
    RANDOM r9
    CONST r10, 7
    AND r9, r10
    BRNZ r9, out       ; one idle spell in eight
    YIELD                 ; a fresh tick for the four record queries
    CONST r0, 6           ; my genome length (OwnCode, inherited bank)
    CONST r6, 0
    CONST r3, 1
    READ r10
    CONST r0, 5           ; PublicTape length: here #1, here #2, front #1, front #2
    CONST r3, 1
    CONST r1, 0
    CONST r2, 0
    CONST r6, 1
    CONST r7, 0x05        ; COPY source address: here
    READ r11
    JUMP pick
look2:
    CONST r6, 2
    READ r11
    JUMP pick
look3:
    CONST r2, -1
    CONST r6, 1
    CONST r7, 0x15        ; front
    READ r11
    JUMP pick
look4:
    CONST r6, 2
    READ r11
    JUMP pick
pick:                     ; r11 = length of record (r7 tile, r6 selector); r10 = my length
    CONST r9, 256
    MOV r8, r11
    LTU r8, r9
    BRNZ r8, next_look    ; nothing, or too short to be a genome
    MOV r8, r11
    EQ r8, r10
    BRNZ r8, next_look    ; same length: most likely my own kind
    JUMP learn
next_look:
    CONST r9, 0x15
    EQ r9, r7
    BRNZ r9, front_next
    CONST r9, 2
    EQ r9, r6
    BRNZ r9, look3
    JUMP look2
front_next:
    CONST r9, 2
    EQ r9, r6
    BRNZ r9, out
    JUMP look4
learn:
    CONST r8, M_LEARNED
    STORE r12, r8         ; only one attempt per life
    CONST r8, M_LEN
    STORE r11, r8
    CONST r8, M_SRC
    STORE r7, r8
    CONST r8, M_SEL
    STORE r6, r8
    MOV r5, r11
    CONST r9, 127
    ADD r5, r9
    CONST r9, 7
    SHR r5, r9            ; pages wanted
grow:
    CONST r0, 7           ; OwnDraft capacity
    CONST r3, 2
    READ r9
    CONST r8, M_LEN
    LOAD r11, r8
    LTU r9, r11
    BRNZ r9, grow_go
    JUMP lcopy_init
grow_go:
    CONST r0, 7
    WRITE 1               ; Resize: one more page (G1 + 128q)
    BRNZ r14, lcopy_drop
    JUMP grow
lcopy_init:
    CONST r9, 0           ; cursor
lcopy:
    CONST r8, M_LEN
    LOAD r11, r8
    MOV r8, r9
    LTU r8, r11
    BRNZ r8, lcopy_go
    MOV r5, r11
    WRITE 2               ; DraftCommit: adopted at the end of this tick
    RET                   ; back in the old main; the next CALLs bind the adopted hooks
lcopy_go:
    CONST r8, M_SRC
    LOAD r0, r8           ; PublicTape on the chosen tile
    CONST r8, M_SEL
    LOAD r7, r8           ; and record
    CONST r1, 7           ; -> OwnDraft
    MOV r4, r9
    MOV r5, r9
    CONST r6, 4
    COPY 0
    BRNZ r15, lcopy_ok
    CONST r0, 0
    CONST r3, 5           ; Self.fault (read and clear)
    READ r10
    CONST r8, 5
    EQ r8, r10
    BRNZ r8, lcopy_wait   ; query limit: try again next tick
lcopy_drop:
    WRITE 3               ; cannot finish: drop the draft (it becomes a public record here)
    RET
lcopy_wait:
    YIELD
    JUMP lcopy
lcopy_ok:
    ADD r9, r15
    JUMP lcopy

out:
    RET
