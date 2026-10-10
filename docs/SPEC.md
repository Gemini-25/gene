# Gene M0 현행 명세

ruleset `gene-m0-1` · 저장 형식 `GENEM001` · 2026-10-10

## 0. 이 문서의 위치

이 문서가 **유일한 현행 명세**다. 다른 문서의 규칙이 이 문서를 덮어쓰지 않는다.

- 원본 설계서 R10(`docs/archive/R10-original-design.md`)은 보관용 근거 자료다. 괄호 안의 `§n`은 원본의 절 번호다.
- 원본의 Grid-5 기준(§06..§67) 중 M0에 필요한 부분만 옮겼다. 원본에서 모호했거나 서로 충돌하던 부분은 §20에 결정을 적었다.
- Matter-1, R7..R10(§68..§138)의 확장은 이 명세에 없다. §21의 증거 규칙을 통과해야 다시 들어온다.
- 규칙을 바꾸면 ruleset 번호를 올린다. 이전 번호로 얻은 실행 결과와 섞지 않는다.

## 1. 목표와 범위

M0가 답하려는 질문은 하나다.

> 공개 조상 Founder-4가 이 세계의 에너지·물질 수지 안에서 살아남고, 번식하고, 변이를 세대에 걸쳐 전달하는가?

이 질문에 답하는 데 필요한 것만 구현한다.

| 포함 | 제외 (M1 이후) |
| --- | --- |
| 2D 격자, 지형 생성, 빛·외기·열·기체·물 수송 | parcel·matrix·pore, LINK/UNLINK, grip, 조립체 이동 |
| 10종 물질, 원소·에너지 장부, Rx0..Rx8 | EMIT·신호장, LocalTrace |
| 고정 몸체·유지비·호흡·광합성·열 손상·죽음 | 공개 tape, draft, learned binding, Commit |
| 가변 genome, 레지스터 VM, I/O ABI | 남의 inventory 인출(외피 0 탈취, 원본 v1) |
| 배아 상태 기계, 단계적 복제, 변이 | PushActor·PushParcel |
| DRIVE Translate/Rotate, TRANSFER, WRITE, COPY, WORK(Heat/Compress/Abrade 표면) | Bevy 화면, 병렬 Think |
| 결정론, 상태 hash, checkpoint·resume | 관찰기의 문명·기술 판정 |
| 헤드리스 실행, CSV 통계, PNG 지도 | |

제외된 capability의 opcode는 비용을 내고 `InvalidTarget`(효과) 또는 `BadAddress`(READ)를 돌려준다. 다른 기능으로 대체되지 않는다. (§05)

금지 사항(§04): core에 `Job`, `House`, `Road`, `Civilization`, `TechnologyUnlock` 같은 역할·장치 enum을 두지 않는다. 관찰 결과가 세계에 되돌아가지 않는다. 문명이 안 나온다고 보너스를 넣지 않는다.

## 2. 수치 규칙

- 모든 세계 상태는 정수다. 부동소수점은 출력·분석에만 쓴다.
- 양수 나눗셈은 floor, signed 나눗셈은 0 방향 절삭이다.
- 에너지 총계는 i128, 원소 총계는 u128로 계산한다.
- VM 산술(u16 modulo) 외에 세계 상태의 wrap은 금지다. 넘치면 실행을 멈춘다(RunHalt).
- 에너지 단위는 q, 온도 θ = floor(U / C).

## 3. 세계

### 3.1 기본 설정

| 항목 | 값 |
| --- | --- |
| 크기 | 512×512 (64의 배수), 경계는 닫힘(torus 아님) |
| 시작 개체 | 1,000 |
| 낮·밤 | 주기 4096 tick, tick 0은 정오 |
| 환경 갱신 | tile별 16 tick마다(due: tick mod 16 = TileIndex mod 16) |
| 수송 | edge별 8 tick마다 |
| 1x 속도 | 10 tick/s(표시용, 물리와 무관) |

TileIndex = y×width + x. 방향 0..3 = 북·동·남·서, 북은 y−1.

### 3.2 tile 필드

| 필드 | 형식 | 의미 |
| --- | --- | --- |
| elevation | i16 (0..15) | 고도 |
| mu | u16 | 마찰 μ |
| lpeak | u16 | 정오 최대 광량 |
| substrate | u8 | 지형 태그(생존 효과 없음) |
| surface_c | u8 | 표면 압축도 |
| loose | [u16;10] | 흩어진 물질 |
| residue | [u64;10] sparse | u16를 넘는 강제 반환분 |
| heat_u | u64 | 열에너지 U (권위 상태) |
| heat_c | u64 | 열용량 C (파생 cache) |

C = 1024 + Σ cᵢ·nᵢ. 같은 tile의 loose·residue·개체(몸체·inventory·기억·매체·배아)·버려진 매체를 모두 한 번씩 센다. 1024는 움직이지 않는 기반 열용량이다.

READ로 loose를 볼 때는 loose+residue를 u16로 포화해 보여 준다. 채취는 loose 먼저, residue 다음이다.

### 3.3 지형 생성 WorldGen-3 (§07)

정수 value noise. 간격 L=64와 L=16의 꼭짓점마다 u16 난수(§15의 WorldGen stream, object=x | y<<32, ordinal=field×2+해상도, lane0의 하위 16bit)를 두고, 칸 안의 좌표 (u,v)에서

V = floor(((L−u)(L−v)·a + u(L−v)·b + (L−u)v·c + uv·d) / L²)

최종값 = floor((3·V64 + V16) / 4). field 번호는 height 0, moisture 1, light 2, ore 3. 해상도 번호는 L64=0, L16=1.

| 파생값 | 식 |
| --- | --- |
| elevation | floor(height / 4096) |
| μ | 256 + floor(moisture / 128) |
| Lpeak | 32768 + floor(light / 2) |
| substrate | floor(moisture / 16384) |
| surface_c | 0 |

초기 loose: W = 8 + floor(moisture/2048), C=16, O=16, G = (draw0 mod 4), S=2. ore ≥ 49152이면 X = 32 + (draw1 mod 32), 아니면 0. H, M, V, I = 0. tile 재고 stream은 object=TileIndex, ordinal=8.

시작 위치: 자기 G ≥ 1이고, 네 이웃이 모두 지도 안이며 고도차 ≤ 4인 tile. spawn hash(object=TileIndex, ordinal=10, lane0)의 오름차순(동률은 TileIndex)으로 1,000개를 고른다. 모자라면 설정 실패이며 seed를 몰래 바꾸지 않는다.

각 시작 개체에는 몸체 G16/S2, 작업 기억 G1, code 페이지 수만큼 G, inventory G32/S4, energy 32768을 외부 투입한다. 모든 배치가 끝난 뒤 모든 tile을 U = 300·C로 둔다. 자원을 나중에 자동 재충전하지 않는다.

## 4. 물질과 에너지 장부 (§09)

원소 순서 C, H, O, Fe, Si. 원소 질량 12, 1, 16, 56, 28.

