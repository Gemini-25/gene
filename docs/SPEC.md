# Gene M0 현행 명세

ruleset `gene-m0-1` · 저장 형식 `GENEM001` · 2026-10-10

**기준 ruleset(2026-10-11부터): `gene-m2-4`** = M0 + M1 전체 + 수동 신호 전도 + LocalTrace(§20h) + 흙이 물을 붙잡음(§20i) + 나이 이틀마다 유지비 +1q/tick(§20j). `--ruleset m2d` 또는 `--ruleset current`. 명령줄의 기본값은 이전 실행을 재현할 수 있게 `gene-m0-1` 그대로다.

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

E1-b에서 Founder-5 개체군은 20개 seed 모두 49일을 살아남았지만 계속 줄었다. 신생아가 하루를 넘길 확률은 출생 시각에 따라 크게 달랐다(20 seed 합산: 새벽 49%, 오전 38%, 이른 오후 25%, 늦은 오후 0%). 그런데 Founder-5는 G ≥ 50을 기다리느라 출생의 95%가 오후에 몰렸다.

Founder-6(`assets/founder6.asm`)은 번식 시작 조건 하나만 바꾼다.

| 항목 | Founder-5 | Founder-6 |
| --- | --- | --- |
| 번식 시작 시각 | 빛이 오르는 중이거나 ≥ 24576 | 빛이 오르는 중(오전)일 때만 |
| G 문턱 | ≥ 50 | ≥ 30 |
| E 문턱 | ≥ 60000 | ≥ 50000 |

오후에는 번식을 시작하지 않고 밤 비축을 남긴다. 나머지는 Founder-5와 같다. 248 word, 520 cell, 5 페이지.

### 16d. 공개 조상 Founder-7 (E1-c 중간 관찰 이후 개정판)

E1-c와 소규모 예비 실험(P1)에서 탄소가 한쪽에 쌓였다. 죽은 몸의 G는 그 tile의 loose G로 돌아간다. 생존 온도(270..329 K)에서 Rx2의 속도는 1이라 분해가 거의 없다. 그래서 loose G와 버려진 코드 기록이 쌓이고 공기의 C는 줄었다. E1-c seed 17에서는 27일 동안 loose G가 39만에서 163만으로 늘고, C는 418만에서 246만으로 줄었다. 같은 기간 개체당 광합성은 약 15% 줄었다.

Founder-7(`assets/founder7.asm`)은 Founder-6에서 G가 모자랄 때의 행동 하나만 바꾼다.

| 항목 | Founder-6 | Founder-7 |
| --- | --- | --- |
| inventory G < 30일 때 | 32 tick 대기 | 자기·앞·오른쪽·뒤·왼쪽 tile의 loose G를 차례로 READ하고, 처음 찾은 곳에서 최대 8을 TRANSFER한다. 없을 때만 32 tick 대기 |

가져온 G는 호흡(Rx1)으로 energy가 되고 탄소는 C로 공기에 돌아간다. 새 규칙이나 숨은 보조는 없다. 기존 READ와 TRANSFER만 쓴다. 291 word, 606 cell, 5 페이지.

소규모 점검(128², 개체 64, 100,000 tick, seed 1..4, `docs/results/E1.md`): 마지막 개체 수가 Founder-6은 58..213, Founder-7은 187..425였다. 21일째 loose G 중앙값은 Founder-6이 7.7만, Founder-7이 3.0만이었다.

### 16e. E2 공개 조상 Founder-8, -9, -10 (사용자 결정 2)

E2는 기록·신호·구조물을 하나씩 쓰는 조상들을 한 세계에 둔다. 셋 다 Founder-7에 습관 하나만 더했다. 더한 습관은 기존 primitive만 쓰고, 조상에게 준 행동이므로 성과로 세지 않는다(§17f의 새로움 규칙).

| 조상 | 쓰는 기능 | 더한 습관 | 크기 |
| --- | --- | --- | --- |
| Founder-8 (`founder8.asm`) | 신호(EMIT, Signal READ) | G가 17 이상 있던 tile에서 G를 가져오면 channel 0에 진폭 48로 먹이 신호를 낸다. S나 G를 못 찾아 돌아다니거나 잘 때가 되면, 먼저 앞·오른쪽·뒤·왼쪽 2칸의 channel 0을 듣고 가장 큰 신호 쪽으로 한 칸 간다 | 349 word, 722 cell |
| Founder-9 (`founder9.asm`) | 공개 기록, 초안, DraftCommit | 부유하고(energy 50000 이상, G·S 충분) 오전이 아닐 때 쉬는 8번에 한 번, 자기 tile과 앞 tile의 공개 기록 앞 두 개를 본다. 256 cell 이상이고 자기 genome과 길이가 다른 첫 기록을 초안에 복사해 commit한다. 그 코드가 learned로 실행된다. 일생에 한 번만 시도한다. 자식은 Founder-9 code를 물려받는다 | 397 word, 818 cell |
| Founder-10 (`founder10.asm`) | parcel(TRANSFER Matrix, WORK Compress) | energy 60000 이상(50000은 자식 몫)이고 오전이 아닐 때 쉬면서, 들고 있는 S 가운데 2개를 등 뒤 tile의 matrix에 넣고(없으면 parcel이 생김) 512q로 압축한다. 그 조각이 S 16을 담으면 멈춘다. S가 4보다 적으면 자기 tile의 loose S만 줍고, 재료를 찾아 걷지 않는다 | 340 word, 704 cell |

설계 중 고친 것:
- Founder-10의 첫 판은 S를 모으러 걸었다. 걷는 동안 광합성이 줄어 밤을 넘기지 못했고, 소규모 점검에서 개체가 18까지 줄었다. 그래서 걷지 않게 바꿨다.
- Founder-9의 첫 판은 자기 tile의 기록 하나만 봤다. 그래서 거의 배우지 못했다.

**소규모 점검**(128², 개체 64, 100,000 tick, `gene-m2-1`, seed 1..4)

| 조상 | 마지막 개체 수 | 출생 | 습관 사용(seed별) |
| --- | --- | --- | --- |
| Founder-7 | 272, 462, 322, 424 | 4,246..6,768 | 변이체가 EMIT를 쓴 seed 2개, parcel을 만든 seed 1개 |
| Founder-8 | 218, 423, 337, 342 | 3,052..5,091 | EMIT 11만..20만 |
| Founder-9 | 133, 365, 208, 273 | 2,183..4,342 | 채택 551..1,070 |
| Founder-10 | 241, 391, 316, 363 | 3,735..5,549 | 마지막 parcel 60..73 |
| 넷을 섞음 | 261, 387, 289, 365 | 3,843..5,721 | 24일째 Founder-9는 seed 2개에서 사라지고, Founder-10은 seed 1에서 2개체만 남음 |

세 조상 모두 혼자 살아남는다. 코드가 길수록 번식 복사와 유지비가 커서 Founder-7보다 조금 약하다. Founder-9가 가장 약하다(채택 비용 약 2만 q와 learned 페이지 유지비). E2 본 실험은 Founder-7 없이 Founder-8, -9, -10만 섞는다(`--founder founder8+founder9+founder10`).

### 16f. 습관을 module로 나눈 E2 조상 Founder-8m, -9m, -10m

**동기.** E2 예비 실험에서 Founder-8..10은 코드가 module 하나였다. 그래서 재조합 변이(§12 Recombine: module 하나의 내용을 다른 module 내용으로 덮어씀, tag 유지)가 일어나도 프로그램 전체가 바뀔 뿐 습관이 섞이지 않았다. 사용자 결정 2의 "변이가 병목이면 module 재조합" 단계로 습관을 module로 나눴다.

**구조.** 세 조상 모두 module 4개다.
- module 0(본체)은 셋이 bytes까지 같다. Founder-7에 hook 세 곳을 넣었다.
  - G를 가져온 뒤 `CALL 1`. r11에 그 tile에 있던 G를 둔다.
  - 길을 잃었을 때(S나 G를 못 찾아 돌아다니거나 자려 할 때) `CALL 2`. r5에 방향 0..3이나 4(없음)를 받는다.
  - 오전이 아닌데 부유하게 쉴 때 `CALL 3`.
- module 1..3은 습관이다. 자기 습관이 아닌 자리는 바로 RET하는 빈 module이다.

| 조상 | module 1 (먹이를 찾은 뒤) | module 2 (길을 잃었을 때) | module 3 (부유하게 쉴 때) | 크기 |
| --- | --- | --- | --- | --- |
| Founder-8m | 먹이 신호 | 신호 듣고 방향 고르기 | 빈 module | 356 word, 760 cell |
| Founder-9m | 빈 module | 빈 module(r5 = 4) | 기록에서 배우기 | 415 word, 878 cell |
| Founder-10m | 빈 module | 빈 module(r5 = 4) | 벽 짓기 | 358 word, 764 cell |

습관의 내용은 Founder-8..10과 같다. 다만 Founder-9m은 배운 뒤 RET으로 옛 본체에 돌아간다. 본체는 빈 stack에서 RET하지 않으므로 계속 상속받은 본체가 돈다. 다음 CALL부터는 배운 코드의 hook module이 묶인다.

**조합이 생기는 길.**
- Founder-9m이 다른 종류의 genome을 배우면, 그 hook module이 재조합의 donor가 된다(§20c).
- 그래서 그 자식은 재조합 한 번으로 예를 들어 "기록 학습 + 먹이 신호"를 함께 가질 수 있다.
- 같은 종류 안에서는 DuplicateModule과 재조합이 습관을 다른 hook 자리로 옮길 수 있다.

**관찰.**
- `births.csv`의 `modules` 열에 module마다 tag와 내용 hash(BLAKE3 앞 4 byte)를 적는다.
- `gene founders`가 조상 module hash를 보여 준다.
- `scripts/e2_mix.py`는 출생 genome의 hook module이 둘 이상의 조상에게서 왔거나 계통과 다른 조상에게서 왔으면 습관 조합으로 센다.

## 17. 관찰과 출력

관찰은 세계를 바꾸지 않는다. 헤드리스 실행이 내는 파일:

| 파일 | 내용 |
| --- | --- |
| `stats.csv` | 주기별 개체 수, 출생, 사망 원인, 평균 energy·inventory G, 배아 상태, 변이 사건, 고유 genome 수, 최대 세대, 장부 잔차, 서식지 W·C. 마지막 열 `provisioned`는 부모가 배아 저장소에 넣은 물질 단위다(출생 때 자식 inventory로 감). 어떤 조상도 하지 않는 행동이라, 진화로 생기는지 보는 표지다 |
| `births.csv` | tick, 자식, 부모, 세대, genome hash, 길이, 변이 여부, 위치, 태어날 때 든 G(`inv_g`) |
| `deaths.csv` | tick, uid, 원인, 나이, 자식 수 |
| `map_<tick>.png` | 고도, 물, 개체(계통 색) |
| `summary.json` | 실행 설정, 최종 상태, hash, 성능 |

### 17b. 행동 family (원본 §58, 분석 버전 A1.4)

관찰기 전용이다. 결과가 세계로 돌아가지 않으며, 켜고 꺼도 CoreHash가 같다.

**token.** 확정된 효과 하나가 8 byte token이 된다. 성공량 0인 결과와 실패도 token을 남긴다.

