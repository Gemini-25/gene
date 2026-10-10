# Gene

인공문명 진화 시뮬레이터의 첫 단계(M0)다. 공개된 최소 조상 하나에서 시작해, 정해진 물리 규칙만으로 생존·번식·변이가 일어나는지 관찰한다. 문명, 직업, 기술은 엔진에 넣지 않는다. 그런 것이 생기는지는 실행 결과로 판정한다.

- 현행 규칙: [`docs/SPEC.md`](docs/SPEC.md). 유일한 기준 문서다.
- 원본 설계서 R10: [`docs/archive/R10-original-design.md`](docs/archive/R10-original-design.md). 보관용 근거 자료다.
- 실험 결과: [`docs/results/`](docs/results/)

## 상태

| 항목 | 상태 |
| --- | --- |
| M0 엔진(`gene-core`) | 구현 완료 |
| 시험 | 61개 통과 (단위 19, 장부·결정론·병렬 5, VM·이동 10, M1a/b 9, parcel 6, 결합·조립체 11, 조상 크기 1) |
| 에너지·원자 장부 | 시험에서는 매 tick 정확히 일치, 실행 중에는 4096 tick마다 검사 |
| 결정론 | 같은 seed면 같은 hash. 저장 후 이어 간 결과도 같음 |
| E1-a: Founder-4(원본 §57) | 20/20 seed가 첫 밤에 전멸 |
| E1-b: Founder-5(개정판) | 20/20 seed가 49일 생존. 그러나 계속 줄어 마지막 개체 수 중앙값 66 ([`docs/results/E1.md`](docs/results/E1.md)) |
| E1-c: Founder-6(오전에만 번식) | 진행 중. 7,000 가까이 늘었다가 감소. 죽은 유기물이 쌓이며 공기의 C가 줄어듦 |
| E1-d: Founder-7(죽은 G 수거) | E1-c 뒤 자동 실행. 소규모 점검에서 Founder-6보다 개체 수 2배 |
| P1: M1 기능의 자연 발견 | 소규모 4 seed에서 없음. m0와 m1d 결과가 완전히 같음 ([`docs/results/P1_M1_pilot.md`](docs/results/P1_M1_pilot.md)) |
| 속도(512², 개체 약 1,700) | 약 190 TPS, 단일 thread. 명세 목표 S1(500+) 미달, 병목은 환경 수송. Think·수송·환경은 `--sim-threads N`으로 병렬(결과 동일) |
| M1a(신호·공개 기록·초안) | ruleset `gene-m1-1`(`--ruleset m1a`) |
| M1b(학습 채택) | ruleset `gene-m1-2`(`--ruleset m1b`): DraftCommit, learned binding, 실행 중 옛 코드 보존 |
| M1c(구조물 parcel) | ruleset `gene-m1-3`(`--ruleset m1c`): 짓기·압축·내부 저장·차폐·통행 차단·내부 화학 |
| M1d(결합·조립체) | ruleset `gene-m1-4`(`--ruleset m1d`): LINK/UNLINK 면 결합·고정·grip, 최대 유량 하중 검사, 16칸까지 조립체 평행 이동·회전, 밀기 |
| 화면 | `gene serve`: 브라우저용 로컬 viewer(지도·재생·속도·개체 상세·genome 순위·탄소 그래프). Bevy는 보류 |

## 빠른 시작

Rust 1.85 이상이 필요하다.

```sh
cargo test --release                      # 전체 시험
cargo build --release

# 작은 세계에서 한 번 실행
./target/release/gene run --founder founder5 --size 128 --agents 64 --ticks 40000 --out runs/demo

# 명세 기본값(512², 1000 개체)으로 seed 여러 개를 병렬 실행
./target/release/gene batch --seeds 1-20 --threads 4 --founder founder5 --ticks 200000 --out runs/E1b

# 브라우저 화면: http://127.0.0.1:8080/ 를 연다 (처음엔 일시정지 상태)
./target/release/gene serve --founder founder5 --size 256 --agents 300
./target/release/gene serve --load runs/demo/final.bin --founder founder5   # 저장 상태 이어 보기

# 조상 프로그램 어셈블·역어셈블
./target/release/gene asm assets/founder5.asm
./target/release/gene founders

# 결과 요약
python3 scripts/e1_summary.py runs/E1b
```

`--founder`에는 `founder4`..`founder7` 또는 어셈블리 파일 경로를 줄 수 있다. `--ruleset`은 `m0`(기본), `m1a`, `m1b`, `m1c`, `m1d`다. 실행마다 다음 파일이 생긴다.

| 파일 | 내용 |
| --- | --- |
| `stats.csv` | 256 tick마다 개체 수, 출생·사망, 에너지 수지(유지·VM·효과), 광합성·호흡, 고유 genome 수, 장부 잔차 |
| `births.csv` | 출생 사건: 부모, 세대, genome hash, 변이 종류 |
| `deaths.csv` | 사망 사건: 원인, 나이, 자식 수 |
| `map_*.png` | 지형(고도·물)과 개체(genome별 색) |
| `summary.json` | 설정, 결과, 상태 hash, 속도 |

## 구조

```
crates/gene-core   시뮬레이션 core (Bevy·관찰기에 의존하지 않음)
  rng.rs           Philox4x32-10 + BLAKE3 stream 주소, 확률적 정수화 SR
  world.rs         지형 생성 WorldGen-3, tile 상태
  species.rs       10종 물질, 원소, 반응 Rx0..Rx8
  vm.rs, isa.rs    레지스터 VM, 어셈블러
  effects.rs       TRANSFER·WRITE·WORK·COPY·LINK (stage 4)
  records.rs       신호·공개 기록·초안·학습 채택 (M1a, M1b)
  parcels.rs       구조물 parcel: WORK, pore 교환, 내부 화학 (M1c)
  bonds.rs         결합·grip, 하중 graph, 조립체 이동 (M1d, stage 5)
  repro.rs         배아 상태 기계
  mutation.rs      point·구조 변이 8종
  motion.rs        동시 이동 의존 graph (stage 6)
  env.rs           빛·외기·수송·화학 (stage 7–8)
  sim.rs           tick 순서, 장부, 저장·이어하기
crates/gene-cli    실행기 `gene` (run, batch, resume, serve, inspect, asm)
  serve.rs         로컬 viewer 서버 (std만 사용, 시뮬레이션 thread가 상태를 단독 소유)
assets/            공개 조상 founder4..7.asm, viewer.html
scripts/           결과 집계
docs/              명세, 결과, 원본 보관본
```

## 원칙

- **몰래 돕지 않는다.** 보너스, 숨은 행동, 관찰 결과의 피드백이 없다. 조상 프로그램은 공개된 시작 조건이다.
- **규칙을 바꾸면 ruleset 번호를 올린다.** 결과를 이전 실행과 섞지 않는다.
- **새 물리는 증거가 있을 때만 넣는다.** 실행에서 그것이 없어서 막힌 경로가 관찰되어야 한다([`docs/SPEC.md` §21](docs/SPEC.md)).
- 구현 완료, 물리적 가능성, 자연 발견은 서로 다른 주장이다.