| ID | 기호 | 조성 | m | v | c | h (q/단위) |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | W | H2O 액체 | 18 | 1 | 3 | −3000 |
| 1 | C | CO2 기체 | 44 | 4 | 3 | −4000 |
| 2 | O | O2 기체 | 32 | 4 | 2 | 0 |
| 3 | G | CH2O 유기물 | 30 | 1 | 4 | −2000 |
| 4 | H | H2 기체 | 2 | 4 | 2 | 0 |
| 5 | X | FeO 산화물 | 72 | 1 | 2 | −2800 |
| 6 | M | Fe 금속 | 56 | 1 | 1 | 0 |
| 7 | S | SiO2 불활성 | 60 | 1 | 3 | −9000 |
| 8 | V | H2O 증기 | 18 | 4 | 3 | −2560 |
| 9 | I | H2O 얼음 | 18 | 1 | 3 | −3060 |

h는 모델 값이며 실측 화학 데이터가 아니다.

**장부 식.**

E = Σ nᵢhᵢ + Σ U + Σ agent.energy + Σ embryo.energy + Σ photo_credit + Ug

Ug = Σ(질량 × 그 물질이 있는 tile의 elevation). 몸체·inventory·매체·배아·버려진 매체를 모두 포함한다.

**불변식.**
- E(t) = E(0) + solar_in + sky_in − radiation_out.
- 원소별 원자 수 A[e]는 시작값과 같다.
- 두 식은 시험에서 매 tick 확인하고, 실행 중에는 checkpoint마다 확인한다. 어긋나면 RunHalt다.

**열의 이동.** 물질이 다른 tile로 가면 열 q_heat = floor(U_src × c_move / C_src)를 함께 옮긴다. 계산은 이동 전 상태로 한다. 같은 tile 안의 이동은 열 이동 0이다.

**위치 에너지.** ΔUg > 0이면 행동한 개체의 action budget에서 추가로 낸다(자연 수송은 동반 열에서 낸다). ΔUg < 0이면 그만큼 목적지 열이 된다.

**비용의 행방.** 모든 계산·행동·마찰 비용은 해당 tile의 U로 간다. 열을 가용 에너지로 되돌리는 primitive는 없다.

**매체 비용.** 매체 128 cell(1 페이지)마다 G1. 작업 기억도 G1이다. 빈 유료 페이지도 질량·h·열용량을 가진다.

## 5. 환경

### 5.1 노출과 빛 (§10)

M0에는 parcel이 없으므로 모든 tile의 투과 P = 256이고, 지도 밖은 P = 0이다.

X = max(1, floor((P_self + Σ이웃 min(P_self, P_n)) / 5))

따라서 내부 256, 가장자리 204, 모서리 153이다.

due tile에서 처리하는 순서:

1. **빛.** p = (tick+1024) mod 4096, f = max(0, 1024 − |p − 1024|), L = floor(Lpeak × f / 1024). solar_in += L. 반사분 floor(L/4)는 radiation_out이다. absorbed = L − floor(L/4).
2. **광합성 credit.** 살아 있는 개체가 있고 260 ≤ θ ≤ 330이면 floor(absorbed × X × min(body_g,16) / 4096)이 그 개체의 photo_credit으로 가고, 나머지는 U로 간다. 조건이 안 맞으면 전부 U다. credit 상한 49,999를 넘는 분량은 U로 간다. 그 뒤 floor(credit/256)이 누출되어 U로 간다.
3. **Rx0 광반응.** 최대 4회.
4. **외기 교환.** k = ceil(2048·X / 256), q = min(|U − 280·C|, k·|θ − 280|). θ > 280이면 U −= q, radiation_out += q. θ < 280이면 U += q, sky_in += q.
5. **외부 화학.** Loose site, 이어서 버려진 매체 site를 UID 순으로.
6. **열 손상.** θ > 340이면 그 tile 개체의 body_g를 1 줄여 loose G로 내놓는다.

### 5.2 자연 수송 (§10, §63)

edge는 수평·수직 인접만 다룬다.
- **bucket**: (작은 TileIndex u32 LE, 큰 TileIndex u32 LE)의 BLAKE3 첫 byte & 7. bucket = tick mod 8인 edge만 처리한다.
- **color**: 동쪽 edge는 시작 x 짝/홀이 0/1, 남쪽 edge는 시작 y 짝/홀이 2/3.
- **color 순서**: 매 tick Resolve stream(object 0, ordinal 7)의 Fisher-Yates 순열을 따른다. 같은 color의 edge는 endpoint가 겹치지 않는다.

edge마다 기체 C, O, H, V 순서로 처리하고, 이어서 W, 마지막으로 열을 처리한다.

**정수화.** SR(N, D) = floor(N/D) + [r < N mod D]. r은 FluxRng(§15)로 얻은 0..D−1의 정수다. N = 0이면 0이다.

| 대상 | 방향 | 후보량 |
| --- | --- | --- |
| 기체 | 재고 많은 쪽 → 적은 쪽 | min(32, SR(\|nA−nB\|·minP, 2048)) |
| W, 고도 다름 | 높은 쪽 → 낮은 쪽만 | min(32, SR(32·minP, 256)) |
| W, 고도 같음 | 많은 쪽 → 적은 쪽 | min(32, SR(\|nA−nB\|·minP, 8192)) |

후보량은 원천 재고와 목적지 여유(65535 − n)로 다시 제한한다. 동반 열은 §4의 q_heat다. ΔUg > 0인 이동은 q_heat ≥ ΔUg일 때만 가능하고, 목적지에 q_heat − ΔUg를 준다. 안 되면 후보량을 큰 값부터 줄여 가능한 최댓값을 쓴다. ΔUg < 0이면 목적지 U에 q_heat + |ΔUg|를 더한다.

**열전도.** θ_hot > θ_cold일 때 κ = max(1, floor(min(CA, CB) × minP / 16384)). q = κ·Δθ이고, 상한은 floor(Δθ·CA·CB / (CA+CB))와 U_hot 중 작은 값이다.

### 5.3 화학 반응 (§12)

| ID | 반응 | 조건 | Δh | U |
| --- | --- | --- | --- | --- |
| Rx0 | C+W→G+O | 광반응: 개체 photo_credit 10000 소비, 260≤θ≤330 | +5000 | +5000 |
| Rx1 | G+O→C+W | 호흡: 개체 inventory G, 260≤θ≤340, 가용 +1666 | −5000 | +3334 |
| Rx2 | G+O→C+W | 외부 산화(속도표) | −5000 | +5000 |
| Rx3 | 2H+O→2V | θ≥500 | −5120 | +5120 |
| Rx4 | 2X+G→2M+C+V | θ≥900, loose O≤1 | +1040 | −1040 |
| Rx5 | X+H→M+V | θ≥700, loose O≤1 | +240 | −240 |
| Rx6 | 2M+O→2X | 속도표 | −5600 | +5600 |
| Rx7 | W↔V | 증발 θ≥374, 응축 θ≤372, due당 최대 4 | ±440 | ∓440 |
| Rx8 | W↔I | 동결 θ≤272, 융해 θ≥274, due당 최대 4 | ∓60 | ±60 |

Rx0는 개체 inventory의 G 여유, tile loose의 C1·W1, O 출력 여유를 요구한다. Rx1은 inventory G1, tile loose O1, C·W 출력 여유, energy + 1666 ≤ 65535를 요구한다. 이 두 반응만 막 통로를 통해 자동으로 일어난다.

속도표 (온도 bin: <270, 270..329, 330..499, 500..699, 700..899, ≥900):