| byte | 내용 |
| --- | --- |
| 0 | 종류 = (opcode − 0x20) << 3 \| sub-op. sub-op가 8 이상이면 255. 단 sub-op를 쓰지 않는 효과(LINK, UNLINK, TRANSFER, EMIT, COPY)는 sub-op 0으로 적는다. 변이로 바뀐 쓰이지 않는 즉시값 때문에 같은 동작이 알 수 없는 종류가 되지 않게 한다(E2 본 실험 seed 1의 우세 계통은 `TRANSFER 16`, 번식 복사 연쇄는 COPY 즉시값이 8 이상이었다) |
| 1 | 결과(Outcome) |
| 2 | 행위자 기준 방향 0..3(앞, 오른쪽, 뒤, 왼쪽). 없으면 255 |
| 3 | channel: TRANSFER 종(0..9, energy 15), EMIT 신호 channel, LINK 대상(면 0..3, anchor 4, grip 5). 없으면 255 |
| 4 | magnitude = min(16, floor(log2(1 + 실제량))) |
| 5 | 같은 개체의 직전 효과와의 tick 차이 bin: 0, 1, 2..3, 4..7, 8..15, 16 이상 → 0..5 |
| 6 | flags: bit0 = 창의 첫 token(이때 지연 bin은 0) |
| 7 | 0 |

**sketch.** 창은 [256k, 256(k+1)) tick이다.
- 창 안 token의 연속 3개(24 byte) 앞에 `behavior-a1.4\0`을 붙여 BLAKE3 hash하고 앞 8 byte를 LE u64로 읽는다.
- 서로 다른 hash 중 가장 작은 128개가 sketch다.
- token이 3개 미만인 창은 ShortTrace로 세고 family를 만들지 않는다.

**비교.** 두 sketch의 합집합에서 가장 작은 min(128, |합집합|)개를 U로 잡는다. U 중 양쪽에 다 있는 개수가 I다. Jhat = I/|U|이며, 판정은 정수 비교(4I ≥ 3|U|, 4I < |U|)로 한다.

**family.** 창 하나를 후보 family의 대표 sketch와 비교한다.
- 후보: archive가 1536개 이하이면 전부다. 그보다 많으면 다음을 합친다(중복 제거).
  - 창의 가장 작은 hash 8개를 공유하는 family. 공유 수 내림차순, 최대 1024개.
  - 같은 context의 고정 hash 표본 256개.
  - 전체 archive의 고정 hash 표본 256개.
  - 고정 hash 표본은 family id의 BLAKE3(`behavior-sample\0` + id LE u32) 앞 8 byte가 가장 작은 256개다. family는 처음 나타난 context마다 그 context의 표본에 들어갈 기회를 얻는다.
- 가장 좋은 후보의 Jhat ≥ 3/4이면 그 family의 창이 된다. 동률은 오래된 family다.
- 모든 후보와 Jhat < 1/4이면 새 family를 만든다(novel). 대표 sketch는 처음 창의 것이고 바꾸지 않는다.
- 그 사이면 intermediate로 세고 억지로 분류하지 않는다.
- 창 3개 이상, 서로 다른 개체 2 이상이 되면 RepeatedBehavior다.

**출력.**
- `behaviors.csv`: 창 3개 이상인 family. 처음 tick, 처음 개체, 창 수, intermediate 수, 개체 수, context 수, 반복 확정 tick, 대표 token 예시.
- `summary.json`의 `effects.behavior`: 창·short·matched·novel·intermediate 수, family 수, 반복 family 수.

**사용 상태(원본 §58 기술 상태).** 반복 family마다 마지막으로 나타난 tick으로 정한다: 4096 tick 안이면 LiveUsers(사용 중), 3 × 4096 tick 안이면 Fading(뜸해짐), 그보다 오래면 NotObserved(관찰 안 됨). 다시 나타나면 LiveUsers로 돌아온다. 행동 증거만 쓴다. 기록이나 구조물만 남은 상태(DormantRecord, ArtifactOnly)는 §17d, §17e의 출력으로 따로 본다. `behaviors.csv`의 state_at_end 열과 화면 배지로 보인다.

**context(원본 §58).** 창이 닫힐 때(256 tick마다) 개체 자리를 본다. 분석용 입력이며 개체에게 새 감각을 주지 않는다.

| 항목 | 값 | bin |
| --- | --- | --- |
| 먹이 | 개체 tile의 loose G + residue G | 0 / 1..3 / 4..15 / 16..63 / 64 이상 |
| 빛 | 그 tick의 tile 빛 | 0 / 1..8191 / 8192..32767 / 32768 이상 |
| 혼잡 | 5×5 안의 다른 개체 수 | 0..2 / 3..6 / 7 이상 |
| 이동 | 창 안 성공한 Translate 수 | 0 / 1..7 / 8..31 / 32 이상 |

- context id = ((먹이 × 4 + 빛) × 3 + 혼잡) × 4 + 이동, 0..239.
- 창이 닫히기 전에 죽은 개체는 context를 모른다. 그 창은 context 표본 없이 비교한다. 원본은 마지막 관측의 context를 쓰지만, 여기서는 그 기록을 따로 두지 않는다.
- `behaviors.csv`의 `contexts` 열은 family의 창이 나타난 서로 다른 context 수다. 화면의 반복 행동 목록에도 "상황 N"으로 보인다.
- 고수준 판정(§17f)의 상황은 이것과 따로, 하루의 때 × energy × 발밑의 물로 본다.

### 17c. 관계 graph와 집단 (원본 §58, §29)

관찰기 전용이다. 결과가 세계로 돌아가지 않는다(켜도 같은 seed의 CSV가 같음을 확인).

**node.** (종류, UID). 개체 0, parcel 1, 매체 2.

**사실.** 성공한 효과에서만 만든다. 단순 근접, READ, 같은 계통은 관계가 아니다.

| 종류 | 사건 | 방향과 양 |
| --- | --- | --- |
| 자원 0 | 개체 사이 TRANSFER(주기, M1e의 꺼내기), 에너지 주기 | 준 쪽 → 받은 쪽. 물질은 질량(종별 channel), 에너지는 q(channel 15) |
| 기록 1 | 공개 매체에서 COPY해 옴 / 공개 매체로 COPY함 | 매체 → 개체 / 개체 → 매체. 복사한 cell 수 |
| 구조물 2 | parcel matrix·pore TRANSFER, LINK·UNLINK, parcel WORK, parcel 매체 WRITE·Resize·COPY, 조립체 이동(하중이 들어간 parcel) | 개체 → parcel. 성공 1회당 1 |

**창과 weight.** 4096 tick 창마다 (from, to, 종류)별로 먼저 합한 뒤 weight = min(255, floor(log2(1 + 합)))로 바꾼다. 한 번의 이전을 작게 쪼개도 weight가 커지지 않는다. edge마다 마지막 4개 창만 남긴다.

**집단 탐지(LPA).**
- 무방향 weight = 남은 4개 창, 두 방향, 모든 종류의 합.
- 처음 label은 자기 node key다. pass 순서는 BLAKE3(`group-a1.4\0`, 창 번호 LE, node 종류, UID LE)의 앞 8 byte, 다음 node key다.
- 이웃 label별 weight 합이 가장 큰 label을 고른다. 동률이면 지금 label, 그다음 작은 key다. 최대 20 pass이고 변화가 없으면 멈춘다.
- 창 끝에 살아 있는 개체가 2 이상인 label만 집단이다. 죽은 개체 node는 전파에는 남고 개체 수에는 들지 않는다.

**집단 정보.** 살아 있는 개체, parcel·매체 node, 내부 edge 종류(bit), 내부 weight 합, 지속 창 수, 경계 구성원 수(자기 label 이웃 weight 비율 < 0.6), HubDependent(모든 내부 edge가 같은 parcel·매체 하나를 지남).

**CivilizationLikeCandidate.** 살아 있는 개체 20 이상, 3창 연속 지속, 내부 관계 종류 2 이상. 개체 수 20은 화면 잡음 기준이지 세계 규칙이 아니다.

**ID 유지.** 이전 집단과 새 집단의 개체 Jaccard ≥ 1/2인 쌍을 점수 내림차순(정수 교차 곱 비교), 이전 ID 오름차순으로 일대일 연결한다. 연결되지 않은 새 집단은 새 ID를 받고, 이전 집단과 개체가 겹치면 split(가장 많이 겹치는 쪽이 parent)이다. 연결되지 않은 이전 집단의 개체 과반이 다른 집단으로 갔으면 merge, 아니면 ended다.

**출력.**
- `groups.csv`: 창마다 집단 하나씩. window, tick, id, agents, parcels, media, kinds(R 자원, C 기록, U 구조물), weight, windows, border, hub, civ, parent.
- `summary.json`의 `relations`: 종류별 총량, 지금 집단 수와 최대 크기, 생긴 집단·split·merge·문명 후보 수.
- 화면: 색 "집단", 집단 목록과 최근 사건.

**구조물 사실의 행동 종류.** 구조물(Use) 사실의 channel은 개체가 한 일이다: 0 matrix 넣기, 1 matrix 꺼내기, 2 pore, 3 LINK, 4 UNLINK, 5 압축, 6 깎기·가열, 7 매체 쓰기, 8 조립체 이동. 0, 3, 5가 "만들기"다(§17d).

**영토(원본 §29).** 16×16 tile 칸마다 집단별 heatmap 셋을 쌓는다: 방문(256 tick마다 구성원 위치 표본), 물질(자원 사실의 양, 주고받은 양쪽 집단 모두), 구조물(구조물 사실 횟수). 사실의 장소는 구조물이면 parcel tile, 아니면 행동한 개체의 tile이다. 소속은 마지막으로 닫힌 창의 집단이다. 창이 닫힐 때마다 이전 값은 floor(3/4)로 줄고 새 관측이 더해진다. 칸 합계의 0.6 이상을 가진 집단이 있을 때만 그 칸을 그 집단 색으로 칠한다. 화면 표시용이며 세계의 허가 정보가 아니다.

복사 계보는 §17e다.

### 17d. 구조물(artifact)의 동일성과 반복 (원본 §58)

관찰기 전용이다.

**모양.** parcel 하나를 중심으로 Manhattan 2 이내의 13 tile을 dy 오름차순, 같은 dy에서 dx 오름차순으로 적는다. tile마다 30 byte다.

| byte | 내용 |
| --- | --- |
| 0 | 경계 밖 |
| 1 | parcel 있음 |
| 2..11 | matrix 종별 양 bin |
| 12..21 | pore 종별 양 bin |
| 22 | c >> 4 |
| 23 | orient |
| 24..27 | 면 N, E, S, W 결합 강도 bin |
| 28 | anchor bin |
| 29 | 공개 기록 있음(버려진 기록 또는 parcel 매체) |

bin = min(16, floor(log2(1 + n)))이다. parcel이 없는 tile의 parcel 필드는 0이다. 좌표, UID, 만든 개체는 넣지 않는다. 회전한 구조물은 다른 모양이다. 모양 hash는 BLAKE3(`artifact-a1.4\0` + 390 byte)의 앞 8 byte다.

**occurrence.** (모양 hash, 그 13 tile 안 parcel UID 집합)이다. 같은 parcel들을 다시 보는 것은 새 제작이 아니다. 4096 tick마다(그리고 실행 끝에) 모든 parcel을 중심으로 본다.

**반복.** 모양마다 처음 본 순서대로, 이미 센 occurrence와 parcel을 하나도 공유하지 않는 occurrence만 독립으로 센다. 독립 occurrence의 parcel을 "만든"(matrix 넣기, 압축, LINK) 개체가 기여자다. 가장 가까이 있던 개체를 제작자로 정하지 않는다. 독립 3곳 이상, 기여자 2 이상이면 반복 구조물이다. 기여자가 하나도 없으면 NaturalOrigin이다. parcel 하나뿐이고 결합·압축·기록이 없는 모양은 trivial로 따로 센다.

**출력.**
- `artifacts.csv`: 독립 2곳 이상인 모양. pattern, first_tick, parcels, bonds, trivial, occurrences, independent, contributors, natural, repeated_at.
- `summary.json`의 `relations`: artifact_patterns, repeated_artifacts, repeated_trivial.
- 화면: "반복되는 구조물" 목록과 13칸 썸네일.

