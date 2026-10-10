; Founder-5 public ancestor (revision of Founder-4). SPEC §16b.
; Same primitives and ABI as Founder-4. Changes are motivated by the E1-a budget
; decomposition (docs/results/E1.md): Founder-4 spent ~120 q/tick on its own decision
; loop and started reproduction late in the day, so it entered the night without reserves.
;
;   1. lean inner loops for COPY, Step and investment (receipt handled inline)
;   2. reproduce only with a reserve: G >= 50, S >= 2, E >= 60000
;   3. start reproduction only while light is rising or bright (>= 24576), not near dusk
;   4. idle in 32-tick sleeps; no wandering for G (photosynthesis is the G source)
;   5. invest the child's energy in one transfer instead of 16
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
    CONST r10, 50
    LTU r9, r10
    BRNZ r9, wait32
    CONST r3, 0
    READ r9
    CONST r10, 60000
    LTU r9, r10
    BRNZ r9, wait32
    ; enough day left: light rising, or still bright
    CONST r8, M_LIGHT
    LOAD r9, r8
    CONST r8, M_PREV
    LOAD r10, r8
    LTU r10, r9
    BRNZ r10, allocate
    CONST r10, 24576
    LTU r9, r10
    BRNZ r9, wait32
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
    RANDOM r5
    CONST r10, 3
    AND r5, r10
    DRIVE 0
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