| 반응 | 속도 |
| --- | --- |
| Rx2 | 0, 1, 4, 256, 1024, 4096 |
| Rx3 | 0, 0, 0, 4096, 4096, 4096 |
| Rx4 | 0, 0, 0, 0, 0, 4096 |
| Rx5 | 0, 0, 0, 0, 4096, 4096 |
| Rx6 | 0, 1, 4, 32, 128, 512 |

credit 갱신: credit += SR(rate × X × min(16, 가능 extent), 256). credit ≥ 65536인 동안 조건(반응물·온도·여유·U)을 다시 검사하며 최대 4 extent를 실행하고, 실행할 때마다 65536을 뺀다. 남은 credit은 65535로 자른다. credit은 에너지가 아니다.

**버려진 매체 site.** Rx2만 일어난다. 가장 큰 Present 페이지부터 한 페이지(G1)를 소비해 hole로 만든다. O는 같은 tile loose에서 받고, C·W는 loose로 낸다. 모든 페이지가 없어지면 매체를 지운다.

기본 세계는 H = 0이고 H를 만드는 반응이 없으므로 Rx3과 Rx5는 도달할 수 없다(§62). 반응 표에 행이 있다는 사실만으로 생산 경로가 있다고 주장하지 않는다.

## 6. 생명체 (§13)

**구성.** 개체 1칸, 몸체 G16/S2, 작업 기억 64 u16(1 페이지), 상속 code 매체, inventory(유효 부피 합 ≤ 64), energy 0..65535, 방향, photo_credit, port_mask, 배아 ≤ 1.

**유지비.** 매 tick M = 32 + 1(작업 기억) + code Present 페이지 수 + 배아(작업 기억 1 + raw 페이지 + result 페이지).
- stage1에 M을 못 내면 남은 energy를 열로 옮기고 그 tick은 Inert(VM 미실행)다.
- stage9에서 Rx1이 가능하면 회복할 수 있다.
- stage10에 energy = 0이거나 body_g < 8이면 죽는다. 나이로 죽이지 않는다.

**죽음.**
- 몸체, inventory, 작업 기억, 배아의 몸체·기억·반환 재고는 같은 tile loose로 돌아간다.
- code 매체와 배아 매체는 같은 UID·bytes의 버려진 매체로 남는다. 그 G를 loose에 중복으로 더하지 않는다.
- energy, photo_credit, 배아 energy는 U로 간다.

## 7. Genome 형식 (§14)

원시 매체 단위는 u16 cell이다. instruction은 연속 두 cell의 LE u32다. 구성은 header 16 cell, descriptor 8 cell × module_count, instruction 2 cell × word_count 순서다.

| header cell | 의미 | 검증 |
| --- | --- | --- |
| 0, 1 | magic 0xC17A, format 2 | 불일치면 형식 실패 |
| 2, 3, 4 | entry_tag, module_count(1..64), code_word_count(1..4096) | entry가 존재해야 함 |
| 5, 6 | point_threshold, structural_threshold | 모든 값 허용 |
| 7..14 | 구조 변이 연산자 8개의 가중치 | 합 0이면 구조 변이 없음 |
| 15 | reserved = 0 | |

**descriptor.** (tag, start_word, length_word, flags=0, reserved×4=0). tag는 유일하다. descriptor 순서대로 start가 0부터 빈틈없이 이어지고 length ≥ 1이다. 총 cell 수는 16 + 8m + 2w = used prefix 길이와 같아야 한다. raw 매체는 최대 9216 cell(72 페이지)이다.

invalid opcode, 깨진 branch, 없는 CALL tag는 형식 실패가 아니라 실행 중 fault다.

## 8. VM (§15)

**인코딩.** bits 0..7 opcode, 8..11 a, 12..15 b, 16..31 imm. 레지스터는 u16 16개이고, signed 해석은 i16이다.

**tick당 실행.**
- 연료는 32 instruction이다.
- 매 명령마다 4q를 낸다. 모자라면 fault EnergyStop을 기록하고 pc를 그대로 둔 채 tick을 끝낸다.
- 외부 query(domain 1, 2, 3, 4, 5, 9, 10, 11)는 8q를 더 내고 tick당 4회까지다. 5회째는 4q만 내고 QueryLimit와 결과 0을 돌려준다.
- tick이 끝나도 pc·frame·레지스터는 다음 tick으로 이어진다.

| op | 이름 | 의미 |
| --- | --- | --- |
| 00 | NOP | |
| 01 | CONST | ra = imm |
| 02 | MOV | ra = rb |
| 03..05 | ADD, SUB, MUL | ra = ra op rb (mod 65536) |
| 06, 07 | DIV, REM | unsigned. rb = 0이면 ra = 0, DivZero |
| 08..0A | AND, OR, XOR | |
| 0B, 0C | SHL, SHR | shift = rb & 15, SHR은 unsigned |
| 0D..0F | EQ, LTU, LTS | ra = (ra op rb) ? 1 : 0 |
| 10 | BRNZ | ra ≠ 0이면 pc+1+i16(imm) |
| 11 | JUMP | pc+1+i16(imm) |
| 12 | CALL | imm tag로 이동, 복귀 pc+1. frame ≤ 8 |
| 13 | RET | frame pop. 빈 stack이면 entry 처음으로 가고 yield |
| 14 | LOAD | ra = mem[rb] |
| 15 | STORE | mem[rb] = ra |
| 16 | RANDOM | ra = 이번 tick의 다음 draw 하위 16bit |
| 17 | YIELD | pc+1을 저장하고 tick 종료 |
| 20 | READ | imm 0만 유효. 인자 r0..r7, 결과 ra |
| 21 | WRITE | imm = sub-op. 효과 |
| 22..28 | DRIVE, LINK, UNLINK, TRANSFER, WORK, EMIT, REPRO | imm = sub-op. 효과 |
| 29 | COPY | 효과 |
| 그 외 | — | 4q, pc+1, InvalidOpcode |

**효과 opcode.** WRITE, DRIVE..REPRO, COPY는 효과 intent를 하나 제출하고 yield한다(§20 D2). tick당 효과는 1개다.

**주소와 제어 흐름.**
- 메모리 주소 64 이상은 BadAddress다. LOAD는 0을 돌려주고, STORE는 무시된다.
- branch 목적지가 module 밖이면 branch를 취소하고 pc+1로 간다.
- pc = module 길이이면 다음 실행 단위에서 implicit RET(4q, 연료 1)를 한다.
- CALL 실패와 stack 가득 참은 pc+1이고 fault를 남긴다.

**fault.** 번호는 Ok 0, BadAddress 1, DivZero 2, InvalidOpcode 3, StackFull 4, QueryLimit 5, MissingTag 6, Occluded 7, Boundary 8, EnergyStop 9. fault는 마지막 오류를 가두는 latch다. 다른 성공 명령으로는 지워지지 않고, READ Self.fault가 값을 돌려준 뒤 Ok로 지운다.

**r14/r15.** 지난 세계 효과의 outcome과 actual량이다. tick 시작에만 갱신되고, 효과가 없었던 tick에는 유지된다.

## 9. 주소와 감각 (§16, §52)

**인자.** r0 = domain, r1 = dx, r2 = dy, r3 = field, r4 = index, r5 = 값·양, r6, r7 = 보조.

**상대 좌표.** dx, dy는 i16이고 개체의 방향 기준이다. dy = −1이 앞, dx = +1이 오른쪽이다. 방향 o만큼 시계방향으로 회전한 결과가 절대 좌표다(90° 회전: (dx, dy) → (−dy, dx)).