### 17e. 복사 계보와 전달 증거 (원본 §58)

관찰기 전용이다. 매체마다 cell 구간 [start, end)별로 그 구간을 마지막으로 쓴 사실을 둔다.

| 사실 | 구간의 출처 |
| --- | --- |
| WRITE Cell | Literal(쓴 개체) |
| COPY | Copied(원천 매체, 복사한 개체, 처음 저자). 처음 저자는 복사할 때 원천 구간의 저자로 정한다. 원천이 자기 code·자기 배아이고 기록된 출처가 없으면 그 개체의 code가 저자다. 원천이 다른 개체의 genome 매체(살아 있거나 죽어 버려진 것)이고 기록된 출처가 없으면 그 genome의 주인이 저자다(복제 사본은 메모리 때문에 추적하지 않으므로) |
| bit 마모 | Damaged |
| 재활용으로 0을 씀 | Erased |
| 산화로 페이지가 사라짐 | Destroyed |

- 나중 사실이 겹치는 구간만 바꾼다. 매체가 버려지거나 주인이 바뀌어도 출처는 유지한다.
- READ는 출처를 만들지 않는다. 같은 bytes를 따로 쓴 것은 복사가 아니다(Literal).

**전달.** 복사 → 채택(DraftCommit 후 learned) 또는 출생(배아 기록이 자식의 code) → 실행이 모두 있어야 한다.
- 실행은 256 tick마다 표본으로 본다: 지금 VM이 도는 module의 매체가 learned·retained 매체이거나, 공개 기록 출처가 섞인 code다.
- 실행된 매체의 사용 구간(learned는 commit 길이) 안에 저자가 다른 개체인 cell이 있으면 (개체, 매체, 저자)마다 한 번 Transfer를 기록한다.
- 저자가 그 개체의 부모면 kin, 아니면 non-kin이다. 경로는 learned(채택) 또는 birth(배아 기록)다.

**출력.**
- `lineage.csv`: tick, agent, media, author, kind(kin/non-kin), cells, via(learned/birth).
- `summary.json`의 `relations`: record_writes, record_copies, copied_cells, bit_damage, transfers_kin, transfers_nonkin.

**원본과의 차이.** module 단위가 아니라 사용 구간 전체로 실행 증거를 본다. 기술 상태는 행동 family에만 붙였다(§17b).

### 17f. 고수준 행동 판정 (사용자 결정 3, 2026-10-10)

관찰기 전용이다. 행동 family(§17b)의 창 기록, 출생·사망, 관계 사실(§17c)을 받아 실행 끝에 판정한다. 원본에는 없는 층이다. 판정 결과는 후보일 뿐이고, 마지막 확인은 화면에서 사람이 한다(결정 3의 D).

**창 기록.** 창(256 tick)이 닫힐 때마다 (개체, 창, family 또는 없음, 성공 token 수, 전체 token 수, 결과물 여부)를 남긴다.
- 성공 token: 효과 결과가 Ok 또는 Partial.
- 결과물: 성공한 효과가 개체 밖의 세계를 바꿨다. 물질 이동(TRANSFER, 실제량 > 0), 밖을 향한 WORK, LINK·UNLINK, 밀기(DRIVE 2, 3), EMIT, 자식 내보내기(REPRO Release), 또는 그 창에 그 개체가 주체인 관계 사실(공개 기록, 구조물, 다른 개체). 읽기, 걷기, 돌기, 자기 기록 쓰기는 결과물이 아니다.

**B. 거르기(family).** 반복 family(§17b)마다 다음 두 가지를 모두 만족해야 통과한다.
- 성공: 성공 token이 전체의 절반 이상.
- 성과와의 연결: 운반자(그 family의 창이 하나라도 있는 개체)가 5 이상이고, 그들의 "살아 있던 tick당 자식 수"가 기준 집단의 1.1배 이상이다. 기준 집단은 어떤 family에든 창이 하나라도 있는 모든 개체다. 갓 태어나 곧 죽는 개체를 기준에서 빼서 모든 family가 유리해 보이지 않게 한다. 또는 다른 계통으로 퍼졌다: 운반자가 3 이상인 조상 유형이 둘 이상이고(운반자 두셋의 우연한 겹침은 퍼짐이 아니다), 운반자의 성과 비가 1 이상이다. 본체 code가 같은 조상들은 같은 실패(자식이 없는 번식 실패 반복)에 각자 빠질 수 있다. 그것은 퍼짐이 아니다(module형 조상 장기 실행 seed 2에서 이런 family 3개가 통과했다).

**C. 계층(연쇄).** 한 개체의 연속된 창에서 서로 다른 family가 이어지면 연쇄 하나다. 길이 2와 3을 센다. 연쇄가 고수준 후보가 되려면 다음을 모두 만족해야 한다.
- 모든 family가 반복 family이고 B를 통과한다.
- 개체 5 이상이 했다.
- 서로 다른 상황 2가지 이상에서 시작했다. 상황이 바뀌어도 반복되는지 보는 대리 지표다.
  - 상황 = (하루의 때 4등분: 새벽·오전, 오후, 저녁, 밤) × (그 개체의 energy: 20000 미만, 50000 미만, 그 이상) × (발밑 tile에 물이나 얼음이 있는가).
  - energy와 발밑은 창이 닫힐 때(256 tick마다) 본다. 그 사이에 죽은 개체의 창은 상황을 모르므로 세지 않는다.
- 발생의 절반 이상이 결과물을 남겼다.

**새로움.** 조상에게 준 행동은 성과로 세지 않는다(결정 2).
- 조상 genome과 bytes가 정확히 같은 개체를 "조상 code"라 한다.
- family나 연쇄의 창(발생) 가운데 10분의 1 이상이 조상 code 개체에서 나오면 조상의 것이다. 조상 code가 어떤 상황에서든 하는 행동은 새것이 아니다. 처음 나타난 날로 정하면 조상 행동의 다른 상황판을 새것으로 잡는다.
- 동작 순서로도 본다. 창의 성공한 효과 3개가 잇달아 일어난 것을 동작 3-gram이라 한다. 효과 종류(opcode, sub-op)만 보고 방향, 양, 시간 간격은 무시한다. sub-op를 쓰지 않는 효과(LINK, UNLINK, TRANSFER, EMIT, COPY)는 sub-op를 0으로 본다. 변이로 바뀐 쓰이지 않는 즉시값은 새 동작이 아니다(장기 실행에서 COPY 4로 표시된 계통이 새 family로 잡혔다). family 예시의 동작 3-gram이 모두 조상 code 창에서 본 것이면 그 family는 조상의 것이다. 조상의 먹이 줍기를 다른 방향과 간격으로 하는 변이체가 새 family로 잡히는 것을 막는다(E2 예비 실험에서 새 family 후보 대부분이 이런 것이었다).
- 동사로도 본다. 동사는 성공한 효과가 "무엇을 어디서 어디로" 했는지다: 효과 종류(sub-op를 쓰지 않으면 0), channel(물질 종, energy, 신호 channel, 결합 대상), 원천과 목적지 주소의 domain(TRANSFER·COPY는 r0·r1, READ·WRITE·WORK는 r0). 예를 들어 "TRANSFER G self>embryo"(배아에 G 넣기)는 "TRANSFER energy self>embryo"(조상의 에너지 투자)나 "TRANSFER G loose>self"(먹이 줍기)와 다른 동사다. 동작 종류와 순서만 보면 셋을 구분하지 못한다. 동사는 A1.4 token에 넣지 않고 따로 기록하므로 family 분류는 그대로다.
  - 조상 code 개체의 창(family가 없는 창 포함)에서 본 동사가 조상 동사다.
  - family의 창에 조상 동사가 아닌 동사가 있으면, 동작 3-gram이 모두 조상의 것이어도 그 family는 조상의 것이 아니다(조상 code 창 비율 규칙은 그대로). 단, 그 동사가 family 창의 10분의 1 이상, 그리고 3개 이상의 창에 있어야 한다. E2 본 실험 seed 3에서 운반자 3,123의 먹이 줍기 family가 "G를 땅에서 parcel로" 동사 때문에 새것으로 잡혔는데, 끝 4일 동안 그 동사는 한 번도 나오지 않았다. 흔한 조상 루틴 안의 드문 한 창은 그 family가 하는 일이 아니다.
  - `judge.csv`, `highlevel.csv`의 `new_verbs` 열과 화면의 "새 동사"에 그런 동사를 적는다. 순서만 새로운 후보와 새 동사를 쓰는 후보를 사람이 구분해 볼 수 있다.
  - 새 동사 가운데 효과의 명령과 원천·목적지 domain 조합을 조상 동사가 한 번도 쓰지 않은 것은 "새 대상"이라 하고 `*`를 붙인다(예: 다른 개체에게 물질 주기 `TRANSFER G self>actor`). 표시가 없으면 조상 동사의 channel이나 sub-op만 다른 변형이다(예: 먹이 신호를 channel 0 대신 3에 냄, 배아에 energy 대신 G 넣기).
- 연쇄 안의 family 하나라도 조상의 것이 아니면 새 고수준 후보다.
- 모든 family가 조상의 것이면, 연쇄가 조상 code에서 거의 나오지 않을 때만 새것이다. 그리고 단계마다 하는 일이 조상 code의 어떤 연쇄와도 달라야 한다. 단계가 하는 일은 그 family 예시의 성공한 동작 종류 집합이고, 순서·간격·실패·반복은 뺀다. 조상의 번식 연쇄에 실패한 명령 하나가 끼어든 변이체는 새것이 아니다.

**출력.**
- `judge.csv`: 반복 family마다 성공·전체 token, 운반자 수, 성과 비(×1000), 조상 유형 수, 창 수, 조상 code 창 수, 조상의 것인지, 통과, 새 동사.
- `highlevel.csv`: 개체 2 이상이 한 연쇄마다 판정(pass 또는 처음 걸린 규칙), 새로움, 처음 시각, 발생 수, 조상 code 발생 수, 개체 수, 때 수, 상황 수, 결과물 발생 수, 개체 예, 새 동사, 각 단계 family의 예시 token. 통과한 것이 먼저 온다.
- `summary.json`의 `highlevel`: families_judged, families_pass, new_families_pass, chains_seen, chains_pass, new_chains_pass.
- 실행 중에도 `gene judge ckpt_<tick>.ana`로 체크포인트의 분석 상태를 판정해 볼 수 있다(새 family, 새 연쇄, 새 동사).

**한계.** 창 하나(256 tick) 안에서 끝나는 묶음은 연쇄가 아니라 family 하나로 잡힌다. 그런 새 행동은 B를 통과한 새 family로 센다. 상황은 위 세 가지로만 본다.

**이어 달리기.** `gene run --checkpoint-every C --resume`은 출력 폴더에 체크포인트가 있으면 가장 새것에서 이어 간다.
- 체크포인트(`ckpt_<tick>.bin`) 옆에 관찰 계수(`ckpt_<tick>.obs`: 누적 출생·사망, M1 계수, 효과 결과 수, 변이 통계)를 둔다. 둘 다 임시 파일에 쓴 뒤 이름을 바꾼다.
- 가장 새 둘만 남긴다.
- 이어 갈 때 `stats.csv`, `births.csv`, `deaths.csv`를 그 tick까지 잘라 이어 쓴다. 끊김 없는 실행과 byte 단위로 같고, core hash도 같다.
- 행동 분석(§17b)과 고수준 판정(§17f)의 상태도 체크포인트 옆(`ckpt_<tick>.ana`)에 저장하고 이어 간다. 결과 파일이 끊김 없는 실행과 같다.
- 관계·구조물·계보 분석은 이어 간 구간만 다룬다.
- `summary.json`의 `resumed_from`에 이어 간 tick을 적는다. 분석 상태 없이 이어 갔으면 `highlevel.partial`이 true다.

### 17g. 경쟁 시험: 진화한 genome이 조상보다 나은가

선택이 실제로 적응을 만드는지 재는 도구다. 새 고수준 행동이 남으려면 먼저 적응이 일어나야 한다.

- `--founder` 이름 자리에 `SAVE.bin@HASH`를 쓰면, 그 저장 상태에서 genome hash가 HASH(16진 앞자리)로 시작하는 개체의 genome이 조상이 된다.
- `scripts/compete.py SAVE.bin HASH[,HASH...]`는 seed마다 두 실행을 한다.
  - 시험: `조상+SAVE@HASH`. 조상 i는 program i mod 2를 쓰므로 짝수 uid가 조상, 홀수 uid가 진화한 genome이다.
  - 대조: `조상+조상`. 같은 seed라 처음 자리가 같고, 프로그램만 같다.
- 지표는 마지막에 살아 있는 개체 중 홀수 자리 계통의 몫에서 대조 실행의 같은 몫을 뺀 값(%p)이다. 같은 seed끼리 빼서 처음 자리의 운을 지운다. 같은 조상끼리도 seed에 따라 몫이 27..61%로 흩어지기 때문이다.
- 기본값: 128², 개체 64, 100,000 tick(24일), seed 1..4.

### 17h. 행동 변화: 진화한 genome이 조상과 무엇을 다르게 하나

§17f의 새로움은 동작 순서와 동사로 본다. 그래서 조상이 가진 동작을 새 상황에 쓰는 변화는 새것으로 잡히지 않는다. 예: E2 seed 1의 계통은 조상의 "고장 난 배아 취소" 루틴으로 변이된 배아를 버렸다. 그래서 행동의 빈도를 따로 비교한다.

- `scripts/habit_shift.py SAVE.bin HASH --founder NAME`: genome과 조상을 새 128² 세계(개체 64, founder i는 program i mod 2)에 넣고, 8,192 tick 뒤부터 32,768 tick 동안 성공한 효과를 동사(§17f)별로 개체-일당 센다(`examples/habits ... csv`). seed 여러 개를 평균한다.
- 표시: 조상은 하는데 genome은 안 하는 동사(dropped), 그 반대(taken up), 비율이 3배 이상 바뀐 동사. 하루 0.2번 미만인 동사는 뺀다.
- E2 seed 1의 7669ef53: REPRO Step 39 → 0(dropped), 걷기 ×8.1, G 줍기 ×3.4, 먹이 신호 0.38 → 0(dropped), 배아 취소 0 → 0.28(taken up).

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

## 18b. E2 본 실험: 섞인 module형 조상 (사용자 결정 2, 5..7)

**사전 고정 조건(2026-10-11).**
- 조상: Founder-8m, -9m, -10m(§16f)을 섞는다. 개체 249(각 83), 조상 i는 program i mod 3.
- 세계: 256², `gene-m2-4`(기준 ruleset), seed 1..4, 2,000,000 tick(약 488일). 체크포인트 131,072 tick마다, 끊기면 이어 간다.
- 분석: 행동 family와 고수준 판정(§17b, §17f), 관계·계보(§17c..§17e). 실행 도중 판정 규칙을 바꾸면 같은 저장에서 다시 판정하고 바뀐 것을 적는다.

**측정.**
- 조상 종류별 개체 수, 최대 세대, 신생아 하루 생존
- 습관 조합 출생(`scripts/e2_mix.py`), 비혈연 코드 전달(§17e), 배아에 물질을 넣는 출생(`provisioned`, `inv_g`)
- 판정기를 통과한 새 family와 새 연쇄. 각 후보는 재현해 추적하고(`trace_agent`) 화면에서 확인한다(결정 3의 D)
- 끝에 개체 수 상위 genome의 경쟁 시험(§17g, 같은 종류의 조상과 비교)

**미리 정한 해석.**

| 결과 | 뜻 | 다음 행동 |
| --- | --- | --- |
| 새 연쇄가 통과하고 사람이 확인 | 저수준 행동에서 새 고수준 행동이 생김 | 같은 조건 seed를 늘려 재현성 확인 |
| 새 family만 통과, 모두 조상 루틴의 순서 변형 | 조합의 재배열은 있으나 새 기능은 없음 | 기간·세계 크기를 늘리거나 환경 과제 추가 검토 |
| 습관 조합이 생겨 퍼짐 | module 재조합이 행동을 섞음 | 조합 계통의 행동을 추적 |
| 판정 후보 없음, 경쟁 시험에서 적응 | 기존 행동의 최적화만 일어남 | 같음 |
| 멸종 | 이 조건에서 개체군 유지 실패 | 원인 분해 |

## 19. 성능 목표 (§45)

| 시나리오 | 목표 |
| --- | --- |
| S1: 512², 1k 개체 | 500..3000 TPS(목표, 실측 공개) |
| S2: 512², 10k 개체 | 100 TPS 이상(M2 gate) |

기준 구현은 단일 thread 직렬이다. 병렬화는 같은 hash를 내는 경우에만 들인다.

- **수송(stage 7):** 같은 color의 edge는 endpoint를 공유하지 않으므로, 각 edge가 한 띠 안에 들어가도록 행 단위 띠로 나눠 병렬 처리한다(동쪽 edge는 아무 행 경계, 남쪽 edge는 color 2면 짝수 행 경계, color 3이면 홀수 행 경계).
- **Think(stage 2):** 각 VM은 공유 세계를 읽고 자기 개체만 쓴다. 그래서 개체 32 이상이면 개체 단위로 병렬 실행한다. 에너지 집계는 그 뒤 개체 순서대로 직렬로 더한다.
- **환경(stage 8):** 개체와 버려진 기록이 없는 due tile은 띠 단위로 병렬 처리하고, 나머지는 직렬 경로로 처리한다.
- residue가 있으면 직렬로 돌아간다.
- `threads` 값은 상태에 포함되지 않으며, 시험이 1 thread와 4 thread의 hash가 같음을 확인한다. 30,000 tick CLI 실행에서 `stats.csv`와 `births.csv`가 byte 단위로 같았다.

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
| D9 | 남의 inventory 인출 꺼짐, port를 통한 주기만 켬. M1e(§20f)에서 켬 | 원본 §13 "v1부터" |
| D10 | ActorSurface.orientation은 절대 방위 | Self.orientation도 절대값이라 추가 정보 아님 |
| D11 | Finish r5 > logical_len은 형식 실패 | 쓰지 않은 cell을 0으로 간주하지 않음(§55 취지) |
| D12 | Step 최종 검사비가 모자라면 다음 Step이 검사만 수행 | 원본에 없음 |
| D13 | CoreHash = BLAKE3(postcard(SimState)) | 원본의 필드별 LE 직렬화와 같은 목적. 형식이 하나라 단순 |
| D14 | WORK는 Surface와 tile Heat만(parcel 없음) | D1 |
| D15 | 화면은 Bevy 대신 로컬 웹 viewer(`gene serve`)로 먼저 제공 | 개인 사용, 컴파일·GPU 부담 없음. core는 화면과 독립이라 Bevy를 나중에 붙일 수 있음. 시뮬레이션 thread가 상태를 단독 소유하고 화면은 스냅샷만 읽음(원본 I07) |
| D16 | DRIVE sub-op 2 = PushParcel, 3 = PushActor | 원본은 2 = PushActor, 3 = PushParcel. M1d가 먼저 2를 PushParcel로 썼다. 이후 ruleset이 이전 ruleset의 상위 집합이 되도록 번호를 유지한다 |
| D17 | M1d의 PushParcel은 개체가 제자리에 남는다. M1e부터 원본대로 개체가 앞 tile로 따라간다 | M1d 결과를 바꾸지 않기 위해 새 ruleset에서만 바꾼다 |
| D18 | 마모가 대상 층에 남은 양을 넘는 일은 하지도, 내지도 않는다 | 원본은 층이 바뀌면 남은 credit을 0으로 한다. 넘는 일을 받아 버리면 요청량이 큰 개체가 이유 없이 손해를 본다. 원본 §10 "cap 때문에 실행하지 못한 일은 지불하지 않는다"와 같은 원칙 |

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

## 20d. M1c: 구조물 parcel (ruleset `gene-m1-3`)

M1c는 M1b에 parcel을 더한다(원본 §10, §11, §12, §16, §17, §18, §63). 결합(LINK/UNLINK), grip, anchor, 밀기, 조립체 이동, parcel 매체는 M1d로 미룬다.

**상태.** tile마다 parcel이 최대 하나다.
- 필드: ParcelUID, matrix[10](G, X, M, S, I만 허용, 유효 부피 합 s ≤ 64), pore[10], 압축도 c(0..255), 반응 credit.
- 투과도: P = max(1, 256 − floor(s·c/64)). parcel이 없으면 P = 256이다.
- pore 용량: K = 64 − ceil(s·c/256). pore의 유효 부피 합은 K 이하다.
- 높이: ceil(s/16). s ≥ 16이고 c ≥ 128이면 개체를 막는다(차단 parcel). 그 밖에는 개체와 같은 tile에 있을 수 있다.

**생성과 제거.**
- TRANSFER의 목적지가 Matrix(2)이고 그 tile에 parcel이 없으면, 새 parcel(c = 0)을 만들고 그 안에 넣는다. 고체가 아닌 종은 InvalidTarget이다.
- matrix에서 꺼내기는 c < 64일 때만 가능하다(아니면 Blocked).
- s가 0이 되면 pore 내용물은 loose로 돌아가고, parcel은 사라지며 credit도 함께 사라진다.
- 개체가 서 있는 tile을 차단 parcel로 만드는 행동(입고, 압축)은 Blocked다.

**pore 출입.** 개체의 능동 출입(TRANSFER Pore 10)은 그 parcel의 P ≥ 32일 때만 가능하다. 용량 K를 넘으면 Capacity다.

**WORK.**
- 대상: Parcel(2), r1·r2는 접촉 반경 1이다.
- Compress: 64q당 c + 1. Abrade: 256q당 c − 1.
- 압축 결과 K가 pore 내용물보다 작아지거나, 개체가 있는 tile이 차단되면 그 단위는 진행하지 않는다.

**노출과 수송.**
- X는 기존 식 그대로 P로 계산하고, parcel이 바뀌면 그 tile과 이웃 4칸의 X를 다시 계산한다.
- edge 수송의 minP는 두 tile의 P 중 작은 값이다.
- 열전도 κ에는 min(8, 두 matrix의 M 합)을 더한다.
- 그래서 차폐가 빛, 외기, 확산, 열전도를 함께 줄인다.

**자연 pore 교환(stage 7, 수송 전).** due parcel마다 기체 C, O, H, V 순서로, 이어서 W를 처리한다.
- d = n_loose·K − n_pore·64. d > 0이면 loose → pore, d < 0이면 반대 방향이다.
- 양 = min(32, SR(|d|·P, 64·2048))이고, W는 분모 64·8192를 쓴다.
- 원천 재고와 목적지 여유로 다시 제한한다. 같은 tile 안의 이동이므로 열과 Ug는 변하지 않는다.
- RNG는 FluxRng domain 9(PoreFlux), selector = 종, object = ParcelUID다.

**화학.** site 순서는 Loose → Matrix → Pore → PoreBoundary → 버려진 기록(UID 순)이다.

| site | 반응 | 반응물 출처 | 생성물 행선지 |
| --- | --- | --- | --- |
| Matrix | Rx2..Rx6 | 응축 반응물은 matrix, 기체는 tile loose | 고체는 matrix, 기체·액체는 loose |
| Pore | Rx2..Rx6 | pore 자기 재고 | pore |
| PoreBoundary | Rx2, Rx5, Rx6만 | 응축은 pore, 기체는 loose | 기체는 loose, 응축은 pore |