**범위.** READ는 Manhattan 반경 2, 효과는 접촉 반경 1이다. M0에는 차폐물이 없어 Occluded가 생기지 않는다. 지도 밖은 Boundary와 결과 0이다.

| domain | field (M0) |
| --- | --- |
| 0 Self (내부) | 0 energy, 1 inventory[r4], 2 age_lo, 3 age_hi, 4 orientation, 5 fault(읽고 지움), 6 embryo_state, 7 inherited_len, 8 draft_len(=0), 9 port_mask, 10 last_actual, 11 body_g, 12 shell_s |
| 1 Loose (외부) | 0 stock[r4] |
| 2 Parcel (외부) | 6 present = 0. 다른 field는 BadAddress |
| 3 ActorSurface (외부) | 0 present, 1 orientation(절대), 2 shell_present, 3 port_mask. 개체가 없으면 present만 0, 나머지는 BadAddress |
| 4, 5, 9, 10 (외부) | M0 capability 꺼짐 → BadAddress |
| 6 OwnCode (내부) | 0 cell[r4], 1 logical_len, 2 capacity. bank r6 = 0만 |
| 7 OwnDraft (내부) | 1, 2, 3 = 0. cell은 BadAddress |
| 8 EmbryoTape (내부) | 0 cell[r4], 1 logical_len, 2 capacity, 3 state, 4 energy, 5 return_stock[r4]. 배아가 없으면 cell만 BadAddress, 나머지는 0 |
| 11 Surface (외부) | 0 elevation, 1 μ, 2 surface_c, 3 θ(65535 포화), 4 current_light |

embryo_state: None 0, Writing 1, SealedWork 2, Invalid 3, Ready 4.

UID, 계통, 절대 좌표는 감각에 나타나지 않는다.

## 10. 효과 (§17, §18, §52)

**시도비.** 효과는 VM 비용과 별개로 시도비를 낸다. 기본 64q, REPRO Allocate·Finish·Release는 256q, EMIT은 128q다. 시도비조차 없으면 intent를 제출하지 않고 다음 tick에 Energy receipt를 남긴다.

**action budget.** stage3에서 남은 energy를 action budget으로 봉인한다. 성공량 비용은 이 budget에서 낸다. 같은 tick에 남에게 받은 energy는 budget을 늘리지 않는다.

**outcome.** Ok 0, Partial 1, Noop 2, Boundary 3, Blocked 4, Energy 5, Matter 6, Capacity 7, InvalidTarget 8, Stale 9, Limits 10, Format 11, Occluded 12, QueryLimit 13, Busy 14.

**실패 판정 순서.** 주소 → capability → 접촉 → generation → 형식 → energy → matter → capacity → 점유.

**packed address.** bits 0..3 domain, 4..6 offset(0 자기 tile, 1 앞, 2 오른쪽, 3 뒤, 4 왼쪽), 나머지는 0이다. Self·OwnCode·Embryo는 offset 0만 허용한다.

| 효과 | 인자 | 비용과 결과 |
| --- | --- | --- |
| DRIVE 0 Translate | r5 상대방향 0..3 | 256 + ceil(질량 × μeff / 960) + max(0, ΔUg). 원자적. 단계 6에서 처리 |
| DRIVE 1 Rotate | r5 quarter-turn 0..3 | 추가 64q. 0이면 Noop |
| DRIVE 2, 3 Push | — | InvalidTarget (M0 꺼짐) |
| TRANSFER | r0 원천, r1 목적지, r3 채널, r5 양 | 물질 단위당 16q + ΔUg. 가능한 최대량까지 부분 성공 |
| TRANSFER energy | r3 = 15, 원천 Self, 목적지 ActorInventory(port bit15) 또는 Embryo | q + ceil(q/64) |
| WORK 0, 1 | r0 = 11 Surface, r5 일 q | Compress 64q당 c+1, Abrade 256q당 c−1. 완성 단위만, 부분 성공 |
| WORK 2 Heat | r0 = 0/1/3/11, 위치, r5 일 q | 목표 tile U += q. 부분 성공 |
| WRITE 0 Cell | r0 = 8, r4 index, r5 값 | 16q. Writing 상태의 배아만 |
| WRITE 1 Resize | r0 = 8, r5 목표 페이지 | REPRO Resize와 같음 |
| WRITE 4 Orientation | r5 절대방위 | 추가 64q. 같은 방위면 Noop |
| WRITE 5 Port | r5 mask | bits 10..14가 0이 아니면 InvalidTarget |
| WRITE 2, 3 | — | InvalidTarget (M0 꺼짐) |
| COPY | r0 원천, r1 목적지, r4 원천 offset, r5 목적지 offset, r6 count 1..4, r7 원천 bank | cell당 32q. 가능한 prefix까지 |
| LINK, UNLINK, EMIT | — | InvalidTarget (M0 꺼짐) |

**TRANSFER 주소.** 물질 원천은 0 SelfInventory, 1 Loose, 8 EmbryoReturn이다. 목적지는 0, 1, 3 ActorInventory(해당 종 port bit 필요), 8이다. 원천과 목적지가 같은 저장소이면 Noop이고 시도비만 낸다. 위치가 다른 이동은 §4대로 열과 Ug를 정산한다.

**inventory 용량.** 유효 부피 Σ vᵢnᵢ ≤ 64. 배아 반환 재고도 같은 상한을 쓴다.

**COPY 원천과 목적지.** 원천은 OwnCode(6, bank 0) 또는 Embryo(8), 목적지는 Embryo(8)이고 배아가 Writing일 때만 쓸 수 있다.
- 원천 bytes는 Think에서 캡처한다. resolve 때 원천 revision이 바뀌었으면 Stale이다.
- 원천의 logical_len이나 목적지 capacity가 모자라면 가능한 prefix까지만 복사한다.

**μeff.** 목적지 tile의 min(μ, max(64, μ − floor(surface_c/2)))다.

**질량.** 몸체, inventory, 작업 기억, 매체, 배아를 모두 더한다. 이동 마찰 열은 원천 tile로 간다(§20 D5).

## 11. 배아와 복제 (§19, §55)

REPRO sub-op: Allocate 0, Resize 1, Finish 2, Step 3, Reopen 4, Release 5, Cancel 6.

| 상태 | 허용 | 효과 |
| --- | --- | --- |
| None | Allocate | inventory G17, S2를 배아 몸체(G16, S2)와 작업 기억(G1)으로 옮김 → Writing. 배아 ordinal + 1 |
| Writing | Resize, COPY, WRITE Cell, Finish, Cancel | raw 매체 작성 |
| SealedWork | Step, Cancel | 변이 결과를 새 매체에 Step당 최대 4 cell씩 기록 |
| Invalid | Reopen, Cancel | Reopen하면 Writing |
| Ready | Release, Reopen, Cancel | Reopen하면 Writing |

**Resize** (한 intent에 최대 한 페이지):
- 늘리기: 페이지 하나를 추가하고 G1 + 128q를 낸다.
- 줄이기: 마지막 페이지를 cell 0부터 4 cell씩 32q/cell로 재활용한다. 128 cell을 마치면 페이지를 지우고 G1을 loose로 돌려준다.
- 최대 72 페이지.