- 일반 site의 credit 증가는 SR(rate·X·min(16, extent), 256)이다.
- PoreBoundary는 SR(rate·X·P·min(16, extent), 65536)이다.
- 공간·용량 검사는 기존 규칙을 그대로 따른다.
- 반응 후 s나 c가 바뀌어 matrix 부피 상한을 넘을 수 있으면 그 extent는 거절한다.
- credit RNG는 domain 10, selector = (site_kind << 8) | 반응, object = ParcelUID다. site_kind는 Matrix 1, Pore 2, PoreBoundary 4다.

**감각.**
- Parcel READ field: 0 matrix[r4], 1 c, 2 방향(0), 3 bond(0), 4 anchor(0), 5 pore 용량, 6 present.
- PoreStock READ field: 0 재고[r4], 1 용량.
- 거리 2 감각은 두 축 순서의 최단 경로 중 하나라도 중간 tile이 열려 있으면(s < 16 또는 c < 128) 보인다. 둘 다 막혀 있으면 Occluded다.

**이동.**
- 차단 parcel이 있는 tile로는 이동하거나 출생할 수 없다(Blocked).
- 통과 가능한 parcel(s ≥ 4, c ≥ 64)의 마찰은 μparcel = max(64, μ − floor(c·s/16))이고, μeff에 포함된다.

**장부.** parcel의 matrix와 pore는 tile의 물질이다. 열용량, 질량·Ug, 원자 수, 엔탈피에 모두 들어간다.

## 20e. M1d: 결합·고정·grip·조립체 이동 (ruleset `gene-m1-4`)

M1d는 M1c에 결합과 조립체 이동을 더한다(원본 §11, §64). 개체끼리의 접촉(PushActor 포함)은 §20f(M1e), parcel 매체는 §20g(M1f)다.

**상태.** parcel마다 다음을 더한다.
- bond[4]: 절대 방위 면(N, E, S, W)별 결합 강도(0..255). 결합은 대칭이다. A의 면 f와 그 이웃 B의 면 f+2가 항상 같은 값이다.
- anchor: 땅과의 결합 강도.
- orient: 조립체 회전으로 누적된 quarter-turn.
- grip, grip_uid: 개체와의 결합 강도와 그 개체의 UID(0이면 없음). grip은 개체당 1개, parcel당 1개다.

**응집력.** b = min(255, Σ 응집 계수 × matrix 양). 계수는 G 3, X 4, M 6, S 4, I 1이다.
- 면 결합 상한: min(bA, bB)
- anchor와 grip 상한: b
- matrix가 바뀌어 b가 줄면(꺼내기, 화학) 그 parcel의 모든 결합을 새 상한으로 줄인다. 상대편도 같이 줄인다.
- parcel이 사라지면 이웃의 마주 보는 면 결합이 0이 되고 grip도 사라진다.
- 개체가 죽으면 그 개체의 grip이 풀린다.

**LINK(0x23)와 UNLINK(0x24).** r0 = 2(Parcel), (r1, r2) = 대상 parcel의 상대 좌표(접촉 반경 1), r4 = 면 0..3 / anchor 4 / grip 5, r5 = 강도 변화량.

| 대상 | 조건 |
| --- | --- |
| 면 0..3 | 대상의 면 r4 너머 이웃 parcel이 상대다. 두 끝이 모두 개체로부터 Manhattan 1 이내여야 한다. 격자에서는 둘 중 하나가 개체가 서 있는 tile이라는 뜻이다. 그래서 서로 붙은 두 차단 parcel 사이에는 새 결합을 만들 수 없다 |
| anchor 4 | 대상만 |
| grip 5 | 다른 개체가 잡고 있으면 Busy. LINK는 이미 다른 parcel을 잡고 있어도 Busy. UNLINK는 자기 grip에만 작용한다 |

- LINK: 강도 1당 64q. n = min(r5, 상한 − 현재, 봉인 잔량/64). 여유가 0이면 Capacity, 64q도 없으면 Energy다. n < r5이면 Partial이다.
- UNLINK: 행동당 128q. n = min(r5, 현재). 현재가 0이면 Noop이다.
- r15는 실제 강도 변화량이다. 에너지는 모두 대상 tile의 열이 된다. 결합 자체는 에너지를 저장하지 않는다.
- Think 때 대상 parcel의 UID와 면 상대의 UID를 캡처한다. 실행 때 다르면 Stale이다.

**단계 5: 구조 이동.** M1d에서는 모든 DRIVE를 단계 4가 아니라 단계 5에서 처리한다. 개체를 (prio6 우선순위, UID) 순서로 하나씩 처리하고, 각 처리는 그 시점의 세계를 본다.

| intent | 처리 |
| --- | --- |
| DRIVE 0, grip 있음 | 조립체 평행 이동 |
| DRIVE 0, grip 없음 | 단계 6(기존 동시 이동)으로 넘김 |
| DRIVE 1, grip 있음, r5 1..3 | 조립체 회전 |
| DRIVE 1, 그 밖 | 기존 방향 전환 |
| DRIVE 2 | PushParcel |
| DRIVE 3 이상 | InvalidTarget |

**조립체.** 기준 parcel에서 결합 강도 > 0인 면을 N, E, S, W 순서로 따라가는 BFS 연결 성분이다. 17번째 parcel을 발견하면 Limits다.

| 동작 | 기준 parcel | 이동 | 하중 진입 용량 | 기본비 |
| --- | --- | --- | --- | --- |
| 평행 이동 | grip parcel | 개체와 조립체가 같은 방향으로 1칸 | 16 × grip | 256 |
| 회전 | grip parcel(중심) | 개체와 조립체가 중심 둘레로 시계 방향 r5 quarter-turn | 16 × grip | 64 |
| PushParcel | 앞 tile의 parcel | 조립체만 개체의 앞 방향으로 1칸 | 16 × b(앞 parcel) | 256 |

회전하면 각 parcel의 bond 면도 같이 돈다(면 f → f + r5). parcel orient와 개체 orient에 r5를 더한다. 중간 궤적의 충돌은 보지 않고 시작과 끝 위치만 검사한다.

**실패 조건(검사 순서).**
1. 17번째 parcel: Limits
2. 개체나 parcel의 목적지가 경계 밖: Boundary
3. 조립체 안에 anchor > 0인 parcel: Blocked
4. 다른 개체의 grip: Busy. PushParcel은 자기 grip을 포함해 어떤 grip이든 Busy
5. 목적지에 조립체 밖의 parcel: Blocked
6. 차단 parcel의 목적지에 다른 개체, 움직이지 않는 자기 자신, 개체의 목적지, 출생 예약 tile: Blocked
7. 개체의 목적지에 다른 개체, 출생 예약, 조립체 밖의 차단 parcel: Blocked
8. 하중 검사 실패: Blocked
9. 봉인 잔량 부족: Energy

실패하면 시도비 말고는 아무것도 바뀌지 않는다. 결합이 자동으로 끊어지지도 않는다.

**하중 검사(원본 §64).** 각 parcel v의 요구량은 다음과 같다. m은 matrix와 pore의 질량, d는 시작과 끝의 Manhattan 거리, μg는 목적지 지면 마찰(μeff에서 parcel 항을 뺀 값), Δz는 고도차다.

demand(v) = ceil(m · d · μg / 960) + max(0, m · Δz)

source → 기준 parcel의 용량은 위 표의 진입 용량이다. 결합된 두 parcel 사이에는 양방향 용량 16 × 강도인 간선이 있다. 각 parcel → sink의 용량은 demand(v)다. 최대 유량이 총 요구량과 같아야 통과한다. 구현은 정수 Edmonds–Karp다. 통과 여부만 쓰고 유량 배치는 저장하지 않는다. 개체 자신의 질량은 grip 하중에 넣지 않는다.

**비용과 장부.**
- 지불 = 기본비 + 개체 마찰 ceil(m_a · d_a · μeff / 960) + Σ parcel 마찰 ceil(m · d · μg / 960) + Σ max(0, 각 물체의 ΔUg)
- 내리막에서 풀린 Ug는 그 물체의 목적지 tile 열이 된다. 오르막과 서로 상쇄하지 않는다.
- 마찰 열은 각 물체의 출발 tile에 남는다. 기본비 열은 개체의 출발 tile에 남는다.
- 열용량과 그 몫의 열은 시작 상태에서 모두 계산한 뒤 한꺼번에 옮긴다. 그래서 줄지어 이동해도 순서에 따라 결과가 바뀌지 않는다.
- r15는 움직인 parcel 수다.

**감각.** Parcel READ 필드가 실제 값을 돌려준다: 2 orient, 3 bond[r4], 4 anchor. M1d에서만 7 grip 강도, 8 응집력 b를 더한다.

**관찰.** `stats.csv`에 links(LINK로 더한 강도 합), unlinks, assembly_moves, parcels_moved, pushes, load_failures 열을 더한다.

**저장 호환.** Parcel 구조가 바뀌어 parcel이 있는 M1c 저장 파일은 읽을 수 없다. parcel이 없는 저장(M0..M1b)과 M0 물리 지문은 그대로다.

## 20f. M1e: 개체끼리의 접촉 (ruleset `gene-m1-5`)

M1e는 M1d에 원본 v1의 접촉 규칙 셋을 더한다(원본 §10, §13). 에너지 없이 피해를 주는 공격력·HP 명령은 없다. 모든 피해는 실제 일과 물질 이동이다.

**상태.** 개체마다 abrasion(u32)을 더한다. 지금 층(외피, 외피가 없으면 몸체)에 쌓인, 아직 한 단위가 되지 않은 접촉 일이다. 에너지 총계에는 들어가지 않는다. 일은 투입할 때 이미 열이 됐다.

**대상 캡처.** 다른 개체에 작용하는 효과는 Think 때 대상 UID를 캡처한다(원본 §59 ObjectKey). 실행 때 그 자리에 다른 개체가 있거나 아무도 없으면 Stale이다. Think 때 아무도 없었으면 WORK와 DRIVE 3은 InvalidTarget, TRANSFER는 M0 주기 규칙과 같이 Stale이다. 한 효과는 개체 하나만 캡처한다.

| 효과 | 캡처 위치 |
| --- | --- |
| TRANSFER | ActorInventory인 끝점(출발 또는 도착). 둘 다 ActorInventory이면 InvalidTarget |
| WORK 1, r0 = 3 | (r1, r2), 접촉 반경 1 |
| DRIVE 3 | 앞 tile |

**접촉 인출(TRANSFER 출발 = 3 ActorInventory).**
- 대상의 외피 S가 0일 때만 대상 inventory에서 꺼낼 수 있다. 외피가 있으면 Blocked다.
- port_mask는 입금 허가일 뿐 인출과 무관하다.
- 가용 에너지(채널 15)는 꺼낼 수 없다(InvalidTarget).
- 비용, 열, ΔUg는 다른 TRANSFER와 같다. 출발 tile은 대상의 tile이다.

**접촉 마모(WORK 1 Abrade, r0 = 3 ActorSurface).** (r1, r2)는 상대 좌표(반경 1)이고 자기 tile(0, 0)은 InvalidTarget이다. r5는 요청한 최대 일(q)이다.

| 층 | 한 단위 | 단위가 끝나면 |
| --- | --- | --- |
| 외피 S | 2048q | shell_s − 1, loose S + 1 |
| 몸체 G(외피 0일 때) | 8192q | body_g − 1, loose G + 1 |

- n = min(r5, 층에 남은 일, 봉인 잔량). 층에 남은 일 = 층의 단위 수 × 단위 일 − abrasion.
- n q 전부가 즉시 대상 tile의 열이 된다. abrasion에 n을 더하고 완성된 단위를 떼어 낸다.
- 떼어 낸 물질은 대상 tile의 loose가 된다(넘치면 residue). 같은 tile 안의 이동이라 열용량은 변하지 않는다.
- 외피가 0이 되면 abrasion을 0으로 한다(층이 바뀌면 남은 credit은 0, 원본 §13).
- body_g가 8 미만이 되면 단계 10에서 죽는다(기존 규칙, 사망 원인 Body).
- r5 = 0이면 Noop, 층에 남은 일이 0이면(body_g 0) Capacity, 잔량이 없으면 Energy, n < r5이면 Partial이다. r15 = n.

**PushActor(DRIVE 3).** 단계 5에서 다른 DRIVE와 같은 순서(prio6 우선순위, UID)로 처리한다.
- 앞 tile의 개체(캡처한 대상)를 그 앞 tile로 밀고, 미는 개체는 앞 tile로 들어간다. 두 이동은 하나의 transaction이다. 연쇄 밀기는 없다.
- 실패 조건(검사 순서): 앞 tile이 경계 밖(Boundary), 대상 없음(InvalidTarget)·바뀜(Stale), 그 앞 tile이 경계 밖(Boundary), 미는 개체가 grip을 쥠(Busy), 대상이 grip을 쥠(Blocked), 그 앞 tile에 개체·차단 parcel·출생 예약(Blocked), 봉인 잔량 부족(Energy).
- 비용 = 256 + ceil(m_미는 개체 · μeff(앞) / 960) + ceil(m_대상 · μeff(그 앞) / 960) + 두 개체 각각의 max(0, ΔUg). 시도비와 256은 한 번만 낸다.
- 내리막에서 풀린 Ug는 그 개체의 목적지 tile 열이 된다. 마찰과 256의 열은 각 개체의 출발 tile에 남는다.
- 열용량과 그 몫의 열은 시작 상태에서 계산한 뒤 한꺼번에 옮긴다.
- 밀린 개체에 아직 처리되지 않은 DRIVE intent가 있으면 그 receipt는 Stale이고 intent는 사라진다. 단계 6에서도 움직이지 않는다.
- r15 = 1.

**PushParcel(DRIVE 2).** M1e부터 원본대로 미는 개체도 앞 tile로 따라간다(D17). 검사와 비용은 grip 평행 이동과 같고 하중 진입 용량만 16 × b(앞 parcel)이다.

**관찰.** `stats.csv`에 taken(인출한 단위), abrasion_q(마모에 쓴 일), shell_removed, body_removed, actor_pushes 열을 더한다.

**저장 호환.** 개체 구조가 바뀌어 이전 버전의 저장 파일은 읽을 수 없다. M0..M1d의 결과는 그대로다(같은 seed의 stats·births·deaths CSV가 이전 binary와 byte 단위로 같음을 확인).

## 20g. M1f: parcel에 붙는 기록 매체 (ruleset `gene-m1-6`)

M1f는 M1e에 원본 §11의 "선택 매체"를 더한다. 이것으로 원본 v0..v1의 M1 범위가 모두 들어온다. `--ruleset m1f`가 M1 전체다.

**상태.** parcel마다 media(선택, 최대 8페이지)를 더한다. 공개 기록과 같은 Media 구조다.

**주소.** PublicTape(5)의 selector 0이 그 tile parcel의 매체다. selector 1..n은 여전히 버려진 매체다. READ(r6), WRITE(r6), COPY 원천(r7)·목적지(r3) 모두 같다.

**생성.** WRITE Resize(r0 = 5, r6 = 0)로 매체가 없는 parcel에 첫 페이지를 만든다(원본 §55). 비용은 다른 공개 매체 증설과 같다(행동한 개체의 G1, 128q, 시도비). parcel이 없으면 InvalidTarget이다. Cell 쓰기와 COPY 목적지는 이미 있는 매체에만 쓴다. 9페이지 이상은 Limits다.

**물리.**
- 매체의 Present 페이지는 parcel 질량과 열용량에 G로 들어간다. 그래서 조립체 하중과 마찰, 장부가 모두 매체를 포함한다(원본 §64 "붙어 있는 tape를 포함한다").
- 매체는 matrix 부피(s)와 응집력에는 들어가지 않는다.
- parcel이 움직이면(조립체 이동, 밀기) 매체도 같이 간다.
- 매체는 노출되어 있다. 같은 tile의 버려진 매체 다음 순서로 Rx2 산화와 bit 마모를 받는다(§20b). 페이지가 모두 없어지면 매체도 없어진다.
- parcel이 사라지면(matrix가 0) 매체는 같은 UID와 bytes로 그 tile의 버려진 매체가 된다(revision + 1). 물질과 정보를 두 번 만들지 않는다.

**관찰.** `stats.csv`에 parcel_media(매체가 있는 parcel 수), parcel_media_created, parcel_media_dropped 열을 더한다. 화면은 매체가 있는 parcel에 글이 새겨진 판을 그린다.

**결과 호환.** M0..M1e의 결과는 그대로다(같은 seed의 CSV가 이전 binary와 같음을 확인). Parcel 구조가 바뀌어 parcel이 있는 이전 저장 파일은 읽을 수 없다.

## 20h. 새 물리판: 분해 속도와 처리 순서, 수동 신호 전도와 LocalTrace

2026-10-10 사용자 결정(`docs/NEXT.md` "정한 것")에 따른 ruleset 두 개다. 이전 ruleset의 결과는 그대로다(같은 seed의 CSV가 이전 binary와 같음).

| ruleset | `--ruleset` | 내용 |
| --- | --- | --- |
| `gene-m0-2` | `m0b` | M0 + 빠른 분해 + 처리 순서. 생존 실험(E1-e)용 |
| `gene-m2-1` | `m2` | M1 전체(`gene-m1-6`) + 수동 신호 전도 + LocalTrace + 빠른 분해 + 처리 순서. E2용 |

**빠른 분해.** 생존 온도 bin(270..329 K, 330..499 K)에서 Rx2 등급을 최소 16으로 한다(원래 1과 4). 그보다 뜨거운 bin은 그대로다. 땅의 G, parcel의 matrix·pore·경계, 노출된 기록(버려진 기록, parcel 매체)에 같은 표를 쓴다. 기록 한 쪽이 생존 온도에서 산화되는 데 수일이 걸린다(원래 수십 일). 원본 구조(기록이 남고 누구나 읽고 베낌)는 그대로다. 근거는 §21의 증거 기록이다.

**처리 순서(locality).** 결과의 물리 의미는 같고 계산 순서만 바꾼다.
- 지도를 메모리 순서로 64 tile씩 자른다(segment). segment hash h = splitmix64 마무리 함수(seg)의 하위 32 bit다.
- 환경 단계: tile마다 16 tick에 한 번은 같다. 다만 `tick mod 16`이 h mod 16과 같은 segment의 64 tile을 한꺼번에 처리한다(원래는 16칸마다 한 칸).
- 수송 단계: edge 묶음(bucket)은 아래쪽 끝 tile이 속한 segment의 h & 7이다(원래는 edge마다 BLAKE3). 색(같은 끝을 공유하지 않는 묶음)은 그대로 x, y의 짝홀로 정한다.
- 속도(512², 개체 약 1,700, 한가한 CPU): 단일 thread 175..184 → 199..211 TPS(약 15%), 한 프로세스 4 thread 최고 325 → 358 TPS(약 10%). callgrind에서 L1 data miss는 2.5배 줄었으나 수송이 난수(Philox) 계산에 묶여 있어 이득이 이 정도다.

**수동 신호 전도(원본 §22, v1).** M > 0인 parcel이 M > 0인 이웃 parcel과 결합해 있으면 그 tile은 전도 tile이다. 다음 장의 값은 trunc(S_자기/2) + Σ 결합 이웃 trunc(S_이웃/8) + 이번 방출이다(채널별, 이전 장에서 동기식으로). 전도 tile이 아니면 이번 방출만 받는다. 증폭은 없고 손실을 열로 세지 않는다(원본 시험 T33: A 8, B 0 → A 4, B 1).

**LocalTrace(원본 §22, v2).** tile마다 최근 외부 동작 4개를 기억한다.
- 항목: 종류(1 물질 이동, 2 힘, 3 일, 4 결합, 5 방출, 6 기록), 방향, 실제량(u16), 일어난 tick, tick 안의 처리 순번. opcode, module, 의도, 계통은 없다.
- 놓이는 곳: 바뀐 tile. TRANSFER는 바깥 끝점(도착 우선), WORK·LINK·UNLINK·공개 기록 WRITE는 대상 tile, COPY는 공개 기록 목적지, EMIT은 자기 tile, 움직인 개체(이동, 밀기, 조립체 이동)는 떠난 tile, grip 회전은 자기 tile.
- 방향: 그 tile에서 동작 후 개체가 있는 쪽의 절대 방위(N, E, S, W = 0..3). 같은 tile이면 4.
- 성공(Ok, Partial)한 효과만 남는다. 새 항목이 앞에 오고 다섯째는 밀려난다.
- READ 9(반경 2, 외부 질의): r4 = 칸 0..3(최근순), field 0 종류, 1 상대 방향((방향 − 자기 방위) mod 4, 4는 4), 2 실제량, 3 나이(지금 − 일어난 tick). 빈 칸이나 256 tick 넘은 칸은 0이다. 칸·field 범위 밖은 BadAddress.
- 256 tick마다 읽을 수 없게 된 항목만 남은 tile을 지운다.

**저장 호환.** 상태에 LocalTrace가 더해져 이전 버전의 저장 파일은 읽을 수 없다.

## 20i. 물리판: 흙이 물을 붙잡음 (`gene-m0-3`, `gene-m2-2`)

**채택(사용자 결정 5, 2026-10-11).** 이전 ruleset의 결과는 그대로다(같은 seed의 core hash가 이전 binary와 같음).

| ruleset | `--ruleset` | 내용 |
| --- | --- | --- |
| `gene-m0-3` | `m0c` | `gene-m0-2` + 흙의 물 붙잡기 |
| `gene-m2-2` | `m2b` | `gene-m2-1` + 흙의 물 붙잡기 |

**근거.** 생물이 없는 세계(256², seed 1)에서도 물도 얼음도 없는 땅이 4일째 23%, 28일째 70%가 됐다. 물의 총량은 그대로였다(`docs/results/E1.md` E1-c 정정).
- 고도가 다른 이웃 사이에서는 물이 한 단위까지 모두 아래로 흐른다(§5.2).
- 생존 온도에는 증발이 없다(Rx7은 374 K 이상).
- 그래서 비탈은 결국 다 마르고, 물은 호수에 모여 돌아오지 않는다.

E1-c에서 W가 0인 자리에서 태어난 신생아는 하루를 넘기지 못했다. 살 수 있는 땅이 시간이 지날수록 줄어드는 것이다.

**규칙.** 고도가 다른 edge에서 위 tile의 W가 `SOIL_RETAIN` = 8 이하면 흐르지 않는다. 넘으면 후보량을 min(32, SR(32·minP, 256), W − 8)로 한다. 고도가 같은 edge의 확산, 얼음, 기체는 그대로다. 위 tile의 W가 8 이하일 때 난수 추첨을 건너뛰어도 counter 기반이라 다른 결과에 영향이 없다.

**확인.**
- 생물이 없는 같은 세계에서 10일째까지 마른 땅이 0%였다(기존 규칙은 4일째 23%, 8일째 36%).
- 시험: 위로 솟은 tile의 물 5는 `gene-m0-2`에서는 모두 흘러가고 `gene-m0-3`에서는 남는다. 물 40은 8만 남고 나머지가 흘러가며, 총량은 보존된다(`tests/water.rs`).

**남은 것.** 증발과 비(물이 높은 곳으로 돌아오는 길)는 넣지 않았다. 붙잡기만으로 개체군이 유지되는지 먼저 본다(`runs/pilot_e1e/m0c_seed*`).