**Finish(r5 = 길이).** r5 > capacity이거나 0이면 InvalidTarget이다. 길이 r5의 prefix를 형식 검사하며 2q/cell을 원자적으로 낸다. r5 > logical_len이면 형식 실패다.
- 실패: Invalid. 아직 변이를 추첨하지 않는다.
- 성공이고 변이 미적용: 변이 계획을 **한 번** 추첨한다(§12). 결과가 prefix와 같으면 바로 Ready이고 mutation_applied가 된다. 다르면 SealedWork다.
- 성공이고 이미 변이 적용됨(Reopen 이후): 추가 변이 없이 Ready.

**Step.**
- 결과 bytes 전체를 새 result 매체에 최대 4 cell씩 기록한다. 32q/cell.
- 새 페이지가 필요하면 inventory G1 + 128q를 낸다. G가 없으면 Matter, energy가 없으면 Energy다.
- 마지막 cell을 쓴 Step에서 2q/cell의 최종 형식 검사를 낸다. budget이 모자라면 다음 Step이 검사만 한다.
- 검사가 끝나면 result가 raw를 대체하고, 옛 raw는 부모 tile에 버려진 매체로 남는다. mutation_applied = true가 된다. 형식이 유효하면 Ready, 깨졌으면 Invalid다. 다시 추첨하지 않는다.
- 새 매체와 옛 매체가 함께 있는 동안 두 매체 모두 G와 유지비를 낸다.

**Release(r5 = 상대방향).**
- 요구: Ready, 배아 energy ≥ 16384, 인접 tile이 지도 안이고 개체가 없으며 다른 출생 예약이 없음.
- stage4에서 예약하고, stage10에서 부모가 살아 있고 목표 tile이 비어 있으면 출생한다.
- 자식은 몸체, 작업 기억, raw 매체(상속 code), 반환 재고(inventory 상한 안, 넘치면 출생 tile loose), energy를 받는다. 방향은 출생 방향(절대)이다(§20 D4).
- 이동하는 물질의 열과 ΔUg를 정산한다. ΔUg > 0은 부모의 남은 budget에서 내며, 모자라면 Energy로 실패하고 배아는 Ready로 남는다.
- 자식은 다음 tick부터 실행한다.

**Cancel.** 배아 몸체·작업 기억·반환 재고를 부모 inventory 여유만큼 돌려주고 나머지는 tile loose로 보낸다. 매체는 버려진 매체로 남고, 배아 energy는 열이 된다.

**에너지 투자.** TRANSFER energy로 목적지 Embryo(8)를 지정해 넣는다. 상한은 65535다.

## 12. 변이 (§20)

**RNG.** Mutation stream(object = 부모 uid, ordinal = 배아 ordinal, counter = 현재 tick). 확률은 부모의 상속 genome header로 정하고, 자식의 header는 다음 세대부터 쓰인다.

**순서.** 먼저 구조 변이를 유효 구문 위에 0 또는 1회 적용하고, 그 결과에 point 변이를 적용한다.
- 구조 변이: draw16 < structural_threshold이면 가중치 비례로 연산자를 고른다.
- point 변이: raw의 32bit 쌍마다 draw16 < point_threshold이면 균일한 bit 하나를 뒤집는다. 마지막에 홀로 남은 cell은 하위 16bit 중에서 고른다. header와 descriptor도 대상이다.

| 연산자 | 결과 |
| --- | --- |
| 0 Replace | 균일한 word 하나를 random u32로 교체 |
| 1 Insert | 균일한 module과 경계에 random word 1..8개 삽입 |
| 2 Delete | module 안의 연속 word 1..min(8, len−1)개 삭제. 길이 1이면 Noop |
| 3 DuplicateBlock | 1..16 word 구간을 균일한 module 경계에 복제 삽입 |
| 4 MoveBlock | 같은 module 안에서 1..16 word를 다른 경계로 이동 |
| 5 DuplicateModule | module을 복제해 바로 뒤에 두고 가장 작은 빈 tag를 붙임 |
| 6 SplitMerge | draw 최하위 bit로 분할 또는 결합 |
| 7 Recombine | 부모의 상속 genome에서 module 하나를 골라 대상 module의 내용을 덮어씀(tag 유지, §20 D6) |

상한(word 4096, module 64, raw 9216 cell)을 넘거나 적용할 수 없으면 Noop이고, 다시 추첨하지 않는다. branch offset이나 CALL tag를 자동으로 고쳐 주지 않는다. 균일 선택은 rejection sampling을 쓴다.

## 13. tick 순서 (§31, §32)

| stage | 처리 |
| --- | --- |
| 0 | 입력(M0에는 없음) |
| 1 Maintenance | 유지비 → 열. 못 내면 Inert |
| 2 Think | receipt를 r14/r15에 넣고 VM 실행. VM 비용은 개체 energy에서 빠지고 열은 보류한다. 세계는 읽기만 한다 |
| 3 Seal | 보류한 VM 열과 시도비를 tile U에 반영하고 action budget을 봉인. 출생 UID를 부모 uid 순으로 예약 |
| 4 LocalResolve | Translate를 뺀 효과를 (block color 순위, priority, uid) 순서로 처리 |
| 5 ComplexMotion | M0에는 없음 |
| 6 SingleMotion | Translate 의존 graph |
| 7 Transport | edge 수송 |
| 8 Environment | due tile 처리(§5.1) |
| 9 Metabolism | Rx1, 개체당 최대 1 |
| 10 Lifecycle | 죽음, 출생 확정 |
| 11 Commit | 나이 +1, 장부 통계, tick + 1 |

**block color.** stage4는 64×64 block과 9색을 쓴다. Resolve stream(object 0, ordinal 4)에서 원점 offset ox, oy(각각 0..63)와 9색 순열을 뽑는다. bx = floor((x − ox) / 64), color = (bx mod 3) + 3·(by mod 3)이다. 같은 색 block은 서로 영향을 줄 수 없으므로, 직렬 기준 구현의 결과는 (color 순위, priority, uid) 정렬과 같다.

**priority.** Resolve stream(object = uid, ordinal = stage)의 lane0(하위)과 lane1(상위)을 합친 u64다.

**단일 이동 (stage6).**
1. 유효한 Translate 후보를 만든다. 지도 밖은 Boundary, budget이 모자라면 Energy다.
2. 같은 목적지에 후보가 여럿이면 priority가 가장 작은 하나만 남기고 나머지는 Blocked다. 남은 후보가 나중에 막혀도 차순위를 다시 고르지 않는다.
3. 목적지가 비었으면 성공, 움직이지 않는 개체가 있으면 실패, 후보 개체가 있으면 그 개체로 가는 의존 edge를 만든다.
4. 빈 곳으로 끝나는 경로와 닫힌 cycle은 전체 성공, 실패로 끝나는 경로는 전체 실패다.
5. 모든 원천의 열과 비용을 먼저 계산한 뒤 한꺼번에 반영한다. 맞교환과 순환도 허용한다.

## 14. 난수 (§33, §63)

**RNG-1.** Philox4x32-10. M0 = 0xD2511F53, M1 = 0xCD9E8D57, key 증가는 0x9E3779B9, 0xBB67AE85. zero key·zero counter의 첫 결과는 6627e8d5 e169c58d bc57ac4c 9b00dbd8이며, 시험으로 확인한다.