## 20j. 나이에 따라 오르는 유지비 (`gene-m2-4` 채택, 비교용 `gene-m0-4`, `gene-m2-3`)

**`gene-m2-4`(나이 이틀마다 +1q/tick)를 채택했다(사용자 결정 6, 2026-10-11).** 원본은 나이로 죽이지 않는다(§6). 하루마다 +1q인 `gene-m2-3`, `gene-m0-4`는 비교용으로 남긴다. 조상 변이율은 원본 값을 지킨다(결정 7). 이전 ruleset의 결과는 그대로다.

| ruleset | `--ruleset` | 내용 |
| --- | --- | --- |
| `gene-m0-4` | `m0d` | `gene-m0-3` + 나이에 따른 유지비 |
| `gene-m2-3` | `m2c` | `gene-m2-2` + 나이에 따른 유지비 |
| `gene-m2-4` | `m2d` | `gene-m2-2` + 약한 나이 유지비: 나이 이틀마다 1q/tick(`2 × AGING_PERIOD`) |

**근거.** 128² 장기 실행(`gene-m2-2`, 488일과 832일)에서 어른이 사실상 죽지 않았다.
- 832일째 개체의 나이 중앙값은 724일이고, 거의 모두 energy가 50,000 이상이었다.
- 번식 장치가 망가진 개체도 죽지 않는다. 재생 2일 동안 REPRO 효과의 무효 대상 2,251, 형식 오류 209, 막힘 348이 나왔고 출생은 198개체에서 29건뿐이었다.
- 자리가 차서 신생아 하루 생존은 10% 남짓으로 떨어졌다. 최대 세대는 320일 이후 62..64에서 멈췄다.
- 죽음이 없으면 선택이 번식 못 하는 개체를 걸러 내지 못하고, 세대 교체가 멈춘다.

**규칙.** stage1 유지비에 floor(나이 / 4096)q를 더한다(`AGING_PERIOD`). 나이 하루마다 1q/tick이다. 낸 energy는 기존 유지비처럼 그 tile의 열이 된다. 그래서 장부는 그대로 맞는다. 난수는 쓰지 않는다. 어른의 하루 잉여를 대략 20..30q/tick으로 보면 30..60일쯤 지나 수입보다 유지비가 커진다.

**시험.** 40일 된 개체가 `gene-m2-3`에서는 tick마다 40q를 더 내고, `gene-m2-2`에서는 그렇지 않다. 장부 감사도 통과한다(`tests/water.rs`).

**비교 실행.** Founder-7, 128², 개체 64, seed 1, 1,000,000 tick(244일). `runs/aging/f7_m2c`, `runs/aging/f7_m2b`.

| | `gene-m2-2` | `gene-m2-3` |
| --- | --- | --- |
| 마지막 개체 수 | 388 | 269 |
| 출생 합계 | 54,144 | 48,270 |
| 마지막 61일 출생 | 5,264 | 6,672 |
| 신생아 하루 생존(첫 61일 → 마지막 61일) | 26% → 21% | 33% → 36% |
| 최대 세대 | 70 | 114 |
| 살아 있는 개체의 세대 중앙값 | 28 | 87 |
| 살아 있는 개체의 나이 중앙값, 최대 | 85일, 243일 | 1.2일, 25일 |
| 어른 사망의 최고 나이 | 133일 | 29일 |
| 마지막 고유 genome 수 | 271 | 79 |

- 나이 유지비가 있으면 누구도 약 29일을 넘기지 못한다. 예상(30..60일)보다 조금 짧다.
- 세대가 계속 깊어진다. 대조군은 최대 세대가 122..183일에 63, 끝에 70으로 느려졌다.
- 자리가 비어 신생아 생존이 오른다. 대조군에서는 나이 든 어른이 자리를 차지해 떨어진다.
- 개체 수는 약 30% 적다. 고유 genome도 적다. 세대 교체가 빨라 유리한 변이체가 빨리 퍼지는 것으로 보인다.
- 부모가 배아에 G를 넣는 출생은 둘 다 없었다. 판정기의 새 연쇄도 둘 다 없었다.
- 경쟁 시험(§17g, 각 실행 끝의 개체 수 상위 genome 5개, seed 1..4):

  | genome(운반자) | `gene-m2-2` 실행에서 | genome(운반자) | `gene-m2-3` 실행에서 |
  | --- | --- | --- | --- |
  | e595d178 (32) | +6.5%p, 4/4 | 0b96c0ab (43) | −7.4%p, 0/4 |
  | 77514f83 (14) | +1.0%p, 2/4 | 217c4bd5 (32) | +3.9%p, 3/4 |
  | 0137b88d (12) | 0.0, 0/4 | 13978ada (19) | −0.8%p, 1/4 |
  | 9372cb06 (11) | +1.8%p, 2/4 | 2de745d4 (13) | −2.6%p, 0/4 |
  | a7674d9b (8) | −0.5%p, 1/4 | 9ffea9be (13) | −21.0%p, 0/4 |

  - 0137b88d는 VM이 읽지 않는 operand bit만 다르다. 네 seed 모두 정확히 0이 나와 시험이 맞게 동작함을 보여 준다.
  - 나이 유지비가 없으면 흔한 genome이 조상과 같거나 낫다. e595d178은 명령 하나(`ADD r8, r12` → `CONST r8, 0`, 다섯 위치 탐색표의 읽기 자리)가 바뀌었고 네 seed 모두 이겼다. 선택이 적응을 만든다.
  - 나이 유지비가 있으면 흔한 genome이 대체로 조상보다 못하다. 세대 교체가 빠르고 개체군이 작아(200..300) 해로운 변이가 표류로 쌓이는 것으로 보인다(무성 번식이라 되돌릴 길이 적다).
- 변이를 끈 같은 실행(조상 header의 point 0, structural 0, `runs/aging/f7_m2c_nomut`):

  | 날 | 변이 있음: 개체 / 최대 세대 | 변이 없음: 개체 / 최대 세대 |
  | --- | --- | --- |
  | 32 | 602 / 20 | 884 / 24 |
  | 96 | 315 / 56 | 548 / 62 |
  | 160 | 283 / 87 | 404 / 99 |
  | 224 | 197 / 107 | 331 / 135 |
  | 244(끝) | 269, 출생 48,270 | 423, 출생 82,080 |

  - 변이 부담만으로 개체 수가 약 40% 줄었다. 출생당 변이 기대값(point 약 0.3 bit, 구조 변이 3%)의 대부분이 해롭다면 무성 개체군의 평형 부담은 약 1 − e^−0.3 ≈ 26%이고, 작은 개체군에서는 해로운 변이가 표류로 굳어 더 커진다.
  - 변이가 없어도 개체 수는 줄었다(995 → 약 330..420). 처음 땅에 있던 G(24,527)가 소비되고 죽은 개체의 기록에 G가 묶이기 때문으로 보인다(끝에 땅 1,718, 개체 22,744, 기록 18,090).
- module형 조상(Founder-8m+9m+10m, seed 1)도 같다. 변이가 있으면 656일째 멸종했고, 변이를 끄면 488일째 188개체로 버텼다.

  | 날 | 변이 있음: 개체 / 최대 세대 | 변이 없음: 개체 / 최대 세대 |
  | --- | --- | --- |
  | 64 | 254 / 33 | 454 / 39 |
  | 192 | 77 / 86 | 251 / 96 |
  | 320 | 46 / 140 | 218 / 149 |
  | 448 | 28 / 205 | 152 / 211 |

- 정리: 나이 유지비로 세대 교체가 빨라지면, 조상의 변이율(원본 §57 고정값 point 64, structural 2048)로는 작은 무성 개체군에 해로운 변이가 빨리 쌓인다. 변이율은 genome header에 있어 유전되고 변이도 되지만, 그 속도로는 따라잡지 못했다. 변이율을 4분의 1로 낮춘 조상(`runs/founders/*_lowmut.asm`)과 약한 나이 유지비(`gene-m2-4`)를 비교 중이다.
- 약한 나이 유지비(`gene-m2-4`, 나이 이틀마다 1q/tick)와 비교했다(`scripts/aging_compare.py`, 칸은 개체 수 / 최대 세대).

  Founder-7(128², 개체 64, seed 1):

  | 날 | `gene-m2-2` | `gene-m2-3` | `gene-m2-3` 변이 없음 | `gene-m2-3` 변이 4분의 1 | `gene-m2-4` |
  | --- | --- | --- | --- | --- | --- |
  | 32 | 841 / 20 | 602 / 20 | 884 / 24 | 779 / 24 | 784 / 24 |
  | 96 | 616 / 51 | 315 / 56 | 548 / 62 | 477 / 61 | 426 / 62 |
  | 160 | 441 / 59 | 283 / 87 | 404 / 99 | 394 / 100 | 312 / 83 |
  | 240 | 353 / 71 | 194 / 112 | 334 / 148 | 279 / 132 | 217 / 103 |
  | 244(끝) 개체, 출생 | 388, 54,144 | 269, 48,270 | 423, 82,080 | 378, 72,405 | 299, 56,483 |

  module형 조상(128², 개체 99, seed 1):

  | 날 | `gene-m2-2` | `gene-m2-3` | `gene-m2-3` 변이 없음 | `gene-m2-3` 변이 4분의 1 | `gene-m2-4` |
  | --- | --- | --- | --- | --- | --- |
  | 64 | 410 / 36 | 254 / 33 | 454 / 39 | 402 / 38 | 290 / 28 |
  | 192 | 219 / 74 | 77 / 86 | 251 / 96 | 223 / 100 | 191 / 92 |
  | 320 | 173 / 122 | 46 / 140 | 218 / 149 | 121 / 166 | 148 / 131 |
  | 480 | 150 / 122 | 25 / 217 | 167 / 230 | 96 / 240 | 100 / 161 |
  | 끝 | 611일 145 | 656일 멸종 | 488일 188 | 488일 146 | 488일 145 |

  - "변이 4분의 1"은 조상 header만 point 16, structural 512로 바꾼 것이다(`runs/founders/*_lowmut.asm`). 강한 나이 유지비에서도 개체 수가 변이 없음에 가깝고, module형 조상도 살아남았으며, 세대가 가장 깊었다.
  - `gene-m2-4`에서는 최대 수명이 약 57일이었다. module형 조상도 버텼고, 세대는 계속 깊어졌다(`gene-m2-2`는 320일 이후 122에서 멈춤).
  - 경쟁 시험(각 실행 끝의 상위 genome 5개, 그 실행의 조상과 비교, %p):

    | 조건 | 1 | 2 | 3 | 4 | 5 |
    | --- | --- | --- | --- | --- | --- |
    | `gene-m2-2` | +6.5 | +1.0 | 0.0 | +1.8 | −0.5 |
    | `gene-m2-3` | −7.4 | +3.9 | −0.8 | −2.6 | −21.0 |
    | `gene-m2-3` 변이 4분의 1 | +4.8 | −0.9 | 0.0 | −1.7 | −1.6 |
    | `gene-m2-4` | +1.8 | +2.1 | +1.0 | +1.0 | −6.4 |

    0.0은 조상과 행동이 같은 genome이다. 약한 나이 유지비에서는 흔한 genome 대부분이 조상보다 낫다. 변이율을 낮추면 해로운 쪽이 작아지지만 나은 쪽도 적다.