**stream key.** BLAKE3("civ-rng-v1" ‖ 0 ‖ seed[16] ‖ domain u32 ‖ object u64 ‖ ordinal u64)의 처음 8 byte를 두 u32 LE로 읽는다. counter = (tick_lo, tick_hi, block_lo, block_hi). draw d는 block = d/4, lane = d mod 4다. seed[16]은 사용자 seed u64의 LE를 0으로 채운 값이다.

domain: WorldGen 1, Controller 2, Mutation 3, Resolve 4, Environment 5, IdTie 6, Observer 7.

| 용도 | 주소 |
| --- | --- |
| VM RANDOM | Controller, object = uid, ordinal = 0. tick마다 draw 번호를 0부터 셈 |
| 수송 color 순열 | Resolve, object 0, ordinal 7 |
| stage4 block·색 | Resolve, object 0, ordinal 4 |
| priority | Resolve, object = uid, ordinal = stage |

**FluxRng-1.** key = BLAKE3("civ-flux-v1" ‖ 0 ‖ seed[16] ‖ domain u32 ‖ selector u32)의 처음 8 byte. counter = (object_lo, object_hi, tick_lo, tick_hi). r = lane0 & (D−1).

| domain | selector | object |
| --- | --- | --- |
| 8 EdgeFlux | 종 | (min TileIndex << 32) \| max TileIndex |
| 10 ReactionRound | (site_kind << 8) \| 반응 | Loose는 TileIndex, Media는 MediaUID |

hash map 순회, 주소, thread 순서, 시간, float는 결과에 쓰지 않는다.

## 15. 결정론·hash·저장 (§33, §39)

- **CoreHash.** 직렬화한 SimState(postcard)의 BLAKE3이다. 관찰기, 통계, wall time은 포함하지 않는다.
- **같은 seed, 같은 hash.** 같은 seed·설정·tick이면 실행마다 hash가 같아야 한다.
- **checkpoint.** 완결된 tick만 저장한다. 형식은 magic `GENEM001` + postcard(SimState) + BLAKE3 footer다.
- **resume.** 불러와서 이어 간 결과는 끊지 않고 실행한 결과와 hash가 같아야 한다.
- **ID.** UID는 단조 증가하며 재사용하지 않는다.

## 16. 공개 조상 Founder-4 (§57)

원본 §57의 우선순위 표를 일반 ISA로 조립한다(`assets/founder4.asm`). 조상 전용 opcode는 없고, 모든 상수와 루프는 변이할 수 있다.

**header.** entry 0, point 64, structural 2048, 가중치 [4, 2, 2, 2, 1, 1, 1, 1].

**작업 기억.** [1] 복사 cursor, [2] raw 길이, [3] 필요 페이지, [4] 탐색 index, [5] Release 방향, [6] 수리 횟수, [8] 직전 효과 종류, [9] 마지막 빛, [10] 탐색 대상 종, [11] 탐색 완료 flag, [12] 방금 대기 끝남, [16..25] 다섯 위치의 (dx, dy) 표. 대기 잔여는 레지스터 r13에 둔다(§20 D7).

행동 우선순위(위에서부터):

| # | 조건 | 행동 |
| --- | --- | --- |
| 0 | 처리 안 된 receipt | COPY면 cursor += actual. 성공한 Allocate·Reopen·Release는 각각 cursor와 수리 상태를 갱신. 실패한 Release는 방향 + 1. Energy이면 8 tick 대기 |
| 1 | 대기 중 | 대기 −1, YIELD |
| 2 | age & 31 = 0이거나 방금 대기 끝남 | 빛 READ. 8192 미만이면 32 tick 대기 |
| 3 | energy < 4096 | 8 tick 대기 |
| 4 | inventory G < 24 | 다섯 위치 중 G가 있는 첫 곳에서 최대 4 TRANSFER |
| 5 | 배아 없음, S < 2 | 같은 방식으로 S 최대 2 |
| 6 | 배아 없음, G ≥ 24, S ≥ 2, E ≥ 49152, 빛 ≥ 8192 | REPRO Allocate |
| 7 | Writing, capacity < 128 × 필요 페이지 | REPRO Resize |
| 8 | Writing, cursor < raw 길이 | COPY OwnCode → Embryo, min(4, 남은 길이) |
| 9 | Writing, cursor = raw 길이 | Finish |
| 10 | SealedWork | Step |
| 11 | Invalid, 수리 0 | Reopen |
| 12 | Invalid, 수리 1 | Cancel |
| 13 | Ready, 배아 energy < 16384, E ≥ 32768 | energy min(1024, 부족분) 이전 |
| 14 | Ready, 배아 energy ≥ 16384 | 앞/오른쪽/뒤/왼쪽 순서로 Release 시도 |
| 15 | 다섯 위치 탐색 실패, E ≥ 8192, 빛 ≥ 8192 | RANDOM 하위 2bit 방향으로 Translate |
| 16 | 그 외 | 8 tick 대기 |

Founder-4는 시작 조건의 편향으로 공개한다. 조상이 가진 확보, 복제, 대기 행동은 자연 발명으로 세지 않는다.

### 16b. 공개 조상 Founder-5 (E1-a 이후 개정판)

E1-a(`docs/results/E1.md`)에서 Founder-4는 20개 seed 모두 첫 밤에 전멸했다. 첫 오후의 개체·tick당 지출은 248q(유지 44, VM 111, 효과 92)였고, 광합성 수입은 180q였다. 물리 규칙(ruleset `gene-m0-1`)은 그대로 두고 시작 조건만 바꾼 개정판이 Founder-5다(`assets/founder5.asm`).

| 변경 | Founder-4 | Founder-5 | 근거 |
| --- | --- | --- | --- |
| 복사·Step·투자 루프 | 매 tick 우선순위 표 전체를 다시 평가(약 30 명령) | receipt를 루프 안에서 바로 처리(약 9 명령) | VM 111q/tick이 최대 지출 |
| 번식 시작 조건 | G ≥ 24, E ≥ 49152, 빛 ≥ 8192 | G ≥ 50, S ≥ 2, E ≥ 60000, 빛이 오르는 중이거나 ≥ 24576 | 해 질 무렵 번식 → 밤 비축 부족 |
| G 확보 | G < 24이면 주변 탐색·배회 | 배회하지 않음(광합성이 G 공급원). S가 모자랄 때만 탐색 | 이동 1회 약 520q |
| 대기 | 8 tick 단위 | 할 일이 없으면 32 tick 단위 | 깨어날 때마다 판단 비용 |
| 자식 energy 투자 | 1024q씩 16회 | 여유분(E − 40000)을 한 번에 | 시도비와 판단 비용 |
| 크기 | 369 word, 762 cell, 6 페이지 | 250 word, 524 cell, 5 페이지 | 유지비와 복사비 |

같은 ISA와 ABI만 쓴다. 조상 전용 opcode나 숨은 보조는 없다. 모든 상수와 조건은 genome 내용이라 변이할 수 있다. Founder-5가 가진 행동도 시작 조건의 편향으로 공개하며, 자연 발명으로 세지 않는다.

### 16c. 공개 조상 Founder-6 (E1-b 이후 개정판)

E1-b에서 Founder-5 개체군은 20개 seed 모두 49일을 살아남았지만 계속 줄었다. 신생아가 하루를 넘길 확률은 출생 시각에 따라 크게 달랐다(새벽 65%, 오전 37%, 이른 오후 24%, 늦은 오후 0%). 그런데 Founder-5는 G ≥ 50을 기다리느라 출생의 대부분이 이른 오후에 몰렸다.