- 변이율 자체도 진화하기 시작한다(`rates` 예제). module형 조상 `gene-m2-3` seed 2의 977일째 개체 221 중 43(19%)이 구조 변이율 0인 계통이었다. `gene-m2-4` Founder-7의 244일째에는 299 중 7이 point 변이율 0이었다. 나이 유지비가 없는 실행에서는 거의 모두 조상 값(point 64, structural 2048)이었다.
- 같은 정체가 module형 조상 장기 실행(`gene-m2-2`)에서도 나왔다. 약 480일 이후 출생이 거의 멈췄고(개체 145, 최대 세대 122), 습관 조합은 0건이었다. 그래서 `gene-m2-3`으로 다시 돌린다(`runs/e2m_aging.sh`).

## 20k. 이끼: 환경이 G를 만든다 (`gene-m2-5` 후보, 결정 8)

**후보다. 기준 ruleset은 여전히 `gene-m2-4`다.** 원본에서 G는 생물의 광합성으로만 생긴다. E2 본 실험에서 먹이(G)는 생물 자신과 밤에 죽은 신생아의 몸에서만 나와 고르게 흩어졌다. 그래서 먹이 신호·짓기·배우기가 보상받지 못했다(`docs/results/E2.md`). 이끼는 땅에 먹이 덩어리를 만들어, 먹이를 찾고 알리고 지키는 행동이 이득이 될 조건을 만든다.

| ruleset | `--ruleset` | 내용 |
| --- | --- | --- |
| `gene-m2-5` | `m2e` | `gene-m2-4` + 이끼 |

**규칙.** stage8에서 tile을 처리할 때, 빛을 흡수한 바로 뒤에 다음을 본다.
- 대상: 비옥한 땅(substrate 3, 습도 장의 가장 습한 등급, 128² seed 3에서 5.8%), 개체도 parcel도 없는 tile.
- 조건: 이번에 흡수한 빛 ≥ 5000, 온도 260..330 K, loose C ≥ 1, loose W ≥ 4(`MOSS_W_MIN`), loose G < 16(`MOSS_CAP`), O 자리 있음.
- 확률: 흡수한 빛 × 64(`MOSS_RATE`) / 2²⁴로 Rx0 한 번(C + W → G + O). FluxRng(ReactionRound, selector 5 << 8)로 정한다.
- 에너지: 흡수한 빛에서 5000을 G에 담는다(tile 열 −5000). 생물의 광합성과 같은 반응열이다. 그래서 장부는 그대로 맞는다.
- 개체가 있는 tile은 그 개체가 빛을 쓰므로 이끼가 자라지 않는다. parcel 아래도 자라지 않는다.
- `stats.csv`의 마지막 열 `moss`는 그 구간의 이끼 반응 수다(체크포인트 형식은 바뀌지 않는다).

**보정(128², 개체 99, 조상 셋, seed 3).**
- 생물 없는 세계: 장부 잔차 0. 이끼가 하루 약 900(MOSS_RATE 8)..2,000(64) G를 만들고, 분해(Rx2)가 G를 C와 W로 되돌린다.
- MOSS_RATE 64에서 8일째 비옥한 땅의 loose G는 tile당 10.7, 나머지는 1.0이다. 먹이 덩어리가 생긴다.
- `gene-m2-4`와 같은 실행에서 `gene-m2-4` 쪽 결과는 저장 상태와 CSV가 byte까지 같다(이끼 열만 더해짐).
- 비옥한 땅은 128²에서 약 30×30의 덩어리 하나다(경계 tile 106). 8일째 개체의 12.3%가 그 위에 있다(면적 5.8%, `gene-m2-4`는 9.4%). 개체 수는 555로 `gene-m2-4`보다 많다.

**보정 시험: 먹이 신호가 이득이 되는가(결정 8의 기준).** Founder-8m 대 먹이 신호를 없앤 변형, `gene-m2-5`, 128², 100,000 tick, seed 1..4.

| 변형 | `gene-m2-4` | `gene-m2-5` |
| --- | --- | --- |
| 먹이 신호 없음 | +0.8%p, 1/4 | +0.8%p, 2/4 |
| 실행되지 않는 코드 한 쪽 | −12.0%p, 0/4 | −11.3%p, 0/4 |

통과하지 못했다. 이끼가 있어도 먹이 신호는 이득도 손해도 없다. 까닭은 둘이다.
- 조상은 G가 17 이상인 tile에서 주울 때만 신호를 낸다. 이끼는 16에서 멈추므로 이끼밭은 신호를 일으키지 못한다.
- 신호는 방출한 다음 tick 한 번만 장에 있고, 자기 tile과 네 이웃까지만 퍼진다(§20b). 듣는 쪽은 반경 2를 읽는다. 그래서 약 3칸 안에서 그 순간에 들어야 한다. 먹이 분포와 상관없이 신호가 닿는 범위가 너무 좁다.

## 20l. 공기 전도: 신호가 모든 칸으로 퍼진다 (`gene-m2-6` 후보, 결정 8의 B')

**후보다.** 원본(§20b, §20h)에서 신호는 방출한 다음 tick 한 번만 장에 있고, 결합한 금속 parcel을 따라서만 전도된다. 그래서 먹이 신호는 약 3칸 안에서 그 순간에만 들린다(§20k 보정). 금속 전도 규칙을 모든 칸에 적용하면 신호가 퍼지고 조금 남는다.

| ruleset | `--ruleset` | 내용 |
| --- | --- | --- |
| `gene-m2-6` | `m2f` | `gene-m2-5`(이끼) + 공기 전도 |

**규칙.** 다음 장 = trunc(S_자기/2) + Σ 네 이웃 trunc(S_이웃/8) + 이번 방출(채널별, 이전 장에서 동기식). §20h의 금속 전도 식을 모든 칸에 쓴다(금속 전도는 이 규칙에 포함된다). 증폭은 없고 손실을 열로 세지 않는 것도 같다. 시험: 한 칸의 64는 다음 tick에 그 칸 32, 이웃 8이 되고, 그다음 tick에 그 칸은 16 + 4 × 1이다(`tests/water.rs`).

**닿는 거리(계산).** 진폭 48(Founder-8m의 먹이 신호)은 7 tick 남고 2칸 떨어진 곳에서 최대 7이다. 듣는 쪽이 반경 2를 읽으므로 약 4칸 안에서 들린다. 진폭 255는 16 tick, 4칸에서 3이다.

**보정 시험.** Founder-8m 대 먹이 신호를 없앤 변형, `gene-m2-6`, 128², 100,000 tick, seed 1..4: +4.1%p, 2/4(+7.2, −2.2, +13.0, −1.7). 통과하지 못했다. 신호가 조금 더 멀리, 조금 더 오래 닿아도 먹이 신호는 이득이 없다. 조상은 길을 잃었을 때만 듣고, 신호는 몇 tick 만에 사라진다. 이끼밭은 크고 오래가서 신호 없이도 찾는다.

## 20m. 옮겨 다니는 이끼 꽃 (`gene-m2-7` 후보, 결정 8)

**후보다.** 이끼 세계(§20k)의 E3 예비 실험에서 개체는 이끼밭에 모였지만 선택의 방향은 바뀌지 않았다. 이끼밭이 크고 자리가 바뀌지 않아, 한 번 찾으면 머물면 되기 때문이다(`docs/results/E3.md`). 먹이 자리가 바뀌면 찾기, 알리기, 따라가기가 이득이 될 수 있다.

| ruleset | `--ruleset` | 내용 |
| --- | --- | --- |
| `gene-m2-7` | `m2g` | `gene-m2-4` + 이끼(§20k의 생장 규칙) + 옮겨 다니는 꽃 |
| `gene-m2-8` | `m2h` | `gene-m2-7` + 공기 전도(§20l) |

**규칙.**
- 땅을 16×16 칸 구역으로 나눈다. 4일(16,384 tick)마다 구역마다 Environment stream(object = 구역, ordinal = 3 << 56 | 기간)으로 1/10 확률로 꽃을 새로 정한다. 꽃은 구역 가운데 반지름 7칸의 원이다(구역의 약 60%). 땅의 약 6%다.
- 이끼 땅에서의 생장은 §20k와 같다. 다만 상한은 loose G 32다(`BLOOM_CAP`). 꽃은 짧게 피지만 풍부하다. Founder-8m이 먹이 신호를 내는 17 G를 넘는다.
- 기간이 끝나면 그 구역의 이끼는 더 자라지 않는다. 남은 G는 분해(Rx2)로 C와 W가 된다.
- 이끼 땅은 seed와 tick에서 다시 계산하므로 저장 상태에 들어가지 않는다. 화면은 같은 계산으로 이끼 땅을 표시한다.
- 시험: 같은 기간은 같은 꽃, 다음 기간은 다른 꽃이다. 64기간 평균 이끼 땅은 3..10%다. 꽃 구역의 가운데는 이끼 땅이고 모서리는 아니다. 장부가 맞는다(`tests/water.rs`).
- `gene-m2-4`, `gene-m2-5` 결과는 저장 상태와 CSV가 byte까지 같다.

## 21. 확장 규칙과 로드맵

### 증거 규칙 (원본 §62를 강제 규칙으로 승격)

새 물리, 새 primitive, 새 capability는 다음을 모두 갖춰야 들어온다.

1. 기록된 실행에서 그것이 없어서 막힌 경로가 관찰됨(seed, tick, 원인)
2. 추가하지 않는 대안(기존 primitive 조합)을 검토한 기록
3. 새 ruleset 번호, 그리고 이전 결과와 비교할 ablation 실행군
4. 시간당 세대 수의 변화량 측정

### 증거 기록: 생존 온도의 분해 속도 (`gene-m0-2`)

1. **막힌 경로.** E1-c(seed 1..20, `gene-m0-1`, Founder-6): 신생아 하루 생존이 처음 7일 55%에서 42..48일 6%로 떨어졌다. seed 20 재현 상태(tick 160,000)에서 물도 얼음도 없는 tile이 81%였고, 그런 자리에서 태어난 신생아 157개체가 모두 하루 안에 죽었다. 광합성은 C와 W를 G로 묶는데, 생존 온도의 Rx2 등급 1로는 죽은 G와 기록이 거의 분해되지 않는다. E1-d(seed 17..20, Founder-7)에서는 땅의 G는 묶이지 않았지만 버려진 기록이 28일째 77만 쪽으로 늘었다(`docs/results/E1.md`).
2. **추가하지 않는 대안.** (a) 죽은 G를 먹는 조상(Founder-7, E1-d): 개체 수가 같은 날 Founder-6의 약 3배였지만 기록이 새 함정이 됐다. (b) 기록을 재활용하는 조상: 한 쪽에 6,144q(32q × 128 cell + 시도비 64q × 32회)를 쓰고 G 1개(호흡 1,666q)를 얻어 쪽당 약 4,478q 손해다. 개체 행동으로 유지되기 어렵다. (c) 사망 시 기록을 남기지 않음: 원본 §55의 기록 통로를 없애므로 기각.
3. **ruleset과 ablation.** `gene-m0-2`(분해 + 처리 순서). 비교군은 같은 조상·seed의 `gene-m0-1`(E1-d)이다. 처리 순서만의 효과는 결과 bit가 달라지는 것 외에 물리 의미가 같으므로 따로 두지 않는다.
4. **시간당 세대 수.** E1-e에서 잰다.

### 단계

| 단계 | 내용 | 진입 조건 |
| --- | --- | --- |
| M0 | 이 명세 | — |
| E1 | 조상 생존 실험 | M0 시험 통과 |
| M1 | parcel, LINK, EMIT, 공개 tape, draft·learned, 개체 접촉, parcel 매체(원본 v0..v1의 나머지). M1a..M1f로 구현함(§20b..§20g) | E1에서 지속 개체군 확인 |
| M2 | 병렬 Think, Bevy viewer, S2 성능 gate | M1 |
| 연구 부록 | 원본 Matter-1, R7..R10(형상 역학, 일반 화학, 발생, 자기·광학, 파동장, 핵, 양자, LawField, 분산) | 위 증거 규칙을 항목별로 통과 |