Founder-6(`assets/founder6.asm`)은 번식 시작 조건 하나만 바꾼다.

| 항목 | Founder-5 | Founder-6 |
| --- | --- | --- |
| 번식 시작 시각 | 빛이 오르는 중이거나 ≥ 24576 | 빛이 오르는 중(오전)일 때만 |
| G 문턱 | ≥ 50 | ≥ 30 |
| E 문턱 | ≥ 60000 | ≥ 50000 |

오후에는 번식을 시작하지 않고 밤 비축을 남긴다. 나머지는 Founder-5와 같다. 248 word, 520 cell, 5 페이지.

## 17. 관찰과 출력

관찰은 세계를 바꾸지 않는다. 헤드리스 실행이 내는 파일:

| 파일 | 내용 |
| --- | --- |
| `stats.csv` | 주기별 개체 수, 출생, 사망 원인, 평균 energy·inventory G, 배아 상태, 변이 사건, 고유 genome 수, 최대 세대, 장부 잔차, 서식지 W·C |
| `births.csv` | tick, 자식, 부모, 세대, genome hash, 길이, 변이 여부, 위치 |
| `deaths.csv` | tick, uid, 원인, 나이, 자식 수 |
| `map_<tick>.png` | 고도, 물, 개체(계통 색) |
| `summary.json` | 실행 설정, 최종 상태, hash, 성능 |

## 18. 첫 실험 E1: 조상 생존 (§49 축소판)

**사전 고정 조건.**
- 탐색군: seed 1..20, 512², 개체 1000, 200,000 tick(약 49일).
- 시간이 모자라면 tick 수를 줄이고 censoring으로 표시한다.
- 실행 도중 규칙이나 threshold를 바꾸면 새 ruleset과 새 seed군으로 분리한다. 전멸한 결과도 보존한다.

**측정.**
- 개체 수 곡선과 첫 출생 시각
- 세대 깊이
- 출생·사망 원인
- 변이의 운명: 형식 실패, 복구, 생존 계통
- 고유 genome 수
- 서식지 조건: 개체 tile의 W, C, θ
- 장부 잔차 (항상 0이어야 함)
- TPS

**미리 정한 해석.**

| 결과 | 뜻 | 다음 행동 |
| --- | --- | --- |
| 첫 출생 전 전멸 | 조상 에너지 예산이 이 세계에서 성립하지 않음 | 원인별 수지 분해. 규칙 변경은 새 ruleset으로 |
| 출생은 있으나 감소·전멸 | 번식률 < 사망률 | 사망 원인과 서식지 분포 분석 |
| 안정 개체군 | 생존 가능, 진화 시작 가능 | 변이 계통과 행동 변화 관찰 → M1 판단 |
| 포화까지 증가 | 자원 한계가 선택압 | 수렴·다양성 측정 |

## 19. 성능 목표 (§45)

| 시나리오 | 목표 |
| --- | --- |
| S1: 512², 1k 개체 | 500..3000 TPS(목표, 실측 공개) |
| S2: 512², 10k 개체 | 100 TPS 이상(M2 gate) |

기준 구현은 단일 thread 직렬이다. 병렬화는 같은 hash를 내는 경우에만 들인다.

- **수송(stage 7):** 같은 color의 edge는 endpoint를 공유하지 않으므로, 각 edge가 한 띠 안에 들어가도록 행 단위 띠로 나눠 병렬 처리한다(동쪽 edge는 아무 행 경계, 남쪽 edge는 color 2면 짝수 행 경계, color 3이면 홀수 행 경계).
- **환경(stage 8):** 개체와 버려진 기록이 없는 due tile은 띠 단위로 병렬 처리하고, 나머지는 직렬 경로로 처리한다.
- residue가 있으면 직렬로 돌아간다.
- `threads` 값은 상태에 포함되지 않으며, 시험이 1 thread와 4 thread의 hash가 같음을 확인한다.

## 20. 원본과 다르거나 원본이 비워 둔 결정

| # | 결정 | 이유 |
| --- | --- | --- |
| D1 | M0는 원본 v0보다 좁다: parcel, LINK, EMIT, 공개 tape, draft 제외 | 조상은 이것들을 쓰지 않는다. 첫 질문에 필요 없는 상태를 미룬다 |
| D2 | 모든 효과 opcode(WRITE, COPY 포함)는 제출 후 yield | 원본은 22..28만 명시. tick당 효과 1개와 일관 |
| D3 | 배아 상태 번호 None 0, Writing 1, SealedWork 2, Invalid 3, Ready 4 | 원본에 번호 없음 |
| D4 | 자식 방향 = 출생 방향(절대) | 원본에 없음 |
| D5 | 이동 마찰 열은 원천 tile로 | 원본은 "현재 위치"로만 표현 |
| D6 | Recombine: 부모 상속 genome의 module로 대상 module 내용을 덮어씀 | M0에 learned·draft 없음. 원본은 결과 형태 미정 |
| D7 | 조상의 대기 잔여를 mem[7] 대신 r13에 둠 | 대기 비용 24q → 12q/tick. 원본 §57의 16q 예산 가정에 더 가까움 |
| D8 | 유지비는 stage1에 직접 반영(VM 열만 stage3까지 보류) | 원본의 overlay 의미와 같은 READ 결과. host 실패 시 실행 전체 정지 |
| D9 | 남의 inventory 인출 꺼짐, port를 통한 주기만 켬 | 원본 §13 "v1부터" |
| D10 | ActorSurface.orientation은 절대 방위 | Self.orientation도 절대값이라 추가 정보 아님 |
| D11 | Finish r5 > logical_len은 형식 실패 | 쓰지 않은 cell을 0으로 간주하지 않음(§55 취지) |
| D12 | Step 최종 검사비가 모자라면 다음 Step이 검사만 수행 | 원본에 없음 |
| D13 | CoreHash = BLAKE3(postcard(SimState)) | 원본의 필드별 LE 직렬화와 같은 목적. 형식이 하나라 단순 |
| D14 | WORK는 Surface와 tile Heat만(parcel 없음) | D1 |
| D15 | 화면은 Bevy 대신 로컬 웹 viewer(`gene serve`)로 먼저 제공 | 개인 사용, 컴파일·GPU 부담 없음. core는 화면과 독립이라 Bevy를 나중에 붙일 수 있음. 시뮬레이션 thread가 상태를 단독 소유하고 화면은 스냅샷만 읽음(원본 I07) |

## 20b. M1a: 신호·공개 기록·초안 (ruleset `gene-m1-1`)

M1a는 원본 v0..v1에 있던 정보 경로 일부를 켠다(원본 §21, §22, §52, §55). 물리 법칙은 M0와 같고, 꺼져 있던 primitive가 동작하게 된다. ruleset `gene-m0-1`로 실행하면 M0와 결과가 같아야 한다.

| 기능 | M0 | M1a |
| --- | --- | --- |
| EMIT, READ Signal(4) | InvalidTarget / BadAddress | 켬 |
| PublicTape(5): READ, WRITE Cell, WRITE Resize, COPY 원천·목적지 | 꺼짐 | 버려진 매체에 대해 켬 |
| OwnDraft(7): Resize로 생성, WRITE Cell, COPY 원천·목적지, CancelDraft | 꺼짐 | 켬 |
| 초안 → 배아 COPY | 없음 | 켬(자기 초안만) |
| 노출 매체의 bit 마모 | 없음 | 켬 |
| DraftCommit, learned binding | 꺼짐 | **꺼짐(M1b)** |
| parcel, LINK, LocalTrace | 꺼짐 | 꺼짐 |

**신호.** 4채널, tile별 signed 16bit.
- EMIT(r3 채널 0..3, r5 진폭 −255..255)는 시도비 128q에 2·|a|q를 더 내고 원자적으로 처리된다. 범위 밖은 InvalidTarget이다.
- 다음 tick의 장에 자기 tile은 a, 네 이웃은 trunc(a/2)를 더한다. 여러 기여는 i64로 합한 뒤 한 번만 [−32767, 32767]로 자른다.
- 전도가 없으므로(parcel 없음) 장은 매 tick 새로 방출된 값만 담는다. 비용은 열이 되고, 신호는 에너지 저장소가 아니다.
- READ Signal은 반경 2, field 0, r4 = 채널이며, i16 비트를 u16로 돌려준다.

**공개 기록.** 버려진 매체(사망·Cancel·변이 교체로 남은 기록)가 공개 기록이다.
- 같은 tile의 매체는 UID 오름차순으로 정렬하고, r6(READ, WRITE)·r7(COPY 원천)·r3(COPY 목적지)의 selector 1..n으로 고른다. selector 0(parcel 매체)은 M1a에 없다.
- READ는 반경 2이고 외부 query다. field는 0 cell, 1 logical_len, 2 capacity, 3 page_present, 4 recycle_remaining이다.
- WRITE·COPY 목적지·Resize는 접촉 반경 1이다.
- 외부 원천 COPY는 외부 query 하나를 쓰고 8q를 더 낸다. Think에서 bytes와 revision을 캡처하고, resolve 때 그 매체의 UID·revision이 바뀌었으면 Stale이다.
- 공개 매체 Resize의 증설과 hole 복구는 행동한 개체의 inventory G1과 128q를 쓴다. G1은 매체가 있는 tile로 옮겨지므로 열과 Ug를 정산한다. 축소 재활용은 32q/cell이고, 페이지를 마치면 G1이 그 tile의 loose로 돌아간다.
- 기록자의 UID로 권한을 주지 않는다. 누구든 읽고, 쓰고, 지울 수 있다.

**초안.** 개체당 하나의 사적 매체다.
- WRITE Resize(r0 = 7)로 첫 페이지를 만든다(G1 + 128q). 이후 Cell 쓰기는 16q, COPY는 32q/cell이다.
- 초안은 자기 code, 공개 기록, 자기 배아에서 COPY해 올 수 있다.
- 초안에서 배아로 COPY할 수 있다. 이것이 비혈연 코드가 다음 세대로 가는 경로다.
- 초안 페이지는 G와 유지비 1q/tick을 낸다. 죽거나 CancelDraft(WRITE 3)하면 그 tile에 버려진 매체로 남는다.

**bit 마모.** 버려진 매체는 due마다 exposure_clock에 X ≥ 64이면 4, 아니면 1을 더한다. 16384 이상이면 16384를 빼고, Present 페이지 하나, cell 하나, bit 하나를 균일 추첨해 뒤집는다. RNG는 Environment domain이고, object는 MediaUID, ordinal은 (2 << 56) | 마모 사건 번호다. 외부 화학 뒤에 처리한다.

**관찰.** EMIT 수, Signal READ 수, 공개 기록 READ와 COPY 수, 공개 기록 쓰기 수, 초안 → 배아 COPY 수를 세고, 비혈연 코드가 섞인 출생을 기록한다.

## 20c. M1b: 학습 채택 (ruleset `gene-m1-2`)

M1b는 M1a에 DraftCommit과 learned binding을 더한다(원본 §21). 살아 있는 동안 초안의 코드를 자기 실행 코드로 받아들이는 경로다. M1a와 같은 seed라도, Commit을 쓰는 프로그램이 나타나는 순간부터 결과가 달라질 수 있다.

**DraftCommit(WRITE 2, r5 = 사용 prefix 길이 L).**
- 요구: 초안이 있고, L ≤ logical_len이며, prefix에 hole이 없어야 한다.
- 비용: 시도비 64q에 2q/cell을 원자적으로 더 낸다. 형식이 깨져도 비용은 낸다.
- 형식 실패는 Format이고, 상태는 바뀌지 않는다.
- 한도 초과는 Limits다: 활성 binding 64개, 실행 code 4096 word(binding된 module 합), slot 128개(retained 포함).
- 성공하면 PendingAdopt가 되고, stage11에 그 개체가 살아 있으면 채택된다.

**채택.**
- 초안 매체가 learned로 옮겨지고 OwnDraft는 None이 된다.
- binding은 learned의 tag 전부, 그리고 learned에 없는 inherited tag로 이루어진다. 같은 tag라면 learned가 우선한다.
- entry는 learned header의 entry_tag다.
- 기존 learned는 교체된다. 현재 pc나 call frame이 그것을 가리키면 retained로 남고, 아니면 그 tile에 버려진 매체로 남는다.
- retained는 참조가 사라진 stage11에 같은 방식으로 내려놓는다.

**실행.**
- 현재 module과 frame은 원래 slot을 계속 쓴다. 다음 CALL부터 새 binding을 조회한다.
- 빈 stack에서 RET(명시적이든 암묵적이든)하면 새 entry로 간다.

**감각과 복사.** OwnCode READ의 bank(r6)와 COPY 원천 bank(r7)는 0 inherited, 1 learned, 2 이상 retained다. 없는 bank는 BadAddress다.

**물질.** learned와 retained 페이지도 G와 유지비 1q/tick을 낸다. 죽으면 모두 그 tile에 버려진 매체로 남는다.

**생식.** 자식은 배아 raw에 실제로 쓴 bytes만 상속한다. learned는 자동으로 복사되지 않는다. 구조 변이 Recombine의 donor에는 inherited와 learned module이 포함된다.

## 21. 확장 규칙과 로드맵

### 증거 규칙 (원본 §62를 강제 규칙으로 승격)

새 물리, 새 primitive, 새 capability는 다음을 모두 갖춰야 들어온다.

1. 기록된 실행에서 그것이 없어서 막힌 경로가 관찰됨(seed, tick, 원인)
2. 추가하지 않는 대안(기존 primitive 조합)을 검토한 기록
3. 새 ruleset 번호, 그리고 이전 결과와 비교할 ablation 실행군
4. 시간당 세대 수의 변화량 측정

### 단계

| 단계 | 내용 | 진입 조건 |
| --- | --- | --- |
| M0 | 이 명세 | — |
| E1 | 조상 생존 실험 | M0 시험 통과 |
| M1 | parcel, LINK, EMIT, 공개 tape, draft·learned(원본 v0..v1의 나머지) | E1에서 지속 개체군 확인 |
| M2 | 병렬 Think, Bevy viewer, S2 성능 gate | M1 |
| 연구 부록 | 원본 Matter-1, R7..R10(형상 역학, 일반 화학, 발생, 자기·광학, 파동장, 핵, 양자, LawField, 분산) | 위 증거 규칙을 항목별로 통과 |
