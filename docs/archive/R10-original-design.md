# 인공문명 진화 시뮬레이터 R10 통합 세계모델 설계서

Rust + Bevy · 핵·국소 양자화학·확장 세계와 CPU/GPU/분산 실행 설계

2026년 9월 29일 · 개정 R10 · Continuum-3 통합 설계·검증 계약 · 실행 미검증

## 01 이 문서가 약속하는 것

**R10은 동위원소·핵반응·방사선, 국소 근사 양자화학, 확장 가능한 세계와 대규모 엔진 실행 계약을 추가한다.** 물리법칙 변화는 고정 meta-law 아래의 LawField 실험으로 분리한다. 신경망과 생애학습을 모두 포함하며 R9의 형태·유전·문화·넓은 계산 표현을 유지한다. 직업·장치·문명·기술·언어를 완성 행동으로 제공하지 않는다. 몸체의 자유로운3D 공간은 추가하지 않는다.

**읽는 순서: 이번 새 계약은121..138절이다.** 01..05절은 최신 공통 원칙이다. 06..67절은 Grid-5 회귀,68..80절은 Matter-1,81..94절은 MatterLife-1,95..107절은 Continuum-1,108..120절은 R9 Continuum-2 기반이다. R10의 교체표는121절, 핵/양자/법칙/세계/유한 자원은122..128절, 병목과 최적화는129..134절, Rust·phase는135절, 성능 gate는136절,80개 추가 기대 사례는137절, 동결 조건은138절이다. 총405개 기대 사례는 실제 통과한 시험이 아니다.

현재 ruleset은 civ-continuum-3-draft, 목표 제품은 v2-Continuum/Base-CT3다. LawField-CT3는 별도 v3 연구 profile이다. 저장 형식은 CIVCT003으로 분리한다. 핵·양자·물성·단위·결합 solver와 실측 성능은 검증 전이며 시뮬레이터 소스는 작성하지 않았다. 정확한 전 세계 양자물리, 무한한 컴퓨터·세계, 무수정 구현과 끝없는 발명을 약속하지 않는다. 새 자유도는 작은 reference부터 실제 검증해야 한다.

생명 형태·발생·대사 경로·생식 투자도 진화 대상에 포함한다. 단, 실제 분자 수준의 생명이나 무생물에서 생명의 최초 발생을 재현한다는 목표는 아니다. 공개된 최소 생체 조상에서 시작해 몸·행동·문화가 함께 달라지는 세계를 목표로 한다.

다만 **완성된 소프트웨어, 물리적으로 가능한 현상, 진화가 실제로 발견한 현상**은 서로 다른 완료 조건이다. 실행 규칙은 명세로 확정할 수 있다. 문명과 지속적인 발명이 나타나는지는 실행 증거가 필요하다. 그 결과까지 수정 없이 보장한다는 약속은 이 프로젝트의 성격상 할 수 없다. 이 사실은 구현 명세의 빈칸을 허용할 이유가 되지 않는다.

이 문서에서 MUST는 동일 규칙을 구현하기 위한 의무, DEFAULT는 실행 시작 시 사용하는 확정된 기본값, EXPERIMENT는 다른 RunManifest로 분리한 연구 조건이다. 정수 기본값이 검증된 생태 균형이라는 뜻은 아니다. 구현 중 문명 출현을 돕기 위해 몰래 값을 바꾸거나 새 행동을 추가하지 않는다.

### 확정된 기반과 R10의 적용 범위

| 초기 설계의 빈칸 또는 위험 | 현재 결정 | 확인 방법 |
| --- | --- | --- |
| 가공·마찰·차폐의 정성 설명 | 정수 물성식, 임계치, 비용 확정 | 수치 예제와 보존 시험 |
| 거대한 구조 이동의 범위 | Grid-5는 최대16셀 원자적 이동; Matter-1은 실제 역학으로 교체 | Grid-5 회귀와 Matter-1 기계 witness |
| 동시 이동을 향후 검토 | 단일 개체 의존 그래프와 복합 이동을 분리 | 맞교환·고리·혼잡 시험 |
| VM의 미정 opcode와 인자 | 32bit encoding, opcode 번호, I/O ABI | 모든 opcode의 의미 대조 |
| 복제의 에너지·복사·변이 순서 | 배아 상태 기계, 작업량, 예약과 실패 | 미완성·사망·재시도 시험 |
| 유전자만 있고 문화 획득 불명확 | 물리 매체·word 복사·실행 채택·출처 연결 | 비친자 전달 fixture |
| 자동 장치가 고수준 shortcut이 될 위험 | 장치 prefab 없이 접촉·압력·전기·열을 연결 | 제작·유지 비용을 포함한 물리 witness |
| 문명·기술 감지 threshold 미정 | 관찰 알고리즘과 초기 threshold 확정 | 원자료와 해석 대조 |
| 최종 제품과 v0 구분 부족 | v2-Continuum이 확장 목표, Grid-5/Matter-1은 기반 | 기능·성능·연구 gate를 별도 승인 |
| 고정 몸·닫힌 물질 목록·자기/광학 미지원 | 발생·구조 화학·자속·빛·생태의 추가 계약 | X01..X64와 복합 witness |
| 미세구조·파동·환경·계산 표현의 확장 | 보존형 국소 모델과 고정 VM의 guest 해석 | Y01..Y64와 W-CT1..W-CT6 |
| 원소·graph·실행·신경/학습·변이·기억 제약 | CT2 교체·비용·소유권·정확/근사 최적화 분리 | Z01..Z64와 W-CT2-1..8 |
| 핵·양자·법칙 변화·확장 공간과 실행 비용 | CT3 에너지/결합·유한 자원·CPU/GPU/분산/저장 계약 | Q01..Q80와 reference/실측 gate |
| 검증 항목만 있고 기대값 부족 | 수치 사례, 적대적 사례, 판정 계약 | 구현 시 자동 검증 |

## 02 최종 제품의 범위와 불변 조건

**완성 대상은 v2-Continuum/Base-CT3 데스크톱 프로그램이다.** 기존 생태·구조·생명·파동과 R9의 신경·학습·유전·넓은 기억에121..138절의 핵·국소 양자·확장 세계·실행 계약을 적용한다. LawField는 별도 연구 profile이다. 생태·문화 관찰을 포함한 통합 목표이며, 작은 reference와 최종 성능 달성을 구별한다. 기준 플랫폼은 64bit Windows·Linux 데스크톱, 키보드와 마우스다. macOS ARM64는 재현성 검증 대상에 포함하며 배포는 같은 gate를 통과한 뒤 표시한다. 네트워크 멀티플레이, 모바일, 브라우저 실행은 이번 제품 범위에 없다.

| 제품 기능 | 완료 조건 |
| --- | --- |
| 실제 2D 세계 | 지도·확대·개체·물질·운반·신호·선택이 실제 상태를 반영 |
| 독립적인 시뮬레이션 | headless와 visual의 같은 입력에서 같은 hash |
| 저수준 진화 | 가변 프로그램·모듈·변이·비용·단계적 복제 |
| 물질적 지속성 | 구조·기록이 제작자의 죽음 뒤에도 물리 규칙대로 유지 |
| 확장 물리·생명 | 형상·열·화학·EM·생명·핵·국소 양자 결합 witness와 유효 범위·오차 gate 통과 |
| 지식 전달 | 유전 및 비친자 매체 전달과 실제 채택을 구별 |
| 역사 관찰 | 계통·기술 증거·집단 변동·소실·되감기·분기 |
| 장기 운용 | 중단 복구·기록 보존·메모리 상한·OOM 안전 정지 |
| 성능 설정 | Pause, 1x, 5x, 50x, 500x, MAX와 달성 TPS 표시 |

불변 조건 I01-CT3: 화학은 동위원소별 핵 수, 핵반응은 지원 channel의 핵자·총 전하를 보존한다. 원소별 수와 단순 질량 보존은 핵반응에 강제하지 않는다. I02-CT3: 정지에너지 차이·전자/화학·열·기계·장·복사·입자·경계와 선택적 LawField를 단일 ledger로 정산한다. I03: Grid-5는 한 actor slot에 최대 한 개체, Matter-1은70절의 물리적 비관통. I04: 결합의 양방향 일치. I05: 개체는 국소 주소만 접근. I06: 관찰 분류는 선택압에 영향 없음. I07: thread·카메라·저장 속도는 미래 상태에 영향 없음. I08: 죽은 참조가 재사용된 다른 객체를 가리키지 않음. I09: 물리적으로 전달되지 않은 코드를 얻지 못함. I10: 완결 tick만 저장. I11: host 자원 부족은 생물학적 실패로 변환하지 않음. I12: 공개되지 않은 조상 행동이나 자동 구조 주입 없음.

구현 완료 판정과 연구 성공 판정을 별도 보고한다. 구현 완료에는 물리적 가능성 시험 통과가 포함된다. 그 시험의 손작성 프로그램을 자연 진화의 성공 사례로 세지 않는다.

## 03 핵심 설계 선택 비교

아래 비교의 성능은 실측이 아닌 구조적 예상이다. 모든 선택의 기준은 먼저 창발 가능성과 관찰의 정직성, 다음으로 대규모 실행 비용이다.

### Agent execution

| 방식 | 장점 | 단점 | 성능과 구현 복잡도 | 개방성에 미치는 영향 |
| --- | --- | --- | --- | --- |
| 레지스터 bytecode VM | 작은 상태, 중단과 재개, 호출과 가변 코드가 자연스럽다 | 명령 집합과 제어 흐름의 편향 | 빠름 예상, 중간 복잡도 | 절차 조합과 코드 전달에 적합 |
| 상태 기계 | 실행이 단순하고 상태를 보기 쉽다 | 규모가 커지면 상태와 연결이 폭증 | 매우 빠름, 초기 낮음 | 계층적 절차 재사용이 불편 |
| 행동 그래프 | 구조 돌연변이와 분기가 직관적 | 포인터 추적과 캐시 비효율 | 중간, 중간 이상 | 모듈성은 좋으나 표현 비용 증가 |
| 신경 제어기 | 연속 감각과 반응에 적합 | 가변 구조, 복사, 의미 있는 프로그램 전달이 어렵다 | 모델별 차이 큼, 높음 | 절차와 외부 기록의 연결을 추가해야 함 |
| VM과 신경 모듈 혼합 | 상징 절차와 학습을 결합 | 비용 회계와 재현성이 복잡 | 가장 높은 개발 비용 | CT2에서 선택적 도입; v0 회귀와 분리 |

**선택: 가변 길이 레지스터 VM과 선택적인 작은 희소 recurrent 신경 모듈.** v0 VM 회귀를 유지하고 CT2에서 유료 생애학습·변이 연산자를 결합한다. 신경 출력에 직업·행동 의미를 붙이지 않는다. 정확한 host 가속만 허용하며 계산 비용은 유지한다. 자가 복제 프로그램을 진화시키는 실험 플랫폼이라는 접근에는 Avida라는 선례가 있다. 이 설계가 Avida의 보상 체계를 채택한다는 뜻은 아니다. [Avida 공식 소개](https://avida.devosoft.org/)

### World storage

| 방식 | 장점 | 단점 | 성능과 구현 복잡도 | 개방성 영향 |
| --- | --- | --- | --- | --- |
| 하나의 flat grid | 주소 계산과 순차 접근이 단순 | 변경 부분의 저장과 전송이 번거롭다 | 소형에서 우수, 낮음 | 고정 세계도 가능하지만 대형 운영 불편 |
| chunked dense grid | 지역성, 병렬 작업, 부분 저장을 함께 지원 | 경계 접근과 청크 관리 필요 | 대형에서 균형적, 중간 | 지역적 수정과 지속되는 구조에 적합 |
| sparse chunk world | 대부분 비어 있는 확장 세계에 유리 | 해시 조회, 할당, 메모리 변동 | 밀집 환경에서 불리, 높음 | 무한 확장의 인상을 주지만 생태적 개방성은 별개 |

**선택: 64×64 tile의 dense SoA와 sparse chunk directory.** 작은 reference는 dense directory를 쓴다. 확장 세계의 미관측 시간·초기 재고·경계·dormancy는127절 계약을 따른다. 유한 자원 위의 넓은 주소 세계이며 진짜 무한 계산을 주장하지 않는다.

### Agent storage

| 방식 | 장점 | 단점 | 성능과 구현 복잡도 | 개방성 영향 |
| --- | --- | --- | --- | --- |
| 순수 Bevy ECS | 개발 도구와 컴포넌트 확장이 편하다 | VM, 대량 입출력, 결정론 순서를 직접 통제하기 번거롭다 | 구현에 따라 충분히 빠름, 초기 낮음 | 기능 추가는 쉽지만 코어 분리가 약해질 수 있음 |
| custom SoA | 데이터 크기와 순서를 직접 통제 | 생성, 삭제, 참조 관리 구현 필요 | 예측 가능, 중간 이상 | 표현과 저장 최적화의 분리가 중요 |
| 시뮬레이션 배열과 표시 ECS | 코어와 화면의 목적에 맞게 최적화 | snapshot 동기화 필요 | 전체 균형이 좋음, 중간 이상 | 표시 변경이 생태 규칙을 바꾸지 않음 |

**선택: 시뮬레이션은 custom SoA와 pool, Bevy ECS는 화면과 UI.** VM 레지스터 묶음처럼 한 개체가 함께 사용하는 작은 상태는 AoS로 둔다. 모든 필드를 무조건 SoA로 쪼개지는 않는다.

### Civilization detection

| 방식 | 장점 | 단점 | 성능과 구현 복잡도 | 개방성 영향 |
| --- | --- | --- | --- | --- |
| 명시적 문명 객체 | 안정된 소속, UI 구현 쉬움 | 문명 가입과 소유 규칙이 세계에 유입 | 저비용, 낮음 | 집단 형태를 미리 제한 |
| 매번 동적 clustering | 다양한 집단을 감지 | 소속과 ID가 흔들림, 비용 증가 | 분석 비용 높음 | 중첩과 분열을 표현하기 좋음 |
| 감지 후 시간적 ID 추적 | 창발 관계와 관찰 연속성을 함께 보존 | 분열과 합병의 매칭 필요 | 비동기 분석, 중간 이상 | 세계에 소속을 강제하지 않음 |

**선택: 동적 감지와 관찰용 ID 추적의 결합.** 코어에는 문명 membership가 없다. hybrid는 세계 규칙의 혼합이 아니라 분석 과정의 혼합이다.

### Technology representation

| 방식 | 장점 | 단점 | 성능과 구현 복잡도 | 개방성 영향 |
| --- | --- | --- | --- | --- |
| 명시적 프로그램 모듈 | 복사 출처와 재사용 추적 쉬움 | 코드가 같아도 기능이 다를 수 있음 | 해시 기반으로 저렴, 중간 | 프로그램 밖의 기술을 놓침 |
| 행동 패턴 감지 | 코드를 몰라도 기능을 볼 수 있음 | 환경 차이와 우연을 혼동 | 샘플링 필요, 높음 | 표현 방식에 덜 묶임 |
| 구조 패턴 감지 | 공간에서 반복되는 물건을 찾음 | 모양만 같고 기능이 없을 수 있음 | 국소 해시부터 시작, 중간 | 물리적 발명을 포착 |
| 세 증거의 결합 | 구조, 절차, 전달을 함께 확인 | 신뢰도와 인과 검증 필요 | 가장 복잡, 단계 도입 | 특정 기술 형태에 덜 편향 |

**선택: 모듈 출처를 정확히 기록하고, 행동과 구조 증거를 결합하는 hybrid.** 기술은 하나의 struct로 세계에 배포되는 능력이 아니라 관찰 증거의 묶음이다.

### Mutation

| 방식 | 장점 | 단점 | 성능과 구현 복잡도 | 개방성 영향 |
| --- | --- | --- | --- | --- |
| 고정 벡터 GA 변이 | 간단하고 배치하기 쉬움 | 새로운 계층과 구조를 만들기 어려움 | 빠름, 낮음 | 표현 공간이 쉽게 닫힘 |
| 선형 genetic programming | 명령 삽입, 삭제, 치환 가능 | 큰 변이가 실행 흐름을 깨뜨림 | 변이 구간 길이에 비례, 중간 | 절차 진화에 적합 |
| 모듈 구조 진화 | 유용한 묶음을 보존해 조합 | 초기 모듈과 경계의 편향 | 코드 복사 비용, 중간 이상 | 축적과 재조합에 유리 |

**선택: 유전되는 일반 VM 변이 절차와 물리 복사 잡음의 결합.** 선형·모듈 편집을 유료 복사·기록으로 구성한다. 전역 세대 교체, fitness 점수 정렬, 우수 개체 재주입 없이 실제 번식 과정에 적용한다.

### 추가로 확정한 선택

| 문제 | 비교와 trade-off | 최종 선택 |
| --- | --- | --- |
| 인공 물리 | Grid-5는 기준;68절의 형상과82..87절의 확장안을 비교 | Continuum-3; 핵·국소 양자·생명·신경/학습·확장 세계, 추가 검증 필요 |
| 정보 습득 | 전역 module import는 빠르나 지식 이동을 우회. raw byte 복사는 비용이 들고 접근 경로가 실제로 남음 | 국소 word 복사와 자신의 draft 채택 |
| 충돌 | 원자 lock은 결과가 실행 순서에 의존. 전체 intent 정렬은 명확하나 직렬 병목 | bounded 국소 transaction + 단일 개체 이동 graph |
| 규칙 확장 | native law 교체는 재현/보존을 깨뜨림; 계수장도 고정 meta-law 필요 | Base는 고정, LawField는 별도126절 실험 |
| 제어 추상화 | 자동 함수 추출은 발견을 대신할 수 있음. 구조 변이는 탐색이 어렵지만 절차 출현을 관찰 가능 | 구문적 모듈 변이와 명시적 CALL, 의미 기반 자동 추출 없음 |

## 04 네 범주의 경계 계약

각 절의 경계 문장은 아래 약어를 쓴다. W는 Hard-coded World Rule, P는 Agent Primitive, E는 Emergent Behavior, U는 UI Interpretation이다. 세계 밖의 기능은 W 대신 Engineering Contract로 표시한다.

| 시스템 | W 또는 공학 계약 | P | E | U |
| --- | --- | --- | --- | --- |
| 생명체 | compartment·유료 발생·합성·복사 | 국소 발현·조직 활성·물질 조작 | 형태·대사·행동·문화 진화 | 종·기관·생활 양식 |
| 에너지·화학 | 원소·열·반응·국소 수송 | 가열·물질 배치 | 생산·생태 순환 | 열 지도·반응 흐름 |
| 공간 | 국소성·점유·접촉 | READ, DRIVE | 경로·차단·이주 | 이동로·영토 |
| 물질 | 보존·마찰·투과·결합 | TRANSFER, WORK, LINK | 보관·도구·건설 | 길·창고·공장 후보 |
| 실행 | VM·유료 신경 계산·물리 기억 | 산술·분기·호출·기록·수치 kernel | 절차·학습·재사용·추상화 | 역할·행동군 |
| 복제 | 실제 template 복사·분할·잡음 | 국소 복사·유전 operator 실행 | 생애사·변이/재조합 방식·체형 | 계통·분기 |
| 통신 | 빛/용질의 실제 이동·흡수·감쇠 | 유료 EMIT, 국소 READ | 신호 관례·중계 | 프로토콜 후보 |
| 문화 | 기록·somatic/germline 분리·유료 학습 | READ, WRITE, 복사·국소 갱신 | 학습·가르침·확산 | 기술 계보 |
| 경제·대립 | 접촉·노출·점유 | 물질 이동·일·힘 | 교환·분업·탈취 | 경제·분쟁 후보 |
| 미세구조 | 실제 재고·접촉·상호작용 potential | 배치·열·힘·합성 | 자기조립·복합재 | 재료 family |
| 파동장 | 동적 Maxwell·재료 응답·에너지 | 기존 물질·전하 조작 | 공진·회절·무선 상호작용 | 파동 장치 후보 |
| 환경 변화 | 유한 재고·침식·퇴적·복사·수문 | 기존 물질·열 조작 | 장기 생태/환경 공진화 | 기후·유역·지층 |
| 계산 표현 | 고정 VM·유료 기억·국소 권한 | 기존 연산·분기·읽기·쓰기 | 자기 언어·해석기 | 표현 계보 |
| 핵반응 | 핵종·channel·단일 에너지·방사선 수송 | 기존 배치·열·힘·감각 | 에너지 이용·차폐·변성·적응 | 핵/침적·기술 후보 |
| 국소 양자 | 전자구조·embedding·kinetics·유효 범위 | 기존 감각·물질·에너지 조작 | 새로운 재료·촉매·생체 구조 | 지원 범위·오차·반응 증거 |
| LawField | 고정 meta-law·계수장·반작용 | 기존 물리적 상호작용 | 유효 환경·생태 공진화 | 법칙 변화 실험 |
| 집단·영토 | 추가 세계 규칙 없음 | 추가 명령 없음 | 지속 관계·공간적 영향 | 문명·정착지·국경 |
| novelty | 관찰 알고리즘 | 없음 | 새로운 실제 패턴 | 발견 알림 |
| 저장·화면·병렬화 | 공학적 계약 | 없음 | 세계 결과를 보존 | 화면·역사·속도 |

Job, House, Road, Factory, Civilization, TechnologyUnlock, Research, Trade, Government와 Plant/Animal/OrganKind/MachineKind 같은 역할·기관·장치 enum은 core에 금지한다. 화면에서 쓰는 lineage 색, 집단 ID와 기술 ID는 READ에 노출하지 않는다. soft ownership도 ObserverCache에만 둔다.

## 05 구성 요소와 규칙의 소유권

Core는 Bevy와 observer를 의존하지 않는다. Runner가 Core를 단독 소유하고 tick·입력·worker pool을 통제한다. Storage는 명시적 스키마로 상태를 직렬화한다. Observer는 사실 이벤트와 읽기 전용 표본에서 해석을 만든다. Viewer는 snapshot을 소비한다. 어느 경로에도 Observer → Core의 자동 보상·행동 명령은 없다.

| 구성 | 권위 있는 데이터 | 외부 경계 |
| --- | --- | --- |
| Simulation | 세계·개체·VM·물질·기록·배아·난수 주소·ID | step(TickCommands), checkpoint view |
| Runner | 실행 모드·속도·입력 확정·작업 pool | 제어 message, immutable snapshot |
| Observer | 행동 archive·계통 색인·기술 증거·집단 추정 | ObservationBatch |
| Storage | 파일 manifest·section·기록 segment | 정확한 load/save, 복구 결과 |
| Viewer | 카메라·GPU cache·패널·선택 | viewport 요청과 Runner 제어 |

PhysicsSpec는 반응식·phase 순서·한계·주소 규칙·RNG를 포함한다. RunConfig는 world 크기·seed·시작 개체·환경 배치를 포함한다. AnalysisConfig는 표본·threshold·분류기를 포함한다. DisplayConfig는 화면 품질이다. 각 config는 별도 hash를 가진다. PhysicsSpec와 RunConfig는 실행 시작 뒤 불변이다. 선택적 LawField의 χ는 이 고정 법칙 아래 변하는 세계 상태다.

아래 v0/v1/v2 표는 Grid-5의 개발 capability profile이다. 자연 교환·반응 정수화·오류 latch는 이 세 profile에서 같은 의미다. 확장 최종 목표 v2-Continuum의 최신 교체는121..138절이다. crate와 authoritative store는135절, 미동결 입력은138절을 따른다. 모든 profile 차이는 manifest에 기록하고 체크포인트를 조용히 승격하지 않는다. 이전에 없던 법칙을 켜서 같은 세계의 자연 발명으로 표현하지 않는다.

### Grid-5 개발 capability profile

| 기능 | v0 | v1 | v2/Grid-5 |
| --- | --- | --- | --- |
| VM·모듈·복제·변이·graph 이동 | 켬 | 켬 | 켬 |
| parcel·pore stock·압축·고정 결합 | 켬 | 켬 | 켬 |
| 이동 조립체 상한 | 1셀 | 16셀 | 16셀 |
| PushActor/PushParcel·몸체 막 마모 | 끔 | 켬 | 켬 |
| local EMIT | 켬 | 켬 | 켬 |
| 외부 tape·learned commit | 끔 | 켬 | 켬 |
| 열·물질 수송·기본 화학 | 켬 | 켬 | 켬 |
| 수동 신호 전도 | 끔 | 켬 | 켬 |
| LocalTrace | 끔 | 끔 | 켬 |
| 기술·집단 관찰 | 기초 통계 | 후보 분석 | 전체 evidence UI |

꺼진 capability의 opcode는 실행 비용을 내고 BadAddress/InvalidTarget을 반환하며 다른 기능으로 대체되지 않는다. v0 code와 재료에 v2 기능을 숨겨 두었다가 tick 수에 따라 활성화하지 않는다.

**경계:** Engineering Contract는 데이터 방향과 버전 규칙이다. P 없음. E는 Core에서만 발생한다. U는 독립 분석과 화면이다.

## 06 현실 유사 모델의 범위·단위·기본 설정

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

R5는 물질 생성 버튼과 추상적인 연료 충전을 없애고 **원자 수·질량·화학 에너지·열·위치 에너지를 정산하는 격자 모델**을 채택한다. 태양으로 들어온 에너지가 화학 상태와 생명체의 가용 에너지를 거쳐 열로 흩어지고 외부로 나간다. 재료 반응에는 실제 반응물과 부산물이 필요하다. 다만 실제 지구를 예측하는 분자 동역학·유체·열역학 소프트웨어는 아니다. 물성 수치는 축약 모델의 DEFAULT이며 실측 상수로 소개하지 않는다.

**Grid-5 기준 profile에서는 생물의 신체·기관·대사 경로 진화를 제외한다. R7 최종 profile은83..85절로 교체한다.** 모든 agent는 같은 기본 몸체·감각·대사·복제 장치를 갖는다. 가변 프로그램과 그 변이·유전, 모듈 습득, 문화적 전파는 유지한다. 따라서 이 문서의 genome는 현실 DNA가 아니라 행동 프로그램의 물리적 매체다. 종 분화·다세포 발생·무생물에서 최초 생명의 탄생을 구현했다고 표시하지 않는다.

| 항목 | DEFAULT | 의미 |
| --- | --- | --- |
| 공간·시간 | 4방향 격자, 1x=10 tick/s | 물질에 대해 닫힌 경계, 에너지는 외부와 교환 |
| world·chunk | 512×512, 64×64 | seed=1, 초기 1,000개체 |
| 에너지·온도 | 정수 q, 온도 θ=floor(U/C) | q·tile·tick의 SI 환산은 미정; θ는 Kelvin을 본뜬 모델 값 |
| 물질 | 10종, 채널별 u16 | 원소 5종의 고정 조성, 물질별 유효 부피 |
| inventory | 유효 부피 합≤64 | 몸체·매체·배아는 별도이며 하중에는 포함 |
| 몸체 | 유기물 G16, 외피 S2 | 손실 가능, 신체 형질·기관은 진화하지 않음 |
| 가용 에너지 | 개체·배아 각각 0..65,535q | 열과 다른 저장량 |
| 기억·코드 | 작업 기억 64 u16, 코드≤4,096 u32 | 유료 용량은128 u16 cell 단위, 페이지마다G1; 자기·배아·Dropped≤9216cell, parcel 공개 매체≤1024cell |
| 모듈·stack | 활성 binding≤64, frame≤8 | retained slot까지≤128 |
| 실행 | 32 instruction, 외부 query≤4, 효과 intent≤1/tick | 계산·읽기·시도·성공량의 비용 구분 |
| 감각·접촉 | Manhattan 반경 2 / 1 | 물리 차폐 적용, 숨은 ID·전역 좌표 없음 |
| 구조 | tile당 parcel 1, matrix 부피≤64 | 최대 16셀 이동 조립체, actor는 하나만 포함 |
| 신호 | 4채널, signed 16bit | 전자기학 대신 유료 국소 신호의 축약 모델 |
| 환경 | tile별 16tick 갱신, edge별 8tick 수송 | 주기4096tick의 낮·밤, 시작은 정오 |
| 관찰·저장 | 행동 창256, 집단 창4,096tick | checkpoint10,000tick, 10회마다 full |

양수 나눗셈은 floor, signed 나눗셈은 0 방향 truncation이다. 에너지 총계는 signed i128, 원소 총계와 큰 곱셈은 u128 중간값으로 계산한다. VM 산술 외 세계 상태의 wrap은 금지한다. 물리 상한 실패와 host RAM 부족은 다른 결과다. 후자는 실행을 일시 정지하며 특정 개체의 생식 실패로 바꾸지 않는다.

### 생물 진화에 대한 선택 비교

| 선택 | 장점·open-endedness | 단점·성능·복잡도 | 결정 |
| --- | --- | --- | --- |
| 신체·대사·행동 모두 진화 | 새로운 생태적 가능성 자체가 늘어날 여지 | 발생·물성·생식의 추가 모델, 검증과 agent 상태 비용 큼 | 별도 미래 연구 |
| 고정 생명체, 행동·문화 진화 | 문명·기술 가설을 분리해 검증, 기존 고성능 VM 사용 | 생물학적 혁신 공간은 제한됨 | **기본 선택** |
| 프로그램 유전·변이도 제거 | 단순한 실행과 분석 | 적응을 위해 별도 학습 기구나 사람이 만든 정책이 필요 | 현재 목표와 맞지 않아 제외 |

**경계:** W는 고정 신체와 물리·대사 법칙. P는 같은 저수준 명령. E는 행동·기술·사회 관계의 변화. U는 계통과 기술의 해석이며 생물학적 종을 확정하지 않는다.

## 07 초기 세계와 공개 조상

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

WorldGen-3은 정수 value noise를 사용한다. 간격 L=64와16의 꼭짓점마다 u16 난수를 얻고 셀 좌표 u,v에서 네 값을 (L-u)(L-v),u(L-v),(L-u)v,uv로 가중 합산해 L²로 floor 나눈다. 최종값은 floor((3V64+V16)/4)다. height,moisture,light,ore는 별도 RNG domain 주소를 쓴다. elevation=floor(height/4096), μ=256+floor(moisture/128), Lpeak=32,768+floor(light/2)다. substrate_kind=floor(moisture/16,384), surface_c=0이다. 지형의 시각적 태그 자체에는 생존 보너스가 없다.

각 tile의 초기 W=8+floor(moisture/2048), C=16,O=16,G=개별 난수 mod4,S=2다. ore≥49,152이면 X=32+(개별 난수 mod32), 아니면 X=0이다. H,M,V,I는0이며 parcel·tape·신호는 없다. 초기 θ=300이 되도록 모든 초기 물질·몸체 배치 후 U=300Ctile로 둔다. 자원을 나중에 자동 재충전하지 않는다. 폐기물과 자원이 순환할지는 반응·수송·생명체의 활동에 달렸다.

시작 위치는 자기 G≥1이며 네 유효 이웃과 고도차≤4인 서로 다른 tile 중 spawn hash 순으로 1,000개를 고른다. 후보가 모자라면 설정 실패다. 성공할 seed를 몰래 재추첨하지 않는다. 각 개체의 body G16/S2, work-memory G1, 실제 code 페이지의 G, inventory G32/S4, 가용 energy32,768q는 초기 조건으로 추가한다. 이 초기 투입은 모두 시작 ledger에 포함한다.

공개 조상 Founder-4는57절의 우선순위·상태 전이·실패 처리로 고정한다. G와S의 국소 확보, 낮·밤의 유료 감각과 대기, 실제 페이지 확보·COPY·Finish·Step·에너지 투자·Release만 한다. 실제 명령은 공통 VM으로 실행하고 초기 상수와 루프도 변이 가능하다. 초기 개체에 건설·기록 공유·제련·거래·신호 중계 절차를 주지 않는다. 우리가 조상에 제공한 확보·자기 복제·대기 자체는 자연 발명으로 계수하지 않으며 시작 조건의 편향으로 공개한다.

조상의 실제 바이너리·hash는 첫 구현에서 표와 대조하여 산출하며 아직 생성하지 않았다. 시작 에너지·물질은 그 실제 코드 페이지 수로 계산한다. 기능 witness와 자연 발견 조상의 bytes/hash를 구별한다. 조상의 밤 유지·복제 비용은57절 계산식과 실제 trace로 검증하며 살아남는다고 선험적으로 가정하지 않는다.

**경계:** W는 초기 조건과 환경 생성. P는 공통 ISA. E는 이후의 생존·복제 전략과 문화. U는 시작 조건·지형 색과 계통 기록이다.

## 08 World 저장과 공간 주소

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

world 크기는 64의 배수, tile 수≤u32::MAX, 각 축≤u16::MAX다. TileIndex=y×width+x, chunk_index=(y/64)×chunks_x+x/64, local_index=(y mod64)×64+x mod64다. 경계 밖은 Boundary다. torus는 별도 시험이며 기본 세계와 혼용하지 않는다.

| SoA 필드 | 형식 | byte/tile |
| --- | --- | --- |
| substrate_kind,surface_c | u8 두 배열 | 2 |
| elevation | i16 | 2 |
| loose species | 10개의 u16 배열 | 20 |
| actor_slot,parcel_slot | u32 두 배열,0=None | 8 |
| thermal_u | u64 | 8 |
| μ,Lpeak | u16 두 배열 | 4 |
| 기본 payload | padding 없는 배열 합 | **44** |

열용량 Ctile은 1024+Σ(c_in_i)다. 같은 tile의 loose/residue, parcel matrix/pore, 개체 몸체·inventory·매체·배아, corpse 매체를 각각 한 번 포함한다. 1024는 움직이지 않는 기반 열용량이며 채취 가능한 자원이 아니다. Ctile은 derived cache, thermal_u는 authoritative 상태다. 물질이나 몸체가 바뀌면 cache를 같은 transaction에서 갱신한다. θ는 계산해서 읽으며 별도 authoritative 온도를 중복 저장하지 않는다.

반응 credit, 신호, 공개 동작 흔적, 매체 목록, 국소 마모 작업은 sparse sidecar다. 화면 밖이라고 물리 갱신을 생략하지 않는다. 빈 지역도 빛·열 변화가 있으므로 정확한 recurrence를 건너뛰는 증명이 없으면 활성 상태다.

tile slot은 순간 조회용이다. 장기 참조는 typed (slot,generation), 역사 ID는 u64다. chunk alive 목록은 commit 때 변경분만 갱신한다. 각 tile을 Bevy Entity로 만들지 않는다.

사망·파괴의 강제 반환으로 u16 한계를 넘으면 ResidueRecord의 [u64;10]에 보존한다. READ는 loose+residue를 u16 포화해 보여준다. 채취는 loose 먼저, residue 다음이다. 정상 입고의 여유는 max(0,65,535-total)이며 강제 반환의 예외를 일반 입고에 이용할 수 없다. 빈 residue만 회수한다.

**경계:** W는 점유·저장·국소 접근. P는 상대 주소. E는 공간적 분화. U는 지도의 축약·확대다.

## 09 물질·열·에너지의 보존 회계

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

원소 벡터의 순서는 C,H,O,Fe,Si, 원소 질량은12,1,16,56,28이다. 아래 분자식의 원자 수로 질량 m을 계산한다. c는 원자 수에 비례하는 모델 열용량, v는 수용 한계를 위한 유효 부피, h는 형성 에너지의 모델 기준값이다. **h와 반응 속도는 실측 화학 데이터가 아니다.** G는 수화된 유기물을 CH2O 상당량으로 묶은 축약종이며 실제 생체분자의 다양성은 표현하지 않는다.

| ID·기호 | 조성·상태 | m / v / c | h(q/단위) |
| --- | --- | --- | --- |
| 0 W | H2O 액체 | 18 / 1 / 3 | -3000 |
| 1 C | CO2 기체 | 44 / 4 / 3 | -4000 |
| 2 O | O2 기체 | 32 / 4 / 2 | 0 |
| 3 G | CH2O 유기 응축물 | 30 / 1 / 4 | -2000 |
| 4 H | H2 기체 | 2 / 4 / 2 | 0 |
| 5 X | FeO 상당 산화물 | 72 / 1 / 2 | -2800 |
| 6 M | Fe 상당 금속 | 56 / 1 / 1 | 0 |
| 7 S | SiO2 상당 불활성 고체 | 60 / 1 / 3 | -9000 |
| 8 V | H2O 증기 | 18 / 4 / 3 | -2560 |
| 9 I | H2O 얼음 | 18 / 1 / 3 | -3060 |

모든 물질 위치의 원자 수 합 A[e]는 초기량+기록된 외부 투입−외부 제거와 정확히 같다. 에너지 현재값은 **Σ(n_ih_i)+Σtile. U+Σagent.energy+Σembryo.energy+Σphoto_credit+Ug**다. Ug=Σ(이동 가능한 물질의 질량×소재 tile의 elevation)이며 매체·몸체도 포함한다. 음수 기준 화학 에너지 때문에 총계는 i128이다. 초기 에너지+solar_in+sky_in+intervention_in−radiation_out−intervention_out=현재값을 만족해야 한다.

HeatProduced는 에너지 전환의 누적 통계다. 같은 열은 이미 tile. U에 있으므로 이 통계를 현재 총계에 다시 더하지 않는다. 모든 계산·행동·마찰 비용은 현재 위치 또는 명시된 접촉 위치의 U로 옮긴다. VM energy를 빼고 열 기록만 늘린 채 U에 넣지 않는 구현은 보존 실패다. 균일한 열을 가용 에너지로 바꾸는 primitive는 없다.

물질이 다른 tile로 이동하면 이동 물질의 열용량 c_move에 대해 q_heat=floor(U_source×c_move/C_source)를 함께 옮긴다. 계산에는 이동 전 상태를 쓰고 source U에서 한 번만 차감한다. source·destination의 C도 이동량만큼 변경한다. 같은 tile의 compartment 이전은 열 이동0이다. 단일 actor의 동시 이동 graph는 모든 source의 시작 U,C로 heat parcel을 먼저 계산해 합산 commit한다. 기반 열용량 때문에 source U보다 큰 열을 가져갈 수 없다.

이동·TRANSFER·출생 분리의 ΔUg>0은 actor의 action budget에서 추가 지불하고 ΔUg<0은 목적지 열로 바꾼다. 비용으로 지불한 위치 에너지를 열에도 중복 가산하지 않는다. 한 조립체는 실제 출발·도착 tile별 질량으로 ΔUg를 합산한다. 같은 source tile에서 여러 부분이 이동하면 그 c_move를 먼저 합해 열을 한 번 떼고, 목적지별 c 비례 floor와 나머지의 목적지TileIndex순으로 나눠 총열을 보존한다. 물질의 수평 재배열·동일 tile 반응은 Ug를 바꾸지 않는다. 물의 수위에 의한 추가 위치 에너지는 이 축약 모델에 없다.

같은 반응을 다른 순서로 조합해도 화학 상태의 총 h 차이는 같아야 한다. 반응 생성기에서 원소 보존과 Δh를 정적으로 검증한다. 엔탈피 정산만으로 현실의 모든 자발 반응과 효소를 예측할 수는 없으므로 허용 반응·온도 범위는 별도 공개 표로 제한한다. 자유에너지·엔트로피·압력을 정밀하게 푼다고 주장하지 않는다.

매체128 u16 cell당 G1, 기본 작업 기억도 G1이다. 빈 유료 페이지도 질량·h·열용량을 갖는다. host interning은 각 개체의 물리 비용을 면제하지 않는다. 코드가 짧아서 유리한 압력은 실제 존재하며 복잡성 보너스로 상쇄하지 않는다.

**경계:** W는 원소·에너지 보존과 반응 가능성. P는 물질 이동·일·기록. E는 생존·순환·생산과 자원 고갈. U는 효율·열·물질 흐름이다.

## 10 빛·열·기체·액체·이동의 수식

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

parcel matrix 부피 s, 압축도 c=0..255에서 P=max(1,256-floor(s×c/64))다. parcel이 없으면 P=256이다. 노출 X=max(1,floor((Pself+Σ네이웃 min(Pself,Pneighbor))/5)), 경계 밖 P=0이다. 최소1은 닫힌 지도 모서리의 정수 절삭으로 외기 교환이 완전히 사라지는 것을 막는다. 이는 한 tile의 열·빛·확산을 요약하는 거친 차폐 모델이며 방·창고·굴뚝을 감지하지 않는다.

tile별 due 조건은 tick mod16=TileIndex mod16이다. p=(tick+1024) mod4096, f=max(0,1024-abs(p-1024)), L=floor(Lpeak×f/1024)다. due에서 L을 solar_in에 더하고 floor(L/4)를 반사된 radiation_out으로 기록한다. 남은 absorbed 중 살아 있는 actor가 있고 260≤θ≤330이면 floor(absorbed×X×min(body_g,16)/(256×16))를 photo_credit에, 나머지를 U에 넣는다. 흡수 자격이 없으면 전부 U다. credit 상한49,999의 초과분과 cap 뒤 floor(credit/256)의 누출은 열로 간다. 이후 광반응은12절대로 최대4회다.

같은 due에서 sky 온도280으로의 교환은 k=ceil(2048X/256), q=min(abs(U-280C),k×abs(θ-280))다. θ>280이면 U를 q 낮추고 radiation_out, θ<280이면 U를 q 높이고 sky_in을 증가시킨다. θ=280이면0이다. 이는 주변과의 열교환을 선형화한 모델이지 Stefan-Boltzmann 법칙의 정확한 구현이 아니다. 정확히 같은 낮·밤·외기 갱신을 headless와 visual 모두 수행한다.

자연 수송은 수평·수직 인접 edge만 처리한다. 정규 endpoint (작은TileIndex,큰TileIndex)의 LE bytes를 BLAKE3로 해시한 첫 byte의 하위3bit를 bucket으로 하고 bucket=tick mod8인 edge만 due다. east edge의 시작 x짝/홀 color0/1, south edge의 시작 y짝/홀 color2/3이다. 매 tick 고정 RNG 순열로 네 color를 처리한다. 같은 color의 endpoint는 겹치지 않아 병렬 가능하다. color 사이는 앞 결과를 읽는다. 따라서 일부 물질이 한 tick에 최대 네 edge를 거칠 수 있으며 이를 숨겨 한 edge 속도라고 부르지 않는다.

edge마다 기체 C,O,H,V 순서, 액체 W, 열 순서다. 기체는 높은 동일 종 재고에서 낮은 쪽으로 min(32,SR(abs(nA-nB)×minP,2048))를 후보로 한다. SR은63절의 결정론적 확률 반올림이며 세계의 정수 재고를 보존한다. 기체 상태방정식·풍속은 풀지 않는다. W는 높은 elevation에서 낮은 쪽으로 min(32,SR(32×minP,256)) 이하, 같은 elevation이면 min(32,SR(abs(nA-nB)×minP,8192)), 높은 쪽으로는0이다. 같은 높이의 퍼짐을 재고차로 근사하며 실제 수위·수압·유체 관성은 지원하지 않는다.

stage7에서 due parcel의 pore와 같은 tile loose를63절대로 먼저 교환한 뒤 edge 수송을 처리한다. 모든 수송은 원천 재고·목적지 용량을 제한한다. 양의 ΔUg를 갖는 기체 수송은 동반한 q_heat 안에서 지불하고 목적지에 q_heat-ΔUg를 준다. 음의 ΔUg는 목적지 U에 더한다. 후보가 열 예산에 맞지 않으면 0..32의 정수량을 큰 값부터 검사해 가능한 최대량만 적용한다. 이동할 수 없는 질량·열은 원천에 남는다. 임의 가용 에너지를 생성하지 않는다.

열전도는 θhot>θcold일 때 κ=max(1,floor(min(CA,CB)×minP/16384))+min(8,접촉한 두 matrix의 M량)이다. 후보 q=κ×(θhot-θcold), 상한 floor((θhot-θcold)×CA×CB/(CA+CB))와 Uhot 중 작은 값으로 제한한다. 이 보수적 상한으로 온도 역전을 막는다. 전도는 에너지 원천이 아니다.

기본 이동의 소산 비용은 256+ceil(mass×μeff/960)q다. parcel이 통과 가능하고 s≥4,c≥64이면 μparcel=max(64,μ-floor(c×s/16)), 표면 압축 효과는 μsurface=max(64,μ-floor(surface_c/2))다. μeff는 두 값과 μ 중 최솟값이다. 별도 ΔUg는09절대로 정산한다. 하중에는 몸체·inventory·기록·배아·운반 물체가 포함된다. 질량30,μ256,같은 고도의 비용은264q다. 복합 이동의 소산 비용은256+ceil(Σ각 이동 물질묶음(m×Manhattan이동거리×목적지 μeff)/960)이다. 회전도 최종 격자 위치의 거리로 계산하고 ΔUg는 별도로 합산한다. 단일 actor의 한 칸 이동은 위 단순식과 같다. 강하게 감쇠된 이동 모델이며 운동량·충격·도르래·유체 기관까지 구현된 것은 아니다.

**경계:** W는 복사·전도·확산·중력·마찰의 축약식. P는 이동·배치·접촉 일. E는 보온·차폐·통로·유체 경로. U는 실제 측정 후 붙이는 용도 이름이다.

## 11 Parcel·결합·형상·내부 재고

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

parcel은 matrix[10], pore_stock[10],c,orientation,네 면 결합,anchor,grip,선택 매체를 갖는다. matrix에 허용되는 종은 G,X,M,S,I뿐이며 부피 합 s≤64다. 빈 matrix 주소에 이 종을 TRANSFER하면 c=0,o=0의 parcel을 만든다. 기체나 물을 넣는 것만으로 구조물이 생기지 않는다. 전하 Q나 자동 가공 progress는 없다.

높이는 ceil(s/16), s≥16,c≥128이면 actor를 막는다. 그 밖은 발밑 layer로 공존한다. cohesion 계수는 G3,X4,M6,S4,I1, b=Σcohesion_in_i다. 두 parcel의 Bmax=min(255,bA,bB), anchor 최대=min(255,bA)다. LINK는 강도1당64q, UNLINK는128q다. 기반은 고정 지지면이며 꺼내 쓸 수 있는 무한 물질이 아니다. 모든 결합은 대칭이다.

Compress는64q당 c+1, Abrade는256q당 c-1이며 완성 단위만 적용한다. cap 때문에 실행하지 못한 일은 지불하지 않는다. matrix 제거는 c<64에서만 가능하다. 화학 반응·제거 뒤 b가 줄면 모든 연결 strength를 새 Bmax로 줄인다. matrix 추가·압축 결과가 이미 있는 actor의 tile을 차폐 점유로 바꾸면 Blocked로 거절한다. c=0이라도 s>0인 parcel은 존재한다. s=0이면 pore를 loose/residue로, tape를 corpse 매체로 내놓고 결합·grip을 해제한다. 이 반환으로 물질·정보를 두 번 생성하지 않는다.

pore 부피 용량은 K=64-ceil(s×c/256)이다. pore는 모든10종을 담을 수 있고 현재 Σv_in_i≤K여야 한다. agent의 능동 pore 입출력은 P≥32인 접촉면만 허용한다. 자연 교환·노출 표면 반응은63절을 따르며 P32에서 끊기지 않는다. 압축·matrix 추가로 기존 pore를 담을 수 없게 되면 해당 행동을 Capacity로 거절한다. 강제로 내부 물질을 사라지게 하지 않는다. 자연 반응도 생성물의 실제 수용 공간을 먼저 검사한다.

grip은 actor와 접촉 parcel의 결합이다. 연결 성분은 최대17셀까지 BFS한다. 17번째 parcel 발견,anchor>0,다른 actor grip,목적지 외부 parcel 점유이면 실패다. 최대16 parcel과 actor 하나를 원자적으로 이동할 수 있다. 회전 중심은 grip parcel이며 actor와 연결 parcel을 함께 회전하고 시작·최종 셀을 예약한다. 중간 원호는 표현하지 않는다. grip 없는 actor 방향 전환은 물질 위치를 바꾸지 않는다.

grip와 모든 face bond는64절의 하중 graph를 통과해야 한다. 각 parcel의 저항을 실제 결합 경로로 전달하지 못하면 Blocked다. actor 자신의 이동 하중은 grip에 더하지 않지만 실제 비용은 전부 지불한다. grip 자체 강도 상한은min(255,bParcel)이고 강도 비용은 다른 LINK와 같다. 실패만으로 결합이 자동 파괴되지 않는다. 구조의 보존·마모는12절의 실제 반응과 접촉 일로 발생하며 모든 구조에 같은 TTL을 걸지 않는다. 불활성 구조는 오래 남을 수 있다.

**경계:** W는 재료의 부피·차폐·결합·접촉. P는 TRANSFER/LINK/UNLINK/WORK/DRIVE. E는 보관·장벽·도구·조립체. U는 집·길·창고·기계라는 기능 해석이다.

## 12 화학 반응·상변화와 생산 가능성

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

반응은 recipe unlock이나 제작 명령이 아니다. 같은 조건의 물질에는 제작자·기술 ID와 무관하게 같은 반응이 적용된다. Rx 번호는 고정 물리 법칙을 식별할 뿐 agent가 호출하는 API가 아니다. 반응 extent1마다 아래 화학 에너지 차이와 열을 정산한다.

| ID | 반응 | 조건·에너지 |
| --- | --- | --- |
| Rx0 | C+W→G+O | 고정 생명체의 광반응,260≤θ≤330,photo_credit10,000 소비; Δh=+5000,U+5000 |
| Rx1 | G+O→C+W | 고정 생명체의 호흡,260≤θ≤340; Δh=-5000,가용1666,U+3334 |
| Rx2 | G+O→C+W | 외부 유기물의 산화,아래 온도별 속도; Δh=-5000,U+5000 |
| Rx3 | 2H+O→2V | θ≥500; Δh=-5120,U+5120 |
| Rx4 | 2X+G→2M+C+V | θ≥900,접촉 기체 저장소 O≤1; Δh=+1040,U-1040 |
| Rx5 | X+H→M+V | θ≥700,접촉 기체 저장소 O≤1; Δh=+240,U-240 |
| Rx6 | 2M+O→2X | 온도별 속도; Δh=-5600,U+5600 |
| Rx7 | W↔V | 증발θ≥374,응축θ≤372; Δh=±440,U∓440 |
| Rx8 | W↔I | 동결θ≤272,융해θ≥274; Δh=∓60,U±60 |

Rx0은 자기 inventory G 수용 공간과 자기 tile loose C1/W1, O1 출력 공간을 요구한다. Rx1은 inventory G1과 tile loose O1, C1/W1 출력 공간 및 energy+1666≤65,535를 요구한다. 기체·물 교환은 이 고정 막의 대사 통로에서만 자동이다. 일반 자원 확보·배출·건설은 프로그램의 TRANSFER다. 광반응 최대4/due,호흡 최대1/tick이다. 몸체나 코드를 자동 연료로 소화하지 않는다.

외부 반응 site의 순서는 loose 혼합물→parcel matrix→pore 내부→pore 경계→노출 매체 uid순이다. 각 site에서 Rx2.. Rx8 번호순으로 갱신한다. 응축 반응물은 **같은 compartment**에 있어야 한다. matrix·매체가 사용하는 외부 기체는 같은 tile loose뿐이다. 자연 반응은 P≥32라는 능동 접근 문턱을 사용하지 않는다. pore 내부는 자신의 기체를 쓰고,63절의 자연 교환과 별도 경계 반응이 외부 기체와의 접촉을 처리한다. 외부 물질을 소모하지 않는 무료 보충은 없다. matrix 반응의 고체 생성물은 matrix, 기체·액체 생성물은 loose로 간다. pore 내부 반응의 생성물은 pore에,loose 반응은 loose에 남는다. pore 경계의 생성물 분배는63절의 공통 상분류를 따른다. 매체 G가 산화되면 해당 페이지와 그 정보가 함께 소실된다. 살아 있는 개체의 사적 매체는 막 내부라 외부 반응 site가 아니다.

온도 bin은 <270,270..329,330..499,500..699,700..899,≥900이다. Rx2 rate=[0,1,4,256,1024,4096], Rx3=[0,0,0,4096,4096,4096], Rx4=[0,0,0,0,0,4096], Rx5=[0,0,0,0,4096,4096], Rx6=[0,1,4,32,128,512]다. 각 due의 credit 증가는63절의 SR 정수화다. 일반 site는 SR(rate×X×min(16,가능extent),256),pore 경계는 추가 투과 P를 포함해 SR(rate×X×P×min(16,가능extent),65536)를 쓴다. threshold=65,536이다. 반응물·온도·수용량·U를 다시 검사하며 최대4extent 실행하고 실제 실행당65,536 차감한다. 실행하지 못한 잔여 credit은65,535로 cap한다. credit은 에너지나 반응물의 예약이 아니다.

PoreBoundary는 Rx2/Rx5/Rx6만 수행하고 응축물은 pore,기체는 loose에서 실제 정산한다. Rx4/Rx5의 O≤1은 Matrix/Loose/Boundary이면 loose,Pore 내부이면 pore의 기체 재고다. 기본 세계의 H0 불변식으로 Rx3/Rx5가 도달 불가하다는 한계는62절에 명시한다.

Rx7/8은 반응 credit 없이 해당 방향으로 due당 최대4단위다. 한 단위씩 온도·U·용량을 재검사하므로 큰 연속 반응이 음의 U를 만들 수 없다. 반응 때문에 다음 Rx의 조건이 달라지면 이 순서대로 반영한다. 동일 site의 c 합은 원자 수 보존에 의해 변하지 않지만 부피는 달라질 수 있으므로 공간 검사는 필수다. 무효 반응은 원천 물질을 유지한다.

기술의 물리적 가능성은 연료·산소·열손실·원료 배치·생성물 회수의 조합으로 확인한다. 재료 방향만 정하면 운반하거나, 금속에 연료를 주면 자동 제련·증폭하는 규칙은 없다. 기본 profile은 전기 회로·관성 강체 기계·임의 화학 합성까지 지원하지 않는다. 그런 장치를 고수준 object로 흉내 내지 않으며 필요하면 독립적인 후속 물리 확장으로 검토한다.62절의 도달 가능성 표는 실제 반응·조작 경로와 이 한계를 구분한다.

**경계:** W는 종·반응·활성 조건과 속도. P는 이동·가열·배치·접촉. E는 연료 이용·보관·가공 절차와 생산 관계. U는 제련·생산 시설·기술 계보다.

## 13 고정 생명체·유지 비용·죽음

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

agent는 actor1셀,body G16/S2,작업 기억·프로그램 매체,inventory,가용 에너지,orientation,grip,배아≤1을 갖는다. 몸체 기본 유지32q/tick,자기 유료128cell 페이지당1q/tick이다. work memory는 G1의128cell 용량 중64cell만 주소로 노출한다. inherited·learned·retained·draft와 배아 raw/result의 Present 페이지를 모두 유지 대상으로 센다. 배아 work-memory G1도 페이지 유지1q를 요구한다. 배아 몸체는 부모 하중에 포함되지만 독립 기본 유지32q와 대사는 없다. Dropped 매체는 개체 유지 대상이 아니다.

tick 시작에 유지 비용을 내지 못하면 남은 에너지를 열로 옮기고 Inert가 되어 VM을 실행하지 않는다. stage9에서 Rx1이 가능하면 마지막 회복은 허용한다. stage10에 energy=0 또는 body_g<8이면 죽는다. 나이로 강제 사망시키지 않는다. body_g는 손상으로 감소하지만 이번 profile에는 체형 성장·기관 분화·대사 유전자 변경이 없다.

v1부터 외피 S가0이면 inventory에서 타인이 접촉 TRANSFER로 가져갈 수 있다. WORK Abrade는 외피 단위당2048q,외피가 없으면 body G 단위당8192q의 실제 접촉 일을 누적한다. 제거 물질은 같은 tile loose/residue로 반환한다. 일은 투입 때 즉시 열이며 abrasion progress는 투입한 작업량 통계라 에너지 총계에 더하지 않는다. 대상 층이 바뀌면 남은 credit을0으로 한다. 에너지 없는 공격력·HP 명령은 없다.

due에서 θ>340이면 body_g를 최대1 줄여 loose G로 내놓는다. θ<260 또는 θ>340에서는 호흡이 멈추고 광반응은 더 좁은12절 범위를 쓴다. 한 tile의 온도를 모든 물체에 공유하는 근사라 미세한 세포 내부 열구배는 없다. 가열은 대상 분류를 확인하지 않으며 같은 환경의 물질과 개체 모두 영향을 받는다.

죽으면 body·inventory·work memory·배아의 미사용 물질은 같은 tile로 반환한다. 코드·draft·배아 raw/result의 물리 매체는55절 Dropped tape로 남기고 그 G를 동시에 loose로 더하지 않는다. 가용 에너지·photo_credit·배아 투자 에너지는 U로 옮긴다. grip을 풀고 uid·부모·사망 원인 사실을 기록한다. 매체는 외부 화학·노출 마모를 계속 받는다. 사망이나 host GC가 질량을 삭제하지 않는다.

**경계:** W는 고정 대사·막·손상·생식 장치. P는 물질·힘·일·기록 조작. E는 생존 전략·탈취·포식 유사 관계. U는 행동·계통·관계이며 생물종 진화가 아니다.

## 14 Genome의 바이너리 표현

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

원시 매체의 단위는 u16 cell, instruction은 연속 두 cell의 little-endian u32다. 실행 genome는 header 16cell, descriptor 8cell×module_count, instruction 2cell×word_count 순서다. 최대 code는 4,096 instruction word, module_count는 1..64다. descriptor를 포함한 raw tape 최대 9,216cell을 예약할 수 있다. 빈 유료 용량은 실행 bytes와 구별한다.

| Header cell | 의미 | 검증 |
| --- | --- | --- |
| 0,1 | magic=0xC17A, format=2 | 불일치면 실행 불가 |
| 2,3,4 | entry_tag, module_count, code_word_count | entry 존재, 범위 안 |
| 5,6 | point_threshold, structural_threshold | u16 전체 범위, 0 허용 |
| 7..14 | 8개 구조 변이 연산자의 가중치 | 합이 0이면 구조 변이 없음 |
| 15 | reserved=0 | 비영이면 형식 실패 |

descriptor는 tag, start_word, length_word, flags=0, reserved 4개=0이다. tag는 유일해야 한다. 각 구간은 code 범위 안에서 서로 겹치지 않으며 전체 code를 빈틈 없이 덮어야 한다. length는 1 이상이다. descriptor 순서는 물리적으로 기록된 순서를 보존한다. 코드의 invalid opcode·깨진 branch·없는 CALL tag는 **형식 검증 실패가 아니라 VM 실행 실패**다.

ProgramBlob은 이 bytes의 BLAKE3-256 hash와 내용을 갖는 immutable 값이다. intern은 bytes가 완전히 같은 경우에만 공유한다. hash 충돌에는 실제 bytes 비교를 한다. 공유 host allocation과 별개로 각 agent는 실제 보유한 모든 Present 매체 페이지에 G와 유지 비용을 낸다. ProgramBlob은 실행 used prefix만 포함하며 매체 suffix와 hole은 Media에 별도로 남는다.

유전 genome, learned binding, 실행 중 retained module, 자기 draft를 구분한다. 현재 실행 module과 call frame은 재사용되지 않는 local module slot을 참조한다. tag는 binding table을 통해 slot에 매핑된다. binding 교체는 그 slot을 즉시 다른 코드로 덮어쓰지 않는다. frame·pc·MutationPlan source 참조가 모두 사라져야 회수 가능하다. 총 코드 상한과 유료 매체는 retained bytes도 포함한다.

**경계:** W는 코드 표현·검증·비용. P는 byte 읽기·쓰기·호출. E는 모듈·재조합·호환 관례. U는 genome 요약과 module 계보다.

## 15 VM ISA와 실행 의미

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

instruction의 bit 0..7=opcode, 8..11=a, 12..15=b, 16..31=imm다. reg는 16개 u16이다. signed 해석은 i16의 2의 보수다. 매 명령 시작 전에 4q와 fuel 1을 요구한다. 부족하면 EnergyStop을 fault에 기록하고 pc를 진행하지 않은 채 그 tick을 끝낸다. 외부 감각 명령은 추가 8q를 요구한다. fault는 VM 내부 status에 기록하며 host exception으로 전파하지 않는다.

| opcode | 이름과 결과 | 실패 또는 경계 |
| --- | --- | --- |
| 00 | NOP, pc+1 | 사용하지 않는 operand 무시 |
| 01 | CONST, ra=imm | u16 그대로 |
| 02 | MOV, ra=rb | imm 무시 |
| 03,04,05 | ADD,SUB,MUL, ra=ra op rb | modulo 65,536 |
| 06,07 | DIV,REM, unsigned | rb=0이면 ra=0, DivZero |
| 08,09,0A | AND,OR,XOR | bitwise |
| 0B,0C | SHL,SHR | shift=rb AND 15, SHR unsigned |
| 0D,0E,0F | EQ,LTU,LTS | ra와 rb 비교 후 ra=0 또는 1 |
| 10 | BRNZ | ra≠0이면 pc+1+i16(imm), 아니면 pc+1 |
| 11 | JUMP | pc+1+i16(imm) |
| 12 | CALL | imm tag로 이동, return pc+1 저장 |
| 13 | RET | frame을 pop, 빈 stack이면 entry 재시작 후 yield |
| 14 | LOAD | ra=mem[rb] | 
| 15 | STORE | mem[rb]=ra |
| 16 | RANDOM | ra=해당 draw의 하위 16bit |
| 17 | YIELD | pc+1 저장, 현재 tick 종료 |
| 20 | READ | imm selector, 인자는 r0..r7, 결과 ra |
| 21 | WRITE | imm selector, 인자는 r0..r7, 모든 쓰기는 효과 intent |
| 22..28 | DRIVE,LINK,UNLINK,TRANSFER,WORK,EMIT,REPRO | imm sub-op, r0..r7 인자, intent 제출 후 yield |
| 29 | COPY | 주소 간 최대 4개의 u16 cell 복사, 모든 유효 대상에서 intent |
| 나머지 | InvalidOpcode | 4q 지불, pc+1, fault 기록 |

mem 주소 64 이상은 LOAD=0 또는 STORE 무효, BadAddress다. branch가 모듈 범위 밖이면 branch를 취소하고 pc+1로 간다. pc가 module length와 같아지면 다음 실행 단위에서 4q와 fuel 1을 쓰는 implicit RET를 한다. pc가 그보다 크면 entry pc=0으로 복귀하고 yield한다. CALL 실패·stack full은 pc+1과 fault, stack 변화 없음이다.

VmFault는66절의 마지막 오류 latch다. 성공한 일반 명령은 이전 fault를 보존하며 READ Self.fault만 값을 반환한 뒤 Ok로 지운다. r14/r15는 지난 **세계 효과**의 outcome/actual량으로 tick 시작에만 갱신된다. VM fault와 세계 효과 outcome을 같은 상태로 쓰지 않는다. 세계 효과가 없었던 tick에는 r14/r15가 유지된다. READ Self.fault로 별도 fault를 읽고 지울 수 있다. opcode의 대상 ra가 r14나 r15여도 일반 register로 쓰이며 다음 효과 결과가 들어오면 덮어쓴다.

한 tick의 external query는 READ world·signal·public tape 또는 외부 source COPY를 합해 최대 4회다. 초과 명령은 실행 비용을 낸 뒤 QueryLimit과 결과 0, pc+1이다. 자신의 register·memory·이미 소유한 tape 읽기는 이 감각 상한을 쓰지 않으나 instruction과 복사 비용은 낸다. 상대 code pointer와 module hash는 감각 값이 아니다.

VmFault 번호는 Ok0,BadAddress1,DivZero2,InvalidOpcode3,StackFull4,QueryLimit5,MissingTag6,Occluded7,Boundary8,EnergyStop9다. READ 실패는 해당 fault와 결과0이며 r14/r15를 바꾸지 않는다. effect Outcome과 VmFault는 서로 다른 enum이다.

**경계:** W는 ISA와 비용. P는 이 명령들. E는 계산 전략·계층적 절차. U는 trace와 행동 추정이다.

## 16 주소 공간과 감각 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

주소는 domain+상대위치+field+index다. 일반 ABI는 r0=domain,r1=dx,r2=dy,r3=field,r4=index,r5=value/amount,r6·r7=보조다. dx/dy는 i16이고 개체의 방위를 기준으로 회전한다. 방위0..3=북·동·남·서, 상대방향에 orientation을 더해 mod4한다. TRANSFER/COPY는17절의 packed address를 쓴다.

| domain | 읽을 수 있는 정보 | 수정 경계 |
| --- | --- | --- |
| 0 Self | energy,inventory[10],age,orientation,fault,배아 상태,자기 매체 길이,port | 자기 제어값·draft,다른 개체의 비공개 정보 없음 |
| 1 Loose | 10종 재고 | TRANSFER·물리 반응 |
| 2 Parcel | matrix[10],c,방향,결합,anchor,pore capacity | TRANSFER/WORK/LINK/UNLINK |
| 3 ActorSurface | 존재,방향,외피 유무,contact port | 접촉 허용·노출 조건에 따른 TRANSFER/WORK |
| 4 Signal | 4채널 signed amplitude | EMIT |
| 5 PublicTape | 노출 cell·길이·유료 용량 | 접촉 WRITE/COPY/Resize |
| 6 OwnCode | 실제 보유 raw tape | 직접 수정 불가 |
| 7 OwnDraft | cell·길이·유료 용량 | WRITE/COPY/commit |
| 8 EmbryoTape | 자기 배아 cell·길이·상태 | Writing 상태의 기록·복제 조작 |
| 9 LocalTrace | 최근 공개 물리 동작4개 | v2 읽기 전용 |
| 10 PoreStock | parcel 내부10종 재고 | 접촉 P≥32 |
| 11 Surface | elevation,μ,surface_c,θ,실제 광입력 | WORK로 압축·가열 |

존재하지 않는 domain/field/index는 BadAddress와0이다. 구체 field 번호는52절을 따른다. UID·lineage·집단·기술 ID·세계 절대좌표는 감각에 없다. u16로 보이지 않는 열 총량은 온도로만 관측한다. Self.age_lo/hi는 실제 나이의 하위32bit이고 관찰자는 전체 u64를 보존한다.

Manhattan 반경2에서 거리2는 두 축 순서의 최단 경로 중 하나라도 중간 tile이 s<16 또는 c<128이면 보인다. 둘 다 막히면 Occluded다. 거리1은 접촉면만 읽는다. PublicTape는 c<192 또는 corpse의 노출 매체만 읽고 쓸 수 있다. c≥192의 봉인은 감각·기록 접근을 막지만 화학·열·물리 파괴를 막는 특권은 아니다. 타인의 raw code를 바로 읽는 기능은 없다.

ActorSurface port_mask는 물질0..9와 가용 에너지15의 입금 허용 bit다. bits10..14는0이어야 한다. 열린 포트는 입금만 허용하며 인출 허가는 아니다. 외피 S=0인 상대의 inventory만 접촉 인출 가능하다. 가용 에너지는 타인에게서 직접 인출하지 못한다. 계통·소유권 ACL은 없다.

**경계:** W는 국소 감각·물리 경계. P는 READ와 주소 지정. E는 탐색·의사소통·관찰. U는 숨은 정보를 agent에게 돌려주지 않는다.

## 17 Primitive의 원자적 처리 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

모든 효과 intent는 VM 실행과 별도로 기본 시도 비용 64q를 예약한다. Resize의 유료 축소는55절의 multi-tick 부분 처리 예외다. EMIT는 128q, REPRO Allocate/Finish/Release는 256q다. 시도 비용조차 없으면 intent를 제출하지 않고 Energy 결과를 다음 tick에 남긴다. 실제 변경량 비용은 성공량에만 적용한다. 허용된 partial은 아래에 명시한 것뿐이다.

| primitive | 입력 | 성공 효과·양 비례 비용 | 부분 성공 |
| --- | --- | --- | --- |
| DRIVE | sub-op Translate=0/Rotate=1, r5 방향 | 10·11절 비용, 위치 또는 조립체 방향 | 없음 |
| LINK/UNLINK | domain,dx,dy,r4 면0..3/anchor4/grip5,r5 강도량 | 11절의 강도와 q 비용 | 강도량만 축소 |
| TRANSFER | r0 source descriptor,r1 destination descriptor,r3 channel,r5 양 | 64q 시도 + 단위당16q + 양의 ΔUg | 자원·용량·에너지 내의 최대량 |
| WORK | domain,dx,dy,sub-op,r5 일 q | 64q 시도 + 실제 일 q | 일의 양만 축소 |
| EMIT | r3 채널,r5 signed amplitude | 128+abs(amplitude)×2q | 에너지 부족이면 전부 실패 |
| WRITE | target domain,dx,dy,index,value | 외부 cell당 16q + 시도64q | 단일 cell은 없음 |
| COPY | r0 source,r1 destination,r4 source offset,r5 dest offset,r6 count | cell당 32q + 시도64q, count≤4 | 가능한 연속 prefix |
| REPRO | sub-op와 상태별 인자 | 19절 | 상태별 명시 |

TRANSFER와 COPY의 descriptor는 u16 packed local address다. bits 0..3 domain, 4..6 offset code, 7..15=0. offset code는 0 자기,1 앞,2 오른쪽,3 뒤,4 왼쪽이다. domain 0은 자기 inventory,1 loose,2 matrix,3 actor inventory,5 public tape,6 own code,7 own draft,8 embryo,10 pore stock이다. 잘못된 조합은 InvalidTarget다. 자기 inventory간 같은 채널 이전과 같은 위치의 같은 저장소 이전은 Noop이며 비용만 낸다.

결과 번호는 Ok=0, Partial=1, Noop=2, Boundary=3, Blocked=4, Energy=5, Matter=6, Capacity=7, InvalidTarget=8, Stale=9, Limits=10, Format=11, Occluded=12, QueryLimit=13, Busy=14다. r15는 성공한 수량·강도·cell 수이며 이동은 1, 실패는 0이다. 여러 실패 사유가 동시에 있으면 주소→capability→접촉→generation→형식→에너지→자원→용량→점유 순서로 판정한다.

대상은 sense 시의 generation key와 좌표를 intent에 고정한다. resolve 전에 대상이 움직이거나 없어졌으면 따라가지 않는다. 같은 slot에 새 개체가 생겨도 Stale다. loose tile은 물리적 위치 대상이므로 최신 재고로 재검증한다. 실패해도 시도 비용과 이미 낸 계산 비용은 돌려주지 않는다.

COPY의 source는 PublicTape/OwnCode/OwnDraft/EmbryoTape, destination은 PublicTape/OwnDraft/EmbryoTape만 가능하다. embryo destination에는 자신의 code/draft source만 허용한다. COPY는 Think에서 최대4cell의 source bytes와 source revision을 캡처하고 query 비용을 낸다. Resolve에서 source 위치·generation·revision이 달라졌으면 Stale이며 bytes를 새로 읽어 바꾸지 않는다. source 또는 destination의 유효 범위가 짧으면 시작점부터 가능한 prefix만 처리한다. 같은 tape의 겹치는 복사는 캡처 bytes를 사용해 memmove 의미를 가지며 성공량 비용을 낸다.

DRIVE Translate/Push의 r5는 상대방향0..3이다. Rotate의 r5는 quarter-turn 수0..3으로,0=Noop,1=시계90도,2=180도,3=반시계90도다. grip 없는 Rotate는 자기 orientation만 바꾸며 시도64q 외에64q를 낸다. grip 있는 Rotate는11·64절의 조립체 비용·하중·목적지를 검사하고 actor와 모든 parcel의 위치·orientation을 같이 회전한다. r5>3은 InvalidTarget이다. WRITE Orientation(sub-op4)의 값은 절대방향0..3이다.

**경계:** W는 원자성·비용·실패 순서. P는 위 연산. E는 여러 tick의 절차. U는 시도·성공·추정을 구분한다.

## 18 WORK·쓰기·전달의 세부 제한

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

WORK sub-op0=Compress,1=Abrade,2=Heat다. Compress/Abrade는 parcel 또는 surface에만 적용한다. surface는 압축 마찰 계산에만 유효 부피4를 사용하며 물질·열용량·질량을 생성하지 않는다. ActorSurface의 Abrade는13절의 외피·몸체 접촉 일이다. Heat는 국소 tile의 U에 실제 받은 일 q를 더한다. 에너지가 있어야 가열하며 열을 직접 가용 에너지로 회수할 수 없다. 특정 자원을 특정 가공물로 바꾸는 Process 명령은 없다.

TRANSFER 물질 채널은0..9, 직접 가용 에너지는15다. 물질 실제량당16q와09절의 위치 에너지 변화를 정산한다. matrix는 고체 허용종만, pore·inventory는 유효 부피 한도를 따른다. 에너지 이전은 Self→수신 actor port 또는 Self→자기 배아만 허용한다. 양은 q이며 추가 이전 비용ceil(q/64)q다. parcel에 가용 에너지를 입금하는 숨은 전지·모터 기능은 없다.

WRITE sub-op1 ResizeTape는 r5의 목표 페이지 수를 받는다. 증설·hole 복구는 G1+초기화128q+시도64q의 원자 처리다. 축소는 마지막 페이지에 최대4cell씩 유료 재활용을 진행하며 전체128cell 처리 뒤에만 G1을 반환한다. 정확한 cursor·부분 성공·공개 공동 작업·손상 규칙은55절 단일 계약을 따른다. 취소·매체 교체는 재료 환급 경로가 아니며 기록 자체를 내려놓는다.

OwnDraft도 같은 물질·용량 원리다. WRITE Orientation(sub-op4),Self.port_mask,DraftCommit,Resize는 효과 intent다. STORE는 고정 작업 기억의 내부 명령이다. 내부 계산에서 외부 tape 수천 cell을 무료 복사하는 경로는 없다.

도구 효용은 소유자에게 주는 능력치가 아니라 실제 질량·차폐·결합·접촉·열·마찰에서 나온다. 현재 조작 범위는 actor 접촉 반경1이다. 긴 구조의 끝에서 원격 WORK를 수행하는 손·도르래·자동 조작기는 기본 모델에 없다. 후속 물리 없이는 그런 기계가 가능한 것으로 UI에 표시하지 않는다.

**경계:** W는 일·물질·정보의 비용과 접촉. P는 WORK/TRANSFER/WRITE. E는 가열·기록·공동 작업·가공 절차. U는 용도가 입증된 제작·도구 사용 후보다.

## 19 배아와 복제 상태 기계

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

REPRO sub-op는 Allocate=0, Resize=1, Finish=2, Step=3, Reopen=4, Release=5, Cancel=6이다. COPY의 destination=EmbryoTape가 실제 유전자 복사이며 소스는 자신의 code 또는 draft로 제한한다. 다른 개체에서 자식으로 곧바로 복사하지 않는다. 부모가 먼저 실제로 획득한 learned tape는 허용한다.

| 상태 | 허용 조작 | 다음 상태·효과 |
| --- | --- | --- |
| None | Allocate | G17·S2를 몸체와 기본 기억으로 투자, Writing |
| Writing | Resize, COPY, WRITE, Finish, Cancel | 실제 raw tape 작성과 에너지 이전 가능 |
| SealedWork | Step, Cancel | 확정된 변이 계획을 최대 4cell씩 유료 실행 |
| Invalid | Reopen, Cancel | 재개 시 Writing, 같은 배아 변이를 다시 추첨하지 않음 |
| Ready | Release, Reopen, Cancel | 투자 추가 또는 분리 |
| Released | 없음 | tick commit에서 부모 embryo=None |

Allocate는 code 길이를 자동 결정하거나 genome를 복사하지 않는다. 몸체 G16, 작업 기억 매체 G1, 외피 S2가 필요하다. code 용량은 Resize로 128cell씩 추가하며 페이지 G1을 별도로 낸다. COPY는 성공한 cell만 채운다. 부모가 직접 bytes를 합치거나 생략할 수 있다.

Finish는 사용 길이 r5를 고정하고 형식을 검사한다. 실패하면 Invalid이며 아직 최초 변이를 추첨하지 않는다. 유효하고 mutation_applied=false일 때만 변이 계획을 **한 번** 정해 SealedWork로 간다. 이미 변이를 완료한 배아를 Reopen으로 수정했다면 유효 형식 검사 뒤 추가 변이 없이 Ready로 간다. 부모가 수정에 실제 작업·비용을 지불하는 것은 허용하며 host가 자동 수선하지 않는다. Step은 변이 결과를 새 tape에 최대 4cell씩 쓴다. 새 tape와 옛 draft가 동시에 살아 있는 동안 두 매체 모두 물질을 지불한다. 메모리 공유로 이 비용을 지우지 않는다. 부모가 필요한 G·에너지를 제공하지 않으면 Matter 또는 Energy 결과로 대기하며 다음 tick에 자동 완료되지 않는다.

변이 결과의 실행 prefix가 원본과 같으면 Step 없이 mutation_applied=true,Ready가 된다. 매체 suffix는 그대로 보존하고 변이 입력에는 used prefix만 사용한다. 새 결과 매체가 필요한 변이는 변경하지 않은 부분도 포함해 전체 결과 bytes를 실제로 복사하며 32q/cell+시도비를 낸다. 결과 페이지를 처음 만드는 Step에는 G1과 초기화128q도 추가된다. raw 형식 검증에 필요한 마지막 비용도52절대로 지불한다. 완료 뒤 mutation_applied=true로 고정하고 source pin을 해제한다. 변이 완료 뒤 새 매체가 raw_media를 대체하고 옛 매체는 부모 tile에 같은UID/bytes의 Dropped 기록으로 남긴다. 옛 페이지 G를 배아 return_inventory에 추가하지 않는다. 반환 재고는 실제로 회수된 비기록 물질만 담으며 부분 페이지도 유료 용량으로 유지한다. 완료 검증에서 형식이 깨지면 Invalid다. mutation 때문에 Invalid가 된 사실을 지우거나 자동으로 유효 코드를 재추첨하지 않는다.

Release는 raw tape의 유효 형식, 완성 몸체, work memory, 자식에게 투자된 energy≥16,384q, 비어 있고 통과 가능한 인접 tile을 요구한다. 투자 energy는 부모→배아 TRANSFER로 실제 납부한다. 투자량 상한은 65,535q이며 시점·추가량은 프로그램이 정한다. 배아가 가진 남은 반환 재고는 자식 inventory 용량 안에서 들어가고 초과분은 출생 tile loose/residue에 남는다.

Release 예약 성공 뒤 같은 tick에 부모가 죽으면 출생을 취소한다. 배아 질량·매체는 부모의 시체로 남고 예약을 푼다. 부모가 살아 있지만 밀려서 출생 예약 tile과 인접하지 않거나 그 tile이 다시 점유되면 분리는 Stale/Blocked로 취소하고 배아는 Ready로 남는다. Release의 최종 receipt는 stage10 판정으로만 확정한다. 같은 tile의 출생 경쟁은 일반 local transaction 우선순위를 따른다. 자식 uid는59절의 stage3 생성 ticket으로 확정하며 사용되지 않은 예약 ID는 재사용하지 않는다. 자식은 다음 tick부터 실행하며 출생 tick에 추가 유지 비용을 내지 않는다.

Cancel은 배아 body G16/S2와 work-memory G1 및 반환재고를 부모 inventory의 여유만큼 돌려주고 나머지를 자기 tile에 둔다. raw/result 매체는55절대로 그대로 내려놓는다. 남은 투자 energy는 열이다. Cancel로 기록 재료나 생식 에너지를 무료 회수하지 않는다. 배아 ordinal은 Allocate 성공 때 증가하며 실패·재시도로 과거 난수 주소를 재사용하지 않는다.

**경계:** W는 공개된 생식 기계와 비용. P는 배아·cell 조작. E는 복제 시점·투자·생식 전략. U는 출생·계통이다. 완전한 생명 기원은 구현 범위 밖이다.

## 20 변이의 정확한 선택과 적용

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

조상의 point_threshold=64, structural_threshold=2,048, 구조 가중치는 [4,2,2,2,1,1,1,1]이다. u16 draw<threshold가 사건이다. 확률 0은 실제 0이다. 65,535는 65,535/65,536이며 완전 1을 뜻하지 않는다. header의 rate 자체도 변이 대상이므로 고정된 최저 변이를 숨기지 않는다.

Finish가 본 부모의 inherited genome header로 이번 배아의 확률을 정한다. 자식이 갖게 될 변이된 rate는 다음 세대부터 사용한다. point 변이는 raw tape의 32bit 쌍마다 독립 추첨하며 당첨 시 균일한 bit 하나를 뒤집는다. 끝에 u16 하나만 있으면 하위 16bit만 고른다. header·descriptor도 포함하며 형식 파괴를 허용한다. 구조 변이는 point 이전의 유효 구문 위에 0 또는 1회 적용하고, 그 결과 bytes에 point를 적용한다.

| 연산자 | 선택 범위와 결과 | 자동 보정의 한계 |
| --- | --- | --- |
| Replace | 균일한 instruction 하나를 random u32로 교체 | opcode 유효성 보장 없음 |
| Insert | module과 경계 선택, 1..8 random word 삽입 | descriptor 길이·offset만 조정 |
| Delete | module 내 1..min(8,length-1) 연속 word 삭제 | 마지막 1word는 남김 |
| DuplicateBlock | 1..16word 구간과 대상 module 경계 | 내용 그대로 복제 |
| MoveBlock | 같은 module의 1..16word와 다른 경계 | branch operand 그대로 |
| DuplicateModule | module 복제, 가장 작은 빈 tag 사용 | 기존 CALL을 새 tag로 고치지 않음 |
| SplitMerge | draw 최하위bit로 분할/결합, 조건 만족 구문만 | CALL의 의미적 수선 없음 |
| Recombine | 부모가 실제 보유한 다른 module 하나를 선택해 복사 | source가 없으면 Noop |

분할은 length≥2 module의 내부 경계, 앞 조각은 원 tag, 뒤는 가장 작은 빈 tag다. 결합은 descriptor 순서의 인접 두 module을 이어 붙이고 앞 tag만 유지한다. Recombine은 inherited·learned·현재 draft 중 유효 형식으로 소유한 module에서 균일 선택하며 후손에 없는 전역 코드풀을 조회하지 않는다. module exchange는 두 개체가 각각 공개 tape를 쓰고 COPY하는 실제 과정이지 원자적 거래 연산이 아니다.

상한 초과 또는 적용 불가능한 연산은 Noop다. 다른 연산을 성공할 때까지 재추첨하지 않는다. cap으로 변이가 거절된 비율을 관찰한다. 점점 짧은 코드만 유리하거나 cap에 몰리는 현상은 연구 결과다.

변이 계획은 interval 편집 목록으로 만들고 실제 bytes 생성은 Step의 cell budget으로 나눠 수행한다. Finish/Commit 검증에는52절의cell 비례 비용을 낸다. 변이 계획이 참조하는 실제 매체는 완료·취소까지 pin하여 수정·제거를 Busy로 거절한다. 한 번의 복제에 O(genome size) host 작업이 생기는 것은 허용하되 매 tick 모든 개체의 전체 genome를 재파싱하지 않는다. 분기 offset을 자동 relocation하지 않는 결정은 탐색을 어렵게 하지만 의미 수선을 통해 좋은 기술을 엔진이 보존하는 개입을 피한다.

**경계:** W는 구문적 편집과 잡음. P는 유전 가능한 rate와 복사 선택. E는 module 계층·재조합. U는 변이 출처와 코드의 변화다.

## 21 외부 기억·학습·모듈 채택

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

v1부터 공개 tape, 사망·폐기 기록, OwnDraft, DraftCommit을 활성화한다. 공개 매체는 노출 조건을 통과한 개체가 읽고 쓸 수 있으며 기록자의 UID로 접근 권한을 부여하지 않는다. code처럼 보이는 bytes도 채택 전에는 단순 데이터다. READ·WRITE·COPY의 물리 접근·비용이 없는 module import 경로는 없다.

DraftCommit은 WRITE sub-op2다. r5의 used prefix가 형식에 맞으면 해당 매체의 tag를 learned binding으로 채택한다. 같은 tag는 learned가 inherited보다 우선하고 새 learned에 없는 inherited tag는 유지한다. 기존 learned 목록은 교체하고 entry_tag는 승인된 draft header 값으로 정한다. 활성 binding64,전체실행 code4096word, retained 포함 slot128의상한을넘으면전부거절한다.

성공한 Commit은 PendingAdopt로 캡처하고 stage11에서효력을갖는다. 현재 module과 call frame은 old slot을계속사용하며다음 CALL은새 binding을조회한다. 빈 stack RET는새 entry로간다. 부모가그 tick에죽으면채택을취소하고물리기록을남긴다. PendingAdopt는완결 checkpoint에없다.

draft의소유권이 learned로옮겨지고 OwnDraft=None이된다. 교체된 old learned는 pc/frame/MutationPlan참조가있으면 AgentRetained로유지한다. 마지막참조가사라지면현재 tile에매체자체를내려놓으며 G를환불하지않는다. inherited매체는생존동안유지한다. 자기 draft를다시만들려면페이지재료·초기화비를다시낸다.

실행 prefix와 logical_len은다르다. used prefix뒤에남은기록을무료로지우지않고동일매체에보존한다. Resize의실제축소·재활용·hole·노출손상은 55절의단일계약이다. CancelDraft는 draft전체를내려놓으며이미쓴 bytes와지불비용을취소하지않는다.

OwnCode READ의 r6는 bank0 inherited, 1 learned, 2+retained slot이다. COPY의 source bank는 r7이다. OwnCode의 logical_len은실제기록길이이고 Self.inherited_len은상속프로그램 used prefix길이다. 존재하지않는 bank나 hole은 BadAddress다. host ProgramBlob의다른개체 code를조회하지못한다.

외부매체는 due마다 X≥64이면 exposure_clock에 4,아니면 1을더한다.16384이상이면 16384를빼고 Present page/cell/bit를균일추첨해최대 1bit손상한다. 외부화학뒤생존한매체만처리한다. 외부소유로바뀌면 clock0이다. 페이지산화는 55절의높은 index우선이며마모와산화의정보·재료차이를사실로기록한다.

자식은배아 raw에실제로쓴 bytes만상속한다. 부모의 learned binding은자동복사되지않는다. COPY출처는정확한 source UID/revision/구간이고 READ→산술→WRITE는 InfluenceCandidate일뿐확정 CopiedFrom이아니다. 상세출처·덮어쓰기·실행증거는 58절을따른다.

**경계:** W는매체·접근·채택·마모와비용. P는 cell읽기·복사·쓰기·commit이다. E는학습·전달·기록관행과문화. U는출처·사용증거·기술계보다.

## 22 신호·전도·공개 동작 흔적

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

EMIT(channel,a)는 다음 tick field에 자기 셀 a와 네 이웃 trunc(a/2)를 더한다. a는-255..255이고 비용은128+2abs(a)q다. 여러 기여는 i64로 합한 뒤 한 번[-32,767,32,767]로 clamp한다. 비용은 U로 들어간다. 신호 amplitude는 읽기용 상태이며 별도 회수 가능한 에너지 저장소가 아니다. 송신자 ID·목적지·문장·패킷·자원 좌표는 자동 부여하지 않는다.

v1 이상 두 parcel의 matrix M>0이며 결합이 있으면 수동 전도가 있다. 해당 tile의 next signal은 trunc(Sself/2)+Σ결합이웃 trunc(Sneighbor/8)+이번 방출 kernel이다. 수동 전도 없는 tile은 이전 신호를 버리고 방출 kernel만 받는다. 이웃 최대4,동기식 old→next이므로 edge당 최소1tick이다. 절댓값 기준 수동 증폭은 없다. 전도 손실을 열로 더하지 않는다. 이 field에는 물리 에너지를 따로 계상하지 않았기 때문이다.

이 통신은 비용·국소성·감쇠·지연을 제공하는 **공개된 감각 추상화**이며 실제 전자기장·음향 PDE가 아니다. 현실 유사 열·화학 모델과 정확도 수준이 다름을 manifest에 표시한다. 에너지로 자동 증폭하는 재료,잡음 제거,주소 라우팅,우선 채널은 없다. 먼 능동 중계에는 실제 개체의 READ/EMIT와 에너지 지출이 필요하다.

v2 LocalTrace는 tile당 최근4개 외부 동작의 kind,상대방향,실제량,0..255 age를 감각으로 제공한다. kind는 물질이동·힘·일·결합·방출·기록이다. opcode·module tag·의도·계통은 없다.256tick 지난 항목은0이며 동률은 CoreEvent 순서다. 이것도 물리 세계의 짧은 관측 흔적을 요약한 가정이다. 역사 archive를 읽는 통로가 아니다.

신호 이용의 기능 증거는 끄기·시간 순서 섞기·중간 방출 제거의 별도 replay 실험에서 확인한다. 상관만으로 언어나 프로토콜을 확정하지 않는다.

**경계:** W는 유료 국소 신호·수동 전도·짧은 흔적. P는 EMIT/READ. E는 의미 부여·시간적 관례·중계·조율. U는 통신망·프로토콜 후보다.

## 23 선택압·다양성과 생태적 정체

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

기본 선택은 자원 확보→유지→실제 복제다. fitness score, tournament, elite 재주입, 계통별 인구 할당, 희귀 보너스, novelty 번식 보상은 없다. 사망 이후 새로운 무작위 개체를 자동 채워 넣지도 않는다. 전멸하면 그 실행은 전멸로 끝나고 보고한다.

지리적 차이, 광 공급과 광물 위치의 불일치, 운반 하중, 차폐·기체 공급·광반응의 trade-off, 몸체 노출, 다른 개체가 만든 접근성·기록·자원 흐름을 niche의 원천으로 쓴다. 한 전략이 흔해져 그 자원을 고갈시키면 다른 전략이 유리해질 가능성이 있지만 다양성 유지가 보장되는 수식은 아니다.

| 다양성 방법 | 장점 | 단점·복잡도·성능 | 결정 |
| --- | --- | --- | --- |
| 지리·자원·상호작용 | 실제 세계 선택압 | 한 전략이 모든 곳을 차지할 수도 있음, 국소 계산 | 기본 |
| 자동 behavior embedding | 사전 직업축보다 넓은 관측 | encoder 편향·버전 변화, 분석 비용 | observer 전용 |
| novelty archive | 재방문·새 패턴 비교 | 유한 archive와 표본 누락 | 관찰 전용 |
| QD·희귀 보상 | 다양한 탐색을 적극 유도 | 외부 목표가 선택을 바꿈 | 별도 실험 profile만 |
| 환경·challenge 생성 | 새 적응 압력 | 인위적 도전 공급과 내생 혁신 혼동 | v3 분리 실험 |

기본 환경은 낮·밤이 있으며 계절·재난은 없다. 별도 변동 환경 시험은 Lpeak를 65,536tick 주기의 삼각파 3/4..5/4로 조절하며 지역 위상은 생성 시 고정한다. 인구·기술 수에 맞춘 적응형 재난은 없다. 이 실험은 고정 환경과 별도 RunId다.

**경계:** W는 유한 자원과 환경. P 추가 없음. E는 공존·공진화·경쟁과 정체 모두. U는 다양성 지표이며 보상이 아니다.

## 24 기술 축적·확산·소실의 물리 경로

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

기술 후보는 재사용 가능한 구조 또는 절차와 그 재현 정보의 묶음이다. 단발성 유용한 물건, 반복 절차, 독립 개체로 전달된 절차, 기능이 검증된 체계를 증거 단계로 구별한다. genome 길이가 늘거나 module 수가 많아진 것만으로 기술 수준을 올리지 않는다.

| 현상 | 실제 가능한 경로 | 없으면 실패하는 조건 |
| --- | --- | --- |
| 유전 축적 | 배아에 실제 code bytes 복사 | 복사하지 않은 learned tape는 제외 |
| 문화 확산 | 공개 매체·유해·신호 기록→draft→commit→사용 | 지리적 접근과 호환성·비용 |
| 모방 | LocalTrace·구조 관찰→자기 draft·binding 변경 | 모방 자체를 이미 구현한 정책 없음 |
| 재조합 | 기존 module·artifact를 실제로 연결 | 새 host opcode를 자동 생성하지 않음 |
| 전파 속도 차이 | 이동·접촉·기록 노출·수신 관례 차이 | 문명별 확산 계수 없음 |
| 소실 | 실행자·유효 기록·의존 생산 단계 제거 | 역사 archive는 복원 자원이 아님 |
| 재발견 | 독립 제작·유사 절차 재출현 | 유사성만으로 ancestry 확정 금지 |

기술 상태는 LiveUsers, DormantRecord, ArtifactOnly, DependencyUnavailable, NotObserved다. TotalLoss는 전수 가능한 관련 코드·매체·의존물 검색에서 후보를 못 찾았고 그 범위를 명시한 경우에도 **이 정의의 소실**이라고 표시한다. 표본 분석만으로 세계 전체의 지식 부재를 증명하지 않는다.

기록을 보유하지만 자원·시설이 없어 실행할 수 없는 상태와, 재료는 있지만 제조 절차가 없는 상태를 구분한다. 생산망은 실제 시간적 물질 흐름의 의존 graph다. 이름을 붙인 기술 node를 제거한다고 Core 동작이 바뀌지 않는다.

**경계:** W는 기존 매체·반응·비용뿐. P는 기존 명령. E는 발명·표준화·전달·퇴보. U는 기술의 이름과 증거 상태다.

## 25 협력·분업·교환·영토의 허용 범위

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

agent code에는 역할 변수가 없다. 역할 분화는 시간 창별 실제 동작 분포가 서로 달라지고, 물질·정보·구조 이용의 의존 관계가 지속되는 경우의 관찰 이름이다. 같은 개체가 역할을 바꿀 수 있고 하나의 행위를 여러 개체가 나누어 수행할 수 있다.

교환은 쌍방 TRANSFER·접근 허용·기록 전달의 실제 사건들이다. 거래 성사, 서비스 계약, 가격, 화폐를 코어에서 처리하지 않는다. 어떤 물건이나 신호가 반복적인 교환 매개로 쓰이는지를 관찰할 수는 있다. 반복 교환만으로 공정성·동의·의미 이해를 추론하지 않는다.

경쟁은 자원 고갈·공간 선점·장벽·막 제거·공개 재고 탈취·신호 간섭으로 나타날 수 있다. 공격력·방어력·전쟁 상태·적대 문명 목록은 없다. 무리의 색이 다르다는 이유로 상호작용을 바꾸지 않는다. 영토는 접근과 이용의 역사이며 채취 권한이 아니다.

정부·군대·도시와 같은 복합 명칭은 자동 확정하지 않는다. 집중된 이전 관계, 지속적인 강제 노출, 공유 물건의 이용, 접근의 비대칭을 먼저 보여준다. 독립 관찰자는 근거를 보고 이름을 붙일 수 있지만 이름은 다음 tick의 법칙이 아니다.

**경계:** W는 접촉·몸체·자원의 기존 규칙. P 추가 없음. E는 사회적 관계. U는 역할·경제·영토·조직 후보다.

## 26 사실 이벤트와 정보의 출처

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

CoreEvent의 공통 envelope는 tick u64, stage u8, local_sequence u32, actor_uid u64 또는 0, subject_uid u64, tile u32, kind u16이다. 세계 효과의 사건 순서는 stage→resolve color→block→priority→transaction 내부 번호다. graph 이동은 목적지 tile→uid 순서로 정렬한다. observer의 도착 wall time을 사건 순서로 쓰지 않는다.

필수 사실은 Birth, Death, GenomeCreated, CopySpan, DraftCommitted, TapeDamaged, EmbryoStateChanged, Intervention,TapeWrite,MediaLifecycle이다. TapeWrite는COPY 외의cell쓰기·재활용구간과revision을 기록한다. MediaLifecycle은생성·owner변경·이동·소멸을기록한다. 덮어쓰기와폐기를선택적trace로만남겨확정계보가끊기게하지않는다. 반복 물질 이동·힘·신호·작업은 집계와 선택적 상세 trace로 보존한다. raw 모든 VM instruction을 영구 기록하는 것을 기본 요구로 하지 않는다. 계보와 모든 행동 영상을 같은 크기 예산으로 약속하지 않는다.

CopySpan에는 source media_uid, source revision, source offset, destination media_uid, destination revision, offset, count, payload hash가 있다. media_uid는 observer가 사용할 실제 매체의 고유 번호이며 agent에게는 숨겨진다. TapeWrite의payload는media_uid,revision,start_cell,count,write_kind,payload hash이며단일WRITE는실제u16값도기록한다. MediaLifecycle은UID,이전/새owner와tile을기록한다. 같은 bytes의 독립 생성은 서로 다른 발생 node다. content hash만으로 두 지식의 전달 관계를 합치지 않는다.

프로그램 실행 사용 증거는 sample window의 (agent_uid, module content hash, call count, 실행 instruction 수)로 모은다. COPY 뒤 사용하지 않은 모듈과 실제 실행한 모듈을 구분한다. 중요한 후보의 실행 확인은 별도 replay에서 상세 trace로 재확인할 수 있다.

데이터가 없는 구간은 Gap(start_tick,end_tick,source,reason)로 표시한다. 필수 계보 event에는 gap을 허용하지 않고 disk backpressure로 runner를 멈춘다. 선택 표본의 gap은 “발생하지 않음”으로 해석하지 않는다.

**경계:** Engineering Contract는 사실과 표본의 의미. P 없음. E는 기록 대상인 실제 과정. U는 출처와 신뢰 수준이다.

## 27 물리적 도달 가능성의 witness 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

witness는 손작성한 primitive 절차와 보정용 세계로 기능이 가능한지 확인한다. RunPurpose=Fixture,별도 genome bytes/hash를 쓰고 자연 진화 조상과 교체하지 않는다. 성공 그림만으로 통과하지 않으며 전체 투입 물질·에너지·시간·인과 대조를 기록한다.

| witness | 실제 과정 | 통과 기준·대조 |
| --- | --- | --- |
| 보관 | pore에 G를 넣고 접근·산소 노출을 바꿈 | 실제 G 손실과 회수량 비교, 같은 재료·열 초기량 대조 |
| 통로 | surface나 낮은 parcel 압축 | 같은 질량·경로의 소산 q 감소, 건설비 포함 회수 시점 보고 |
| 공유 사용 | A/B가 같은 저장소에 넣고 꺼냄 | 두 uid의 실제 성공 transfer, 근접만으로 통과 금지 |
| 조립 운반 | LINK/grip/DRIVE/해제 |16셀 원자 이동과 하중·열 일치,anchor·17셀 실패 |
| 신호 중계 | 중간 개체 READ→EMIT | 직접 kernel 밖 입력·지연,중간 방출 제거 대조 |
| 기록 전달 | A의 외부 기록→B draft→commit→실행 | 비친자 출처와 실제 호출·행동 변화 |
| 누적 절차 | B가 획득 module을 조합해 C에게 전수 | A 사망 뒤 재현,관찰 archive 접근0 |
| 자연 물질 흐름 | 물을 내려보낼 통로·장벽 배치 | 동일 물질·Ug·U 정산, 흐름 방향은10절 물리식 |
| 화학 가공 | 열·X·G의 같은 matrix 배치,기체 배출 | Rx4 조건에서 실제 M과 부산물,산소·열·원료 대조 |
| 생산 관계 | 개체들이 연료·원료·생성물을 다른 위치에 옮김 | 공급 edge 제거 뒤 재현성 있는 기능 변화 |
| 기술 소실 | 사용자·유효 기록·필수 자원 경로 소실 | history가 남아도 세계에서 재현 못함을 범위와 함께 표시 |

제련 witness는 초기 M0에서 원료 배치·단열·가열·credit 누적·가공·회수까지 실제 primitive로 수행해야 한다. 가열 직후가 아니라 stage8 외기 냉각 뒤의 θ≥900과 산소·용량 조건을 요구한다. 57절의 수치 fixture는 한 단계 정산이고 완전 제조 witness와 다르다. 1회 반응만으로 장기 효율·자율 공장·자연 발견을 주장하지 않는다.

보관은 감소한 산소 접촉과 입출고 가능성을 함께 검증한다. 모든 반응을 막은 완전 봉인이 물건을 회수 불가능하게 만든다면 그 trade-off를 보고한다. 인간이 원했던 구조라서 decay를 별도로 줄여주는 코드는 금지한다.

**경계:** W/P는 같은 규칙. 손작성 witness를 E의 발견으로 세지 않는다. U는 가능성·자연 발견·기능 근거를 분리한다.

## 28 행동 novelty 분석 알고리즘

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

기본분석기는 A1.4다. 전개체의 256tick창별효과·물질·module실행집계를만들고, BLAKE3(uid)의하위 6bit=0인개체와사용자추적최대 32개체에서상세 effect열을수집한다. 관찰샘플은 core RNG와독립이다. 추적설정이달라지면 AnalysisConfig와표본범위를기록한다.

58절의 8byte token·3-gram·128개 bottom-k sketch를사용한다. 비교는합집합의 bottom-k안에서공유 hash의비율이며, 독립 sketch둘의단순 Jaccard와혼용하지않는다. 후보검색은작은 hash8개역색인과고정표본이고전역최근접을보장하지않는다. novelty<1/4,같은 family≥3/4,3창·2개체반복기준은관찰 threshold다.

빈 sketch·짧은 trace·관찰 gap을새행동으로승격하지않는다. 자원·빛·혼잡·이동 context의정확한 bin은 58절대로고정한다. novelty나분류 ID는 agent의에너지·복제·세계감각에돌려주지않는다.

archive RAM256MiB, 완성 window의디스크이동, offline전체처리와 live gap규칙을적용한다. A2 신경 embedding은 version과학습범위를명시하는미래실험이며기본 A1.4의결과를몰래대체하지않는다.

**경계:** 추가 W/P없음. E는실제새행동이다. U는표본·bias·gap이표시된분석이다.

## 29 집단·정착지·영토 감지의 확정 절차

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

4096tick 창의 실제 관계만 edge로 만든다. typed node와원자료·weight는58절을따른다. 자원이전 총질량,기록복사 총cell,구조접촉 총횟수를각각먼저합산한뒤log2로변환한다. 개별사건의log를합하지않는다. 쌍방교환은방향별최솟값을별도표시하고signal상관은tentative edge다.

공유 구조 이용은 artifact node를 경유하는 bipartite edge로 저장한다. 10,000 사용자를 5천만 pair로 확장하지 않는다. 지리적 인접·동일 계통은 표시 정보이며 관계 edge를 대신하지 않는다. edge당 마지막 4개 창의 정수 weight를 보존하고 오래된 창은 버린다.

탐지는 deterministic weighted label propagation이다. 초기 label=자신의 observer node ID. 각 pass는 node hash 우선순위를 고정하여 순차 reference 순서로 갱신한다. 이웃 label별 weight 합이 최대인 label을 택하고, 동률이면 현재 label 우선, 이후 작은 ID다. 최대 20pass 또는 변화 0에서 멈춘다. component별로 병렬 가능하나 서로 연결된 component 내 순서를 바꾸지 않는다.

20개 이상 agent, 3개 연속 창 지속, 두 종류 이상의 관계 증거가 있는 cluster만 CivilizationLikeCandidate로 승격한다. 그보다 작은 것은 interaction cluster다. 관계가 한 artifact에만 집중되면 HubDependent flag를 붙인다. 양의 상호 이익을 검증하지 않았으면 협력 문명이라고 단정하지 않는다. 최소 개체 수 20은 UI 잡음 제어이며 세계 규칙이 아니다.

이전·현재 cluster 간 agent Jaccard≥0.5인 pair를 점수 내림차순, 이전 ID 오름차순으로 greedy 일대일 연결한다. 연결된 최대 자손은 기존 ID, 나머지 split은 새 ID와 parent edge다. 여러 이전 집단과 중복되는 merge는 연결된 ID 하나를 유지하고 나머지 edge를 기록한다. soft membership은 자기 label 이웃 weight/전체 이웃 weight다. 0.6 미만은 경계 구성원으로 표시한다.

영토는 16×16 표시 셀에 방문 수·물질 입출력·공유 구조 이용을 각기 다른 heatmap으로 누적한다. 창마다 이전 값의 floor(3/4)를 유지하고 새 관측량을 더한다. 집단별 지분이 0.6 미만이면 단일 소유색을 주지 않는다. 이 지도는 이동·채취·번식의 허가 cache가 아니다. 정착지 후보는 3창 이상 유지된 밀도 구역과 persistent artifact 이용의 겹침이며 별도 city object를 만들지 않는다.

**경계:** W/P 추가 없음. E는 관계·정주·영향. U는 위 알고리즘의 집단과 영토 추정이다.

## 30 기술 계보·기능·인과 검증

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

artifact의13tile footprint,물질·압축·결합bucket,직렬화순서와반복occurrence는58절로고정한다. 서로독립적인3개물질구성과2개기여actor를요구하며,같은물건의이동·snapshot반복은새제작으로세지않는다. 회전은별도후보고내용hash일치뒤실제field를비교한다.

기술 node는 하나의 발생 사례를 중심으로 procedure hash·artifact family·입출력 흐름·복사 증거·사용 증거·관측 범위를 묶는다. CopiedFrom, MutatedFrom, RecombinedFrom은 사실 edge, ConstructedWhileExecuting은 실행 동시성 사실, DependsOn은 별도 개입 증거가 있어야 한다. R1의 ConstructedUsing만으로 인과 의존을 확정하지 않는다. SimilarTo는 계보 부모가 아닌 점선 연결이다.

생산 흐름은 물질의 tag를 모든 단위에 붙이는 대신 실제 transfer와 변환을 시공간 window로 연결한다. 같은 물질이 섞인 뒤 정확한 분자 출처를 안다고 주장하지 않는다. FIFO 등 가상의 소유 추적을 경제의 실제 거래로 해석하지 않는다. 의존 chain은 공급 경로 단절에 따른 반복적인 기능 변화로 확인한다.

인과 검증은 원본을 수정하지 않는 checkpoint branch에서 한다. 구조 배치 변경은 총 질량·에너지·점유 면적 대조를 맞춘다. 신호 검증은 단순 차단, 동일 진폭의 시간 순서 섞기, 중계 하나 제거를 각각 다른 실험으로 수행한다. 기록 검증은 바이트 수·매체 질량이 같은 무효 bytes를 대조로 쓴다. 개입 때문에 다른 외생 조건이 생긴 사실도 로그에 남긴다.

최초 기능 확인은 8개 독립 checkpoint 맥락에서 대조를 실행하고 차이의 방향·크기·실패 사례를 모두 보고한다. 의미를 강하게 확정할 통계 기준은 49절의 연구 protocol에 따르며 “8회 중 한 번 좋아짐”을 증명으로 세지 않는다. 공통 난수는 분산을 줄이는 장치일 뿐 혼돈적 분기 뒤 단일 행위의 정확한 장기 효과를 보장하지 않는다.

기술 UI는 Candidate, Reused, Transferred, FunctionSupported, CompositeSupported라는 근거 상태를 표시한다. 단계는 개발자가 만든 세계의 tech progression이 아니라 독립 관찰의 증거 강도다. 사용자의 희망 역사 순서를 event scheduler로 구현하지 않는다.

**경계:** W/P 추가 없음. E는 발명·재조합·누적 의존. U는 DAG·이름·확신 수준이다.

## 31 한 tick의 규범적 실행 순서

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

Wt는 tick t 시작의 완결 상태다. phase 순서는 PhysicsSpec 일부이며 처리 성능 때문에 바꾸지 않는다. stage0 외부 입력을 제외한 에너지·물질은 보존한다.

| stage | 처리 | 관측·원자성 |
| --- | --- | --- |
| 0 Preflight/Input | 기본 scratch 예약,입력의 provisional patch | 실패하면 Wt에서 정지 |
| 1 Maintenance | 유지 비용·Inert의 provisional delta | 아직 authoritative 쓰기·사망 확정 없음 |
| 2 Think | stage1 뒤 불변 상태 읽기 | NextVm·계산 비용·intent만 작성 |
| 3 Seal | 실제 intent별 추가 예약 후 입력·유지·VM·시도 비용 commit | 첫 authoritative 쓰기,action budget 봉인 |
| 4 LocalResolve | transfer·work·write·복제·결합 |9색 block,현재 상태 재검증 |
| 5 ComplexMotion | push·grip 조립체 이동·회전 | footprint 내 원자 이동·열 이동 |
| 6 SingleMotion | 단일 actor Translate | 의존 graph로 동시 이동 |
| 7 Fields/Transport | 신호 next→due pore 교환→edge별 물질·열 수송 | 신호 old/next,국소 교환 뒤 수송4색 |
| 8 Environment | due tile 빛→광반응→외기→외부 화학→매체 bit마모→열 손상 | tile 독립,내부 site 순서 고정 |
| 9 Metabolism | Rx1 최대1/개체 | Inert도 마지막 회복 가능 |
| 10 Lifecycle | 사망,출생 예약 최종 검사,ID·slot | newborn은 다음 tick부터 실행 |
| 11 Commit/Extract | binding·indices·ledger,signal swap,t+1 | 저장·관찰은 완결 상태만 |

stage0..2는 Wt를 수정하지 않는 입력·유지 delta overlay를 사용한다. READ의 열·재고도 이 overlay를 포함한다. stage3에서 실제 intent 유형과 최대 발생량에 맞는 pool·매체·변이 계획 용량을 예약하고 실패하면 모든 provisional 결과를 버려 Wt를 유지한다. 모든 agent가 최대 genome를 복제할 것이라는 일률적 최악 예약을 매 tick 요구하지 않는다.

READ는 stage2의 상태다. Self.energy는 자기 provisional 계산 비용을 반영하지만 남의 미래 행동은 보지 못한다. effect의 실제 재고는 resolve에서 재검사한다. 출생에 필요한 이동 열·ΔUg는 최종 분리 시점 상태로 계산하되 부모의 봉인된 남은 action budget 안에서만 지불한다.

배아 Step·페이지 재활용은 승인된 intent의 cell budget만 수행하며 환경 단계에서 무료로 진행하지 않는다. PendingAdopt의 learned binding은 stage11에서만 보인다. newborn은 출생 tick의 유지·VM·대사를 받지 않는다. 같은 tick 물질 반응은 정해진 site·reaction 순서의 직렬 reference와 동일해야 한다.

**경계:** W는 동시성과 시간 순서. P는 intent. E는 그 규칙 안의 상호작용. U는 화면 FPS와 독립인 tick이다.

## 32 충돌·이동·flux 해결 알고리즘

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

### 국소 transaction

R5는 모든 profile에서 logical resolve block=64×64, 최대 footprint 반경 R=17을 사용한다. 저장 chunk와 숫자가 같아도 서로 다른 개념이다. tick별 RNG로 x/y 원점 offset 0..63과 9색 순열을 정한다. block 좌표는 Euclidean floor division, color=(bx mod 3,by mod 3)다. 잘린 경계 block도 포함한다. 같은 색 block의 R halo는 겹치지 않는다.

각 색에서 block 내부 intent는 (priority64,actor_uid,ordinal) 오름차순으로 실행한다. reference는 같은 순서의 단일 thread다. 병렬 버전은 동일 색의 독립 block만 동시에 처리한다. footprint는 origin 반경 17 바깥으로 확장할 수 없다. 실제 접촉·조립체 탐색에 필요한 record만 scratch로 가져온다. 같은 색의 write-set이 겹치면 물리 실패가 아니라 구현 오류다.

stage 4에서 효과 대상이 바뀌면 primitive의 결과표대로 실패 또는 수량 축소한다. stage 5에서 자신이 이미 밀려 origin이 달라졌으면 자신의 이동은 Stale다. 다른 개체의 움직임을 따라가거나 모자란 비용을 미래 수입으로 충당하지 않는다.

### 밀기와 복합 이동

DRIVE sub-op 2=PushActor는 바로 앞 개체 한 명과 그 뒤 한 tile을 대상으로 한다. 뒤가 비어 있고 모두 통과 가능할 때 자신→앞, 상대→뒤로 원자 이동한다. 연쇄로 여러 actor를 밀 수 없다. 비용은 두 개체 각각의 질량·목적지·이동 거리를10절 복합식에 넣고 ΔUg를 합산한 값이다. 시도 비용과 소산 상수256q는 이 원자 transaction에서 한 번만 낸다. 상대가 잡은 parcel 또는 anchor가 있으면 실패한다. 밀린 개체의 아직 실행되지 않은 이동 intent는 Stale가 된다.

sub-op 3=PushParcel은 접촉한 최대 16셀 조립체를 한 칸 밀고 자신은 앞 tile로 이동한다. grip 없이 작용하지만 actor가 둘 이상 포함되거나 anchor가 있으면 실패한다. Translate로 grip을 끌고 가는 경우와 같은 질량·목적지 검사를 쓰고,64절의 접촉 root cohesion·모든 결합 하중도 통과해야 한다. stage 5에서는 최종 destination이 현재 비어 있어야 하며 복합 물체끼리 맞교환은 없다. 복잡한 동시 강체 contact solver는 v2 범위 밖이다.

### 단일 actor의 동시 이동

1. post-complex 상태에서 유효한 Translate 후보를 만들고 목적지 통과·비용을 확인한다.
2. 같은 목적지 후보가 여러 개면 priority 최소 하나만 남긴다. 그 후보가 나중에 막혀도 차순위를 재선발하지 않는다.
3. 목적지가 비면 성공 terminal, 이동 후보가 없는 actor로 차 있으면 실패 terminal, 후보 actor면 그 actor에 의존 edge를 만든다.
4. out-degree≤1 graph를 3색 DFS 또는 같은 결과의 iterative walk로 O(A)에 판정한다. 빈 곳으로 끝나는 경로와 닫힌 cycle은 전체 성공, 실패 terminal로 가는 경로는 전체 실패다.
5. 모든 성공 source를 지운 뒤 destination을 채우며 비용을 차감한다. swap과 4개체 순환도 허용한다.

이동 graph는 지역 block 경계를 넘을 수 있으므로 9색의 근사 이동으로 대체하지 않는다. 구현 초기에는 reference 직렬 O(A)를 사용한다. component 분리 뒤 독립 component만 병렬화할 수 있다. 전 세계를 도는 cycle도 결과를 보존해야 한다.

### 자연 수송과 일반 제한 배분

R5의 물질·열 수송은63절의 due pore 교환 뒤10절의 네 edge color를 차례로 실행하며 같은 color의 endpoint만 병렬 수정한다. source와 destination의 여유·열·위치 에너지를 각 edge의 최신 상태에서 확인한다. 더 멀리 전파되지 않게 하려고 임의로 이전 tile 상태를 읽거나, 성능 때문에 한 번의 전체 동시 diffusion으로 바꾸지 않는다.

일반 다중 후보량 제한이 필요한 분석용 fixture에서는 각 후보에floor(candidate×budget/sum)를배분하고 나머지는분수나머지내림차순,고정edge priority순으로준다. 실제 core의R5수송은이 다중edge알고리즘을호출하지않는다. 신호는합산field이며물질재고가아니다. 질량·에너지 후보를신호clamp규칙과혼용하지않는다.

**경계:** W는 경쟁 순서·graph 의미·flux. P는 동일 intent. E는 혼잡·밀기·흐름. Engineering Contract는 worker 수가 결과를 바꾸지 않는 것이다.

## 33 난수와 bit 단위 결정론

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

RNG-1은 Philox4x32-10이다. 각 round에서 M0=0xD2511F53, M1=0xCD9E8D57로 counter c0,c2의 32×32 곱을 얻는다. 결과는 (hi(M1c2) XOR c1 XOR k0, lo(M1c2), hi(M0c0) XOR c3 XOR k1, lo(M0c0))다. 다음 round의 key는 각각 0x9E3779B9,0xBB67AE85를 modulo 2³²로 더한다. 총 10round이며 처음 round는 입력 key를 그대로 쓴다.

stream key는 BLAKE3-256(ASCII "civ-rng-v1" + 0 byte + seed[16] + domain u32 LE + object_uid u64 LE + event_ordinal u64 LE)의 처음 8byte를 두 u32 LE로 해석한다. counter는 tick의 low/high u32와 draw_block의 low/high u32다. draw d는 block=floor(d/4), lane=d mod 4다. 작은 range의 균일 선택은 rejection sampling을 써 modulo 편향을 피한다. rejection draw는 같은 사건 내부 index를 증가시키며 다른 사건을 밀지 않는다.

RNG-1 domain 번호는 WorldGen=1, Controller=2, Mutation=3, Resolve=4, Environment=5, IdTie=6, Observer=7이다. 자연 정수화는63절 FluxRng-1의 별도 key/counter 주소와 domain8/9/10을 쓰며 edge별 상태를 저장하지 않는다. environment의 object는 TileIndex 또는 MediaUid, mutation의 event_ordinal은 embryo ordinal이다. 여러 용도의 draw 예약 범위는 kind별 독립 event_ordinal에 분리한다. seed·kind·uid·ordinal이 같은 두 사건이 같은 tick에 발생하는 것은 금지하고 사건 counter를 저장한다.

reference 알고리즘의 zero key·zero counter 첫 결과는 6627e8d5, e169c58d, bc57ac4c, 9b00dbd8이다. 공식 Random123 vector와 implementation port를 대조하는 gate를 둔다. stream derivation은 본 프로젝트 규격이며 Random123 자체가 정의한 주소 mapping이라고 주장하지 않는다. [Random123 공식 저장소](https://github.com/DEShawResearch/random123), [BLAKE3 공식 구현과 test vectors](https://github.com/BLAKE3-team/BLAKE3)

hash map 순회, allocator 주소, thread 도착 순서, 시스템 시간, float reduction은 코어의 결과 결정에 금지한다. 동일 tick의 AgentUid·MediaUid·ParcelUid·EmbryoUid는59절대로 stage3의정렬된potential생성ticket으로미리단조할당한다. 사용되지않은UID는빈번호로남긴다. generation counter 또는 uid의 overflow가 임박하면 RunPaused(IdExhausted)다. wrap해서 과거 참조를 재사용하지 않는다.

CoreHash는 고정 필드 순서의 canonical LE bytes를 BLAKE3로 계산한다. slot 대신 stable uid 순서의 논리 상태를 기본 hash에 사용하고 slot allocator·free list·generation은 ReplayStateHash에 추가한다. 전자는 물리 비교, 후자는 완전 저장·이어하기 검증이다. observer·render·압축 bytes·wall time은 제외한다. float는 분석과 화면에만 허용하며 analysis의 플랫폼 차이는 별도 표기한다.

WorldGen object_uid는 격자 꼭짓점의 x u32와 y u32를 결합한 u64이며 event_ordinal은 (필드번호×2+해상도번호)다. 필드는 height0,moisture1,light2,ore3,개별재고4,spawn5이고 해상도는 L64=0,L16=1이다. 셀별 재고 draw index는 G0,X1이고 나머지는 고정 생성식이다. 셀 재고 stream은 object_uid=TileIndex,event_ordinal=8이며 noise 꼭짓점 stream과 구분한다. spawn은event_ordinal=10이다. Resolve의 event_ordinal은 stage 번호다. 같은 stream 안에서 분리된 반복 사건의 counter는 Runtime state에 저장한다.

수송 color 순열은 Resolve domain,object_uid=0,event_ordinal=7에서 [0..3] Fisher-Yates를 쓴다. stage4/5의 9색 순열과 별도 stream이다.

Environment event_ordinal의 상위8bit는 객체 종류(tile0,parcel1,media2), 하위56bit는 해당 사건 counter다. counter가 범위를 넘기 전에 정지한다. block 원점·color 순열은 Resolve의 object_uid=0, actor priority는 uid≥1로 분리한다. 색 순열은 [0..8]을 i=8부터1까지 Fisher-Yates로 교환하며 각 단계의 j는 0..i 균일 선택이다. actor priority는 lane0을 하위32bit,lane1을 상위32bit로 결합한다.

**경계:** W는 난수와 정수 계산. P는 비용 있는 RANDOM. E는 결정론적이지만 관찰자에게 예측 어려운 역사. U는 재현성 증거다.

## 34 Rust 소유권과 thread 모델

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

Simulation은 coordinator thread가 단독 소유한다. step 중 phase가 끝날 때만 전체 mutable 접근을 회수한다. Think에서는 provisional 입력·유지 delta를 포함하는 불변 WorldRead를 worker에 빌려주고 각각 disjoint NextVm slice를 쓴다. local resolve에서는 동일 색 block별 scratch patch를 만들고 phase barrier 뒤 owner array의 겹치지 않는 범위로 commit한다.

| 데이터 | 쓰는 주체 | 공유 방식 |
| --- | --- | --- |
| World·AgentStore | coordinator가 phase별 분할 | immutable view 또는 disjoint mutable slice |
| ProgramBlob | batch interner | 불변 arena 참조, 회수는 tick 경계 |
| Intent·NextVm·Patch | worker별 scratch | 작업 종료 뒤 ownership 반환 |
| RenderSnapshot | extractor→viewer | Arc의 불변 페이지, 최대 3개 |
| 필수 EventBatch | coordinator→I/O | bounded ordered queue, backpressure |
| 분석 표본 | extractor→observer | bounded queue, mode별 gap 또는 대기 |
| GPU resource | Bevy main/render tasks | Simulation pointer 공유 없음 |

Visual mode의 기본 worker 수는 max(1,physical_cores-2), MAX headless는 max(1,physical_cores-1)이다. 사용자가 바꿀 수 있으며 이는 표시/운용 설정이라 물리 hash에 들어가지 않는다. Bevy와 simulation pool의 과도한 thread 경쟁은 벤치마크에서 함께 측정한다.

global Arc<Mutex<Simulation>>를 renderer와 모든 worker가 잠그는 방식은 채택하지 않는다. 성능을 이유로 unsafe alias를 도입하지 않고 먼저 safe slicing·patch를 사용한다. 국소 gather/scatter 최적화를 위해 unsafe가 필요하면 해당 경계의 disjoint proof와 serial 동등성 시험을 별도 gate로 둔다.

**경계:** Engineering Contract만 추가한다. W/P/E는 같아야 하며 U와 I/O 지연은 wall time에만 영향을 준다.

## 35 Pool·슬롯·자원 부족의 의미

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

AgentKey는 slot u32+generation u32, uid는 별도 u64다. slot 0은 none이고 free list는 작은 slot 순서다. 개체·parcel·media store마다 별도 key type을 써 섞어 쓰지 않는다. 삭제는 pending tombstone을 기록하고 lifecycle에서확정한다. 이번tick삭제된slot은다음tick부터재사용하며59절의예약순서를따른다. 새객체로공개할때generation이달라야한다.

hot 상태는 SoA, VM·64word memory는 고정 slab, media는 128cell 페이지 pool, program blob은 immutable arena다. 4,096word 상한 덕분에 변이·검증의 최악 작업량은 유한하다. active lists는 swap_remove할 수 있으나 의미적 순서는 uid·priority로 따로 정한다. 물리 결과를 active list 순서에 의존시키지 않는다.

tick 시작에는 Think와 입력 overlay용 bounded scratch를 확보하고, stage3에서 실제 intent의 허용 최대 신규 페이지·후보 출생·잔해·event·변이 계획 용량을 추가 예약한다. 보수적인 예약이 너무 크면 작업 전에 실패 가능성이 없는 작은 pool slab를 추가 확보하는 단계까지 허용하되 첫 물리 mutation 이후에는 host allocation 실패 경로가 없어야 한다. 모든 지원 allocation은 fallible 예약으로 감싼다. OS가 프로세스를 강제 종료하는 상황까지 완전 보호한다고 약속하지 않는다.

예약 실패는 완료된 Wt를 유지한 채 RunPaused(MemoryBudget)다. 실패한 agent만 복제에서 제외하거나 오래된 artifact를 삭제하지 않는다. 저장용 peak buffer를 위한 비상 예산은 기본 RAM budget의 10%이고 이것도 부족하면 마지막 완결 checkpoint와 입력 로그로 복구한다.

host GC는 참조 없는 불변 blob·빈 page만 회수한다. 생물학적으로 남아 있는 잔해·기록·죽은 agent의 물질은 GC 대상이 아니다. 역사 record를 압축해도 세계의 물건은 사라지지 않는다.

**경계:** Engineering Contract는 memory·참조 안전성. W는 공개된 물리·프로그램 cap뿐. P/E에 host memory 편향을 주입하지 않는다.

## 36 Snapshot 프로토콜과 화면 독립성

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

RenderSnapshot은 epoch, tick, world_summary, visible_chunk_pages, agent_instances, parcel_instances, overlay_pages, selection_reply를 가진 불변 message다. epoch는 load·replay 전환 때 증가하며 viewer는 과거 epoch의 응답을 버린다. 대상 선택은 AgentUid 또는 ParcelUid와 요청 sequence로 한다. 메모리 slot을 클릭 ID로 저장하지 않는다.

chunk 표시 page는 chunk_id, revision, tick, self-contained data다. v2 기본은 dirty delta chain 대신 현재 완성 page를 보내고, 변경 없는 page는 Arc 공유한다. viewer는 mailbox에서 중간 snapshot을 버려도 최신 page만으로 복원 가능해야 한다. camera 변경 때 필요한 page가 아직 없으면 loading 표시를 하며 오래된 지형을 새 상태처럼 쓰지 않는다.

최대 snapshot 3개, 선택 응답 1개 최신값, 카메라 요청 1개 최신값을 유지한다. extractor는 카메라 주변 최대 1,024 chunk page와 가까운 instance 최대 100,000개를 한 요청에 내보낸다. 초과하는 far view는 mip summary로 전환하고 임의의 물리 개체를 지우지 않는다. summary는 원본과 기준 tick을 표시한다.

필수 분석 입력과 렌더 입력은 다른 queue다. 렌더가 정지해도 core는 진행한다. 사용자가 선택한 한 개체의 상세 조회가 느리면 응답 지연을 표시하며 전체 core memory를 UI에 빌려주지 않는다. 상세 조회에는 선택 시점과 현재 생존 여부를 함께 보낸다.

**경계:** Engineering Contract는 message·revision·수명. W/P/E 불변. U는 원본 tick과 분석 tick을 구분한다.

## 37 Bevy 렌더링 구현 구조

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

Bevy는 창·입력·카메라·2D batch·UI·GPU 자원을 담당한다. simulation은 별도 Runner thread에서 돈다. Bevy FixedMain도 고정 schedule을 제공하지만 headless/MAX의 같은 core 실행과 render 독립성을 위해 여기서는 core clock으로 쓰지 않는다. adapter의 호환 API와 crate patch 버전은 첫 구현에서 공식 문서와 빌드로 확인하고 lockfile로 고정한다. 움직이는 latest 문서 주소를 재현성의 버전 고정으로 취급하지 않는다. [Bevy FixedMain](https://docs.rs/bevy/latest/bevy/app/struct.FixedMain.html)

타일별 entity는 만들지 않는다. 한 표시 chunk를 색상·재료·상태 texture와 quad로 그린다. 고도와 자원은 texture channel을 통해 shader에서 조합한다. 물질량과 강도의 색 구간은 전 세계 동일 범례를 사용한다. agent는 position·orientation·lineage_color·load·body_mask를 포함한 instance buffer다. 개별 entity는 선택 highlight·UI·카메라에만 쓴다.

표시 LOD는 화면에서 tile 크기<2px이면 Far, 2..10px이면 Mid, >10px이면 Close다. Far는 미리 집계한 4×4·16×16·64×64 mip에서 지형·자원·활동·집단 영역을 보여준다. Mid는 구조·이동 집단·실제 이동량 선분, Close는 몸체 방향·grip·물질 색·결합·신호를 보여준다. 에이전트가 없는 구조의 움직임을 애니메이션으로 발명하지 않는다.

agent 외형은 실제 몸체 잔량·방향·휴대 물질·grip을 반영한다. genotype 식별 무늬는 관찰자 표식이라고 구분한다. 실제 팔·크기·센서가 없는 v2 몸체에 기능처럼 보이는 장식을 붙이지 않는다. 몸체 확장이 별도로 승인·구현되는 경우에만 그 실제 구조를 표시한다.

지도 layer는 terrain, material, actor, selected outlines, evidence overlays 순서다. 신호·자원 흐름·territory는 기본 꺼진 overlay이며 사용자가 켠다. lineage 색과 집단 영역색은 팔레트를 분리한다. color-only 구분을 피하기 위해 선택 윤곽·무늬를 함께 사용한다. 이것은 과학 console 대신 세계가 중심인 화면을 유지하기 위한 계약이다.

기본 visual은 60FPS 목표, 고배속에서도 UI 입력은 최소 20Hz 처리한다. MAX에서 render snapshot 1Hz, summary 1Hz, particle 0, detailed overlay 0.2Hz다. headless는 extractor의 시각 작업을 완전히 끈다. thread utilization보다 실제 TPS와 메모리 대역폭을 보고 pool 수를 정한다.

**경계:** Engineering Contract는 표시. W/P 변화 없음. U는 실제 상태의 가독성과 증거 overlay다.

## 38 UI·속도·역사 탐색의 동작

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

기본 창은 1440×900을 기준으로 지도가 중심이며, 우측 선택 패널 320px, 위 제어줄 48px, 아래 최근 사건 strip 80px다. 창이 작으면 패널을 접고 지도 상호작용을 유지한다. 화면의 우선순위는 실제 세계→선택 대상→근거→통계다.

| 조작·패널 | 동작 | 오류·지연 표시 |
| --- | --- | --- |
| Space / Play | 다음 tick boundary에서 실행·정지 | pause pending 표시 |
| Step | 정확히 1tick 진행 | pause 상태에서만 가능 |
| Speed | 1x=10,5x=50,50x=500,500x=5,000TPS,MAX | requested와 achieved를 동시에 표시 |
| Click | snapshot의 실제 uid 선택 | 이미 죽으면 역사 record로 전환 |
| Zoom·Pan | 화면 데이터 요청만 변경 | missing page는 loading |
| Agent | 나이·에너지·재고·몸체·module·자손·최근 outcome | 시도/실행/역할추정 분리 |
| Group | 추정 구성원·지분·흐름·기술 증거·split/merge | 관측 범위·confidence |
| Technology | 실제 사례·출처·사용·기능 시험·휴면·소실 | 근거 없는 단계명 없음 |
| Timeline | event tick과 detected tick, 위치와 replay 링크 | 계산되지 않은 과거는 seek progress |
| Lineage | 부모·변이·module 획득·멸종 | 전체 계통과 표본 행동 구분 |

상단에는 tick, population, 현재 살아 있는 계통 수, 감지된 집단 후보 수, 사용 관측 기술 수를 표시한다. 전역 Generation은 없다. 세대 표시를 켜면 부모 깊이 또는 표본 세대 간격임을 명시한다. Population=0이면 “전멸” 상태로 정지하며 자동 복구 버튼처럼 개체를 몰래 추가하지 않는다.

500x에서 장비가 따라가지 못하면 tick skip이나 VM fuel 감소 없이 달성률을 표시한다. MAX의 높은 TPS는 관찰자의 시간 압축이며 생태 비용은 같은 값으로 유지한다. 저배속 animation은 두 확정 snapshot 사이의 위치 보간만 하고 실패 행동을 성공처럼 그리지 않는다.

timeline seek는 가장 가까운 이전 checkpoint에서 별도 replay core를 실행한다. 원본 실행은 기본 pause한다. seek를 취소하면 원본 Wt로 돌아간다. 과거에서 Continue하면 parent_run_id·parent_tick을 가진 새 RunId가 된다. 미래에 발견한 기술 이름을 과거 화면에 주석으로 표시할 수는 있지만 그 정보가 과거 agent의 READ에 들어갈 수는 없다.

UI는 Bevy UI를 기본으로 정한다. profiler·VM disassembly는 접힌 개발 패널이다. egui를 제품 기본으로 섞지 않는다. 외부 OS 파일 대화상자나 폰트 호환은 adapter 구현 문제로 한정하고 Core API를 오염시키지 않는다.

**경계:** Engineering Contract는 제어와 탐색. U는 모든 인간용 명칭. 실제 실험 개입은 별도 입력 로그와 분기를 요구한다.

## 39 저장 파일과 복구 프로토콜

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

저장 컨테이너는 확장자 .civ, magic 8byte "CIVSIM5" 뒤 0byte다. header 필드 순서는 magic[8], major u16=5, minor u16=0, flags u32, tick u64, run_id[16], physics_hash[32], manifest_len u64, section_count u32, reserved u32=0, directory_offset u64, header_hash[32]다. header 총 128byte이며 header_hash는 앞 96byte의 BLAKE3다.

manifest는 UTF-8 JSON이며 typed field schema와 정렬된 key의 canonical 표현을 쓴다. 정수는 10진 정수, seed·hash·uid의 JSON 문자열 표기는 고정 길이 소문자 hex다. object key는 UTF-8 byte 순서, 공백 없음, duplicate key 금지, NaN·float core 값 금지다. manifest는 JSON library의 임의 출력 대신 이 canonical bytes로 보존한다. physics_hash는59절 rule_text hash이며 RunConfig hash와 구분한다.

directory entry는 kind u16, schema u16, flags u32, offset u64, compressed_len u64, raw_len u64, codec u8, reserved[7], raw_hash[32]의 72byte다. codec 0=raw,1=zstd. 압축 단계는 물리 결과에 영향 없으며 DEFAULT level=3. offset·length overlap, 정수 overflow, quota 초과를 할당 전에 검사한다. 미지 mandatory section은 load 거절, optional section은 skip한다.

| kind | 필수 내용 | canonical 순서 |
| --- | --- | --- |
| 1 Rules | PhysicsSpec·RunConfig·RNG·초기 조상 bytes/hash | 명세 field 순 |
| 2 World | 44byte tile payload·sidecar·residue | TileIndex 순 |
| 3 Agents | hot·VM·memory·body·binding·key mapping | AgentUid 순 |
| 4 Programs | raw bytes·content hash·retained module | hash, bytes 순 |
| 5 Artifacts | parcel·bond·matrix·pore·매체 | ParcelUid 순 |
| 6 Embryos | state·draft·mutation plan·cursor·paid matter | 부모 uid·ordinal 순 |
| 7 Fields | current signal·반응 credit·trace | tile·channel 순 |
| 8 Runtime | tick·ID counter·generation·free list·scheduled work | type별 고정 순 |
| 9 Ledgers | 원소 초기량·에너지·외부 교환·개입·열 통계 | 명세 field 순 |
| 10 Observer | 계통·archive·evidence index·analysis config | optional, 세계 복구와 분리 |
| 11 Inputs | checkpoint 이후 필요 입력 log reference | tick·sequence 순 |

각 section record는 record_count u64 뒤 고정 field 또는 length u32+bytes의 순서다. 논리 record 순서는43절, 정확한 schema 규칙은53절을 따른다. Vec capacity, padding, host pointer, Rust enum discriminant의 메모리를 dump하지 않는다. enum 숫자는 명세의 번호로 고정하고 미지 값은 필수 core 상태에서 오류다.

save는 완결 tick에서 불변 view를 확보한다. v0 기본은 pause-copy-resume이며 worker가 압축·쓰기 한다. v1 이상의 dirty page reuse는 기존 immutable page를 공유하는 방식이며 writable state와 alias하지 않는다. 임시 파일에 section과 directory를 모두 쓰고 hash를 확인한 뒤 flush·rename한다. 작업 중 crash이면 마지막 완결 파일을 유지한다. OS·filesystem에 따라 durability 보장의 한계가 있으므로 저장 성공은 반환된 sync 결과로 판정한다.

load는 별도 Simulation에 완전 검증하고 성공한 뒤 현재 실행과 교체한다. 잘못된 파일이 현재 world를 절반만 덮어쓰게 하지 않는다. ruleset 불일치 파일은 view-only 역사 모드 또는 지원 executable로 열며 자동 변환 실행하지 않는다. 변환 도구는 source hash와 변환 버전을 가진 새 branch를 만든다.

**경계:** Engineering Contract는 직렬화·복구. W/P 불변. E를 재현하며 U metadata 소실이 물리 소실을 뜻하지 않는다.

## 40 역사·압축·삭제·되감기의 비용

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

checkpoint와 입력 로그가 정확하면 중간 tick을 다시 계산할 수 있다. 모든 tick의 이미지와 VM trace를 저장해야 replay가 되는 것은 아니다. 반대로 sparse checkpoint만으로 임의 시점의 즉시 표시를 보장할 수도 없다. 기본 checkpoint 간격은 10,000tick, 10번째마다 full이다. delta는 최대 9개 base를 참조하고 원본이 삭제되지 않도록 reference index를 유지한다.

필수 lineage 기록은 모든 Birth/Death와 genome·변이·획득 출처다. 최근 100,000 사건과 살아 있는 개체의 경로만 RAM 색인, 나머지는 압축 segment로 보존한다. segment는 65,536 record 또는 16MiB raw 중 먼저 도달할 때 닫는다. index는 uid range와 tick range다. 모든 agent의 부모 관계를 보존하는 FullHistory가 기본 연구 모드다.

100,000개체가 1,000tick마다 평균 한 자식을 만들면 1백만 tick 동안 1억 Birth다. 최소 32byte/Birth만 잡아도 3.2GB이며 실제 envelope·mutation·media 기록은 더 크다. 무제한 상세 역사와 작은 파일을 동시에 보장하지 않는다. CompactHistory는 선택 옵션이고 손실한 범위를 manifest에 기록한다.

history retention은 기본 무삭제다. 사용자가 budget을 정하면 가장 오래된 **독립적으로 복구 가능한 묶음**부터 삭제 대상으로 제시한다. 이어하기에 필요한 base checkpoint·program blob·live lineage ancestor를 먼저 참조 검사한다. 필수 기록 disk full은 RunPaused(StorageFull)이며 simulation만 계속 돌려 계통을 잃지 않는다.

실시간 선택 trace의 누락은 허용하되 count와 interval을 저장한다. 완전 분석 모드는 queue backpressure로 느려질 수 있다. 파일 크기 예상에는 checkpoint·history·artifact tape·genealogical branching을 각각 표시한다. “압축이 잘 될 것”을 capacity 계획의 전제로 쓰지 않는다.

**경계:** Engineering Contract는 저장 수명과 정보 보존. 세계 안 기술은 history 삭제와 독립적이다.

## 41 공용 API·명령·오류

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

아래는 함수 계약이며 전체 Rust 소스코드가 아니다. return과 오류의 의미를 구현 간 일치시킨다.

| API | 입력·출력 | 계약 |
| --- | --- | --- |
| Simulation::new | ValidatedSpec, InitialState → Simulation | 초기 ledger·참조 검사 후 생성 |
| Simulation::step | TickCommands → TickReport | 한 tick, 부분 완료 반환 금지 |
| Simulation::state_view | read-only committed view | phase 중간 외부 접근 금지 |
| Runner::command | ControlCommand → Ack(sequence,tick) | 적용 boundary를 응답 |
| Storage::save/load | committed state / bytes | 원자적 결과 또는 오류 |
| Extractor::render | ViewRequest → RenderSnapshot | core state 변경 없음 |
| Observer::consume | FactBatch → ObservationBatch | core output 명령 생성 금지 |
| Replay::seek | RunId,tick → progress / restored state | 취소 가능, 원본 실행 분리 |

ControlCommand는 Pause, Resume, Step, SetSpeed, Save, Load, Seek, Branch, Select, SetViewport다. 마지막 네 UI 조회는 world input이 아니다. 실제 개입은 AddMatter, RemoveAgent, ModifyMaterial, ReplaceInitialGenome 같은 명시적 ExperimentCommand이고 run provenance에 Experimental을 표시한다. 모든 개입은 질량·에너지 외부 ledger와 tick·sequence를 갖는다.

프로세스 오류는 ConfigInvalid, UnsupportedSchema, CorruptSave, MemoryBudget, StorageFull, IdExhausted, NumericLimit, InvariantViolation, WorkerFailure다. VM 오류와 섞지 않는다. InvariantViolation은 현재 tick을 정상 결과로 저장하지 않고 마지막 완결 상태·입력·진단으로 정지한다. 모든 오류에서 success 메시지를 먼저 표시하지 않는다.

CLI는 headless(config,until_tick,output), visual(save/config), replay(run,tick), analyze(run,analysis_config), inspect(save), bench(scenario,workers) 작업을 제공한다. 같은 config에 mode만 바뀌면 CoreHash가 같아야 한다. 프로파일러용 simplified physics를 benchmark의 전체 시뮬레이션 결과로 보고하지 않는다.

**경계:** Engineering Contract는 API·실패·운용. P는 VM에만 있다. 사용자 개입은 E가 아니라 기록된 실험 조건이다.

## 42 Rust workspace와 dependency 방향

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

| crate | module | 소유 책임·허용 의존 |
| --- | --- | --- |
| civ_types | ids,units,limits,events,snapshot,schema | shared 타입, std만 |
| civ_vm | isa,decode,execute,program,bindings | civ_types, 순수 VM |
| civ_core | world,agents,chemistry,thermal,transport,parcels,fields,media,embryos,mutation,resolve,motion,phase,rng,ledger | civ_types,civ_vm, 정수·hash·worker adapter |
| civ_runner | clock,commands,pool,extract,control,metrics | core 소유, observer 직접 의존 없음 |
| civ_storage | manifest,schema,codec,checkpoint,history,replay_index | DTO schema와 명시적 serialization |
| civ_observer | sampling,sketch,artifacts,groups,lineage,technology,evidence | 사실과 snapshot만 소비 |
| civ_viewer | camera,terrain,instances,overlays,picking,panels,timeline | Bevy와 Runner message |
| civ_cli | headless,replay,analyze,inspect,bench | mode 조립·파일·종료 코드 |

observer와 core 사이 순환 의존은 금지한다. schema에서 core 내부 포인터 타입을 export하지 않는다. 단위 type은 EnergyQ, ThermalQ, ChemicalQ, MatterQty, TileIndex, Tick, AgentKey, ParcelKey, MediaKey, EmbryoKey, BindingKey, ProgramHash다. Rust newtype으로 같은 u32끼리의 오용을 막는다.

dependency 선택은 Bevy(viewer), rayon(전용 pool), blake3(hash·key derivation), zstd(section codec), serde+serde_json(manifest), clap(CLI)로 고정한다. 바이너리 core section에는 serde default struct dump를 쓰지 않는다. versions는 첫 구현의 Cargo.lock과 rust-toolchain.toml에 exact pin하며 그 파일 hash를 실행 manifest에 보존한다. 라이브러리 patch 번호 자체보다 저장 bytes·RNG·ISA의 규범이 우선한다. migration 없이 checksum 알고리즘을 교체하지 않는다.

코어 release의 기준 Rust edition은 2024다. 초기 compiler·Bevy adapter·지원 OS에서 빌드 gate를 통과한 toolchain을 고정한다. 성능 측정은 release, LTO 여부·target CPU·thread 수를 반드시 기록한다. nightly 기능이나 GPU compute를 기본 의존으로 요구하지 않는다.

신경 확장은 ControllerKind enum 경계로 둔다. VM은 현재 유일한 variant=0이다. 미래 제어기는 동일 ObservationAccess·FuelBudget에서 StateDelta와 최대 한 Intent만 낼 수 있다. 모델 가중치·곱셈 비용·난수·정확한 추론 방식이 없는 Neural variant를 미리 빈 기능으로 배포하지 않는다.

**경계:** Engineering Contract는 crate·의존·toolchain. 의미 없는 host API 편의가 P를 우회해서는 안 된다.

## 43 핵심 struct와 authoritative field

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

아래는 논리 필드와 소유권이다. 내부 SoA layout과 저장 schema는 분리하며 정확한 wire 규칙은53절을 따른다. 포인터·Vec capacity·padding은 저장하지 않는다. EnergyQ는 가용 에너지, ThermalQ는 tile 열, ChemicalQ는 signed 화학 기준량이다.

| type | authoritative field·자료형 | 소유자 |
| --- | --- | --- |
| Simulation | tick:u64,spec_hash:[u8;32],World,AgentStore,ProgramArena,ParcelStore,MediaStore,EmbryoStore,FieldStore,Ledger,IdState,RuntimeState | Runner coordinator |
| World | width/height:u32,08절 tile SoA,ResidueRecord 목록,SurfaceWork 목록 | Simulation |
| AgentHot | uid:u64,tile:u32,energy:u32,inventory:[u16;10],body_g:u16,shell_s:u16,orientation:u8,port_mask:u16,flags:u16,parent_uid:u64,born_tick:u64,embryo_ordinal:u64,grip:ParcelKey?,embryo:EmbryoKey?,binding:BindingKey,draft:MediaKey?,photo_credit:u32,abrasion_credit:u32,last_outcome:u16,last_actual:u16 | AgentStore |
| VmState | regs:[u16;16],pc:u16,module_slot:u16,sp:u8,fault:u8,frames:[Frame;8] | VmPool 고정 slab |
| Frame | module_slot:u16,return_pc:u16 | VmState |
| WorkingMemory | cells:[u16;64],물리 페이지 G1 | AgentStore 고정 slab |
| BindingTable | inherited/learned ProgramView,entry_tag:u16,revision:u64,active mappings,retained slots | 개체별 cold sidecar; PendingAdopt는 tick scratch |
| ModuleSlot | slot:u16,tag:u16,media ref,start_word:u16,length:u16,retention reasons | BindingTable |
| Media | uid:u64,owner:MediaOwner,logical_len:u16,page_slots:[PageSlot],revision:u64,exposure_clock:u64,recycle:RecycleState? | MediaStore |
| PageSlot / MemoryPage | Absent 또는 Present(cells:[u16;128]) | Present 하나가 G1,주소를 hole 뒤에서 당기지 않음 |
| Parcel | uid:u64,tile:u32,matrix:[u16;10],pore:[u16;10],c:u8,orientation:u8,bonds:[u8;4],anchor:u8,gripper:AgentKey?,media:MediaKey? | ParcelStore |
| Embryo | uid:u64,parent:AgentKey,ordinal:u64,state:u8,body_g:u16,shell_s:u16,work_g:u8,energy:u32,raw_media?,new_media?,used_cells:u16,mutation_applied:bool,MutationPlan?,cursor:u16,return_inventory:[u16;10],validation? | EmbryoStore; birth reservation은 tick scratch |
| MutationPlan | source pins,operator:u8,interval edits,point edits,result_cells:u16,cursor:u16,plan_hash:[u8;32] | 편집 계획은 불변,cursor만 진행; suffix cleanup 없음 |
| ProgramBlob | hash:[u8;32],raw_cells:[u16],descriptors,decode cache | immutable arena,물리 tape와 구분 |
| FieldStore | current signal pages,ReactionCredit records(kind4 포함),LocalTrace records | Simulation |
| ReactionCredit | site kind:u8,owner uid:u64,reaction:u8,credit:u16 | sparse,0이면 생략 가능 |
| ResidueRecord | tile:u32,stocks:[u64;10],corpse media UID 목록 | World |
| Ledger | initial_atoms:[u128;5],initial_energy:i128,solar_in/sky_in/radiation_out:u128,intervention_atoms_in/out:[u128;5],intervention_energy_in/out:i128,heat_produced:u128 | Simulation |
| IdState | next agent/parcel/media/embryo:u64,slot generations,free lists | Simulation |
| RuntimeState | RNG event counters,RunConfig,scheduled inputs,capability profile | Simulation |

derived 값은 Ctile,질량 합,render dirty,active list,decoded code cache,content hash cache다. reaction credit·photo_credit·매체 노출 clock·RecycleState·VM pc·mutation cursor는 미래를 바꾸므로 authoritative다. 세계 phase 중 scratch·next signals는 완결 checkpoint에 존재하지 않는다. stage11 이후 current에 swap한 신호만 저장한다.

flags는 Inert bit0,MovedByOther bit1,DeathPending bit2이며 나머지0이다. stage11에서 Inert/MovedByOther를0으로 하고 DeathPending인 개체는 제거하므로 정상 checkpoint의 flags는0이다. key generation/uid overflow 전에는 IdExhausted로 정지한다.

ReactionSiteKind는 Loose0/Matrix1/Pore2/Media3/PoreBoundary4이며 실행 순서는63절이다. FluxDomain은 Edge8/Pore9/Reaction10이다.

핵심 enum 번호: MatterSpecies W0/C1/O2/G3/H4/X5/M6/S7/V8/I9; ControllerKind Vm0; EmbryoState None0/Writing1/SealedWork2/Invalid3/Ready4/Released5; RunMode Visual0/Headless1/Replay2/Analysis3; IntentKind Drive0/Link1/Unlink2/Transfer3/Work4/Emit5/ExternalWrite6/Copy7/Repro8/ControlWrite9. Outcome과 VmFault는15·17절이다. CoreEvent는 Birth0,Death1,GenomeCreated2,CopySpan3,DraftCommitted4,TapeDamaged5,EmbryoStateChanged6,Intervention7,MatterMoved8,BondChanged9,Motion10,Work11,SignalEmitted12,Reaction13,ThermalExchange14,TapeWrite15,MediaLifecycle16이다. TapeWrite subtype은Literal0/Recycle1,MediaLifecycle subtype은Created0/OwnerChanged1/Moved2/Destroyed3이다. Technology/Job/Civilization enum은 observer에만 있다.

MediaOwner는 AgentInherited0,AgentLearned1,AgentRetained2,AgentDraft3,EmbryoRaw4,EmbryoResult5,ParcelPublic6,Dropped7이다. 뒤의payload는59절typed owner 규칙이며Dropped는TileIndex다. 부모의 MutationPlan이 참조하는 매체는 실제 세계에서 계속 보유·지불하고 수정·축소는 Busy다. 마지막 pc/frame/plan 참조가 없어지면 retained 매체자체를현재tile에내려놓는다. host bytes를 pin한 것만으로 물리 원본을 공짜 제거할 수 없다.

**경계:** Engineering Contract는 타입·자료 소유권·저장. W는 authoritative 상태만 읽고 P가 변경한다. U의 분류는 물리 타입에 들어오지 않는다.

## 44 핵심 system·호출·병렬 작업 단위

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

| system | 읽기 | 쓰기·반환 | 병렬 단위 |
| --- | --- | --- | --- |
| reserve_tick | pool·bound·input·실제 intent·due환경·사망상한 | stage0/3 예약·UID ticket 또는 RunPaused | coordinator |
| maintain_agents | energy·매체 페이지 | provisional energy·Inert·heat delta | agent slice,열은 tile별 합산 |
| execute_vm | 불변 sense view·code | NextVm·Intent·cost | agent slice |
| seal_intents | NextVm·cost | VM·봉인 action budget | disjoint agent·열 합산 |
| resolve_local | 관련 tile·keys·매체 | 검증된 transaction patch | 같은 resolve color block |
| resolve_complex_motion | 연결 성분·점유 | 위치·열·결합 patch | 같은 color block |
| resolve_single_motion | 단일 이동 후보 | dependency graph 결과 | 직렬 O(A),독립 component 확장 |
| update_signals | current field·emits·결합 | next field | tile/chunk |
| exchange_pores | due parcel·같은 tile loose·P/K | 보존된 국소 재고·사실 | tile/parcel disjoint |
| check_load_paths | 연결·재고 질량·목적지 | 이동 가능 판정·고정 scratch | complex motion의 worker |
| transport_matter_heat | due edge endpoint | 보존된 물질·U | 같은 edge color |
| update_environment | due tile | solar·외기·반응·열손상 |16상 tile slice |
| metabolize | G·O·출력 공간·energy | Rx1·U·가용 energy | agent slice,actor1/tile |
| finalize_lifecycle | 사망·출생 예약 | stores·IDs·residue·매체 | 정렬 사건 뒤 commit |
| commit_bindings | 승인된 draft | binding·retained ref | agent별 |
| validate_commit | transaction ledger | indices·tick·hash cache | 변경 영역,주기 전수 |
| extract_record | 완결 immutable view | facts·render pages | 코어 읽기 전용 |

tile 열을 여러 worker가 동시에 증가시키지 않는다. 유지·계산 비용 delta는 actor별 출력 후 tile 순서로 합산하며 local transaction 이후는 disjoint write-set만 허용한다. 유지·사망·반응으로 생기는 모든 질량 경로는 같은 LedgerDelta API를 거친다. Δ의 합이 보존식을 만족해야만 commit한다.

전체 world를 매 tick 복제하지 않는다. phase barrier와 bounded preflight 예약 뒤 in-place 적용한다. tick 중 불변식 위반은 정상 save로 내보내지 않으며 마지막 완결 checkpoint와 입력으로 재현할 진단을 남긴다. CPU·worker 실패까지 매 tick rollback용 전체 복사를 요구하지 않는다.

CoreHash 기본 주기는1024tick/checkpoint이고 gate에서는 매 tick을 켠다. full ledger 재합산은 debug/gate와1024tick마다 수행한다. 성능 보고에는 이 검증 비용을 포함한 설정을 명시한다. 정확한 recurrence 증명이 없는 lazy 환경 갱신·GPU 근사는 기본 core에 넣지 않는다.

**경계:** W는31절 phase 순서. P는 동일 intent. Engineering Contract는 ownership·병렬 동등성. E/U의 내용을 성능 편의로 바꾸지 않는다.

## 45 메모리·성능 예산과 측정 기준

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

기본 메모리 식은 **44T+448A+128P+G+B+E+S+X+R+H**다. T=tile,A=agent,P=parcel,G=물리 매체·고유 code,B=binding,E=배아,S=신호·반응·trace,X=scratch,R=render,H=history RAM이다.448byte/agent와128byte/parcel은 설계 예산이며 Rust struct의 검증된 size_of 값은 아니다. 같은 code를 공유할 것이라는 낙관을 기본 전제로 삼지 않는다.

| 항목 | 조건 | 예상 payload |
| --- | --- | --- |
| 기본 world |512²×44 |11MiB |
| 큰 world |10M×44 /20M×44 |420MiB /839MiB |
| agent 기본 |100k×448 |42.7MiB |
| parcel 기본 |1M×128 |122MiB |
| 짧은 고유 code |100k×128instruction×4 |48.8MiB,header·매체 metadata 별도 |
| 최대 고유 code |100k×4096×4 |1.53GiB,복제용 원본·결과 중복 별도 |
| binding 예산 |100k×3KiB |293MiB |
| 신호 double buffer 최악 |20M×4×i16×2 |305MiB |
| trace 전역 최악 |20M×4×16byte |1.19GiB,실제로는 활성 tile만 |
| reaction credit |활성 site당≤5×u16+key |sparse 관리 metadata까지 실측 |

100k agent·10M tile·1M parcel은 core2..5GiB를 초기 예산 목표로 둔다. 모든 개체의 최대 유료 code·draft·배아가 동시에 존재하면 더 커진다. renderer64..512MiB,observer256..512MiB,checkpoint 시 최대 core 한 벌의 복사 peak를 별도로 잡는다.32GB 장비도 무한 역사·최대 상태를 모두 보장하지 않는다.

| scenario | 목표 TPS | release gate | 측정 조건 |
| --- | --- | --- | --- |
| S1 512²·1k |500..3000 |실측값 공개 |기본 물리·생태·관찰 |
| S2 512²·10k |100..800 |100TPS 이상 |v0 전체 규칙 |
| S3 10M·100k·1M parcel |10..100 |10TPS 이상 |v2,신호·반응 active10% benchmark |
| S4 20M·300k |2..30 |도전 목표 |모든 필드 활성·혼잡 별도 |

S3의 정확한 world는3200×3200(10.24M tile),S4는5120×4096(20.97M tile)이다. 표의10M/20M 메모리 계산은 비교용 근삿값이다. 모두 미측정 목표다. 기준 장비는12개 이상 물리 코어·32GB RAM·NVMe이며 실제 CPU·compiler·OS·profile·code 길이·출생률을 보고해야 한다. S3 gate를 통과하기 전에는 대규모 지원 완료라고 표시하지 않는다. 단순 agent 수만 같고 빈 VM·반응 없음인 결과로 대체하지 않는다.

warm-up10,000tick 뒤50,000tick을3회 측정해 median·p95/p99 tick time,VM instruction/s,반응·수송량,allocation,history I/O,RSS peak를 보고한다. 최소 한 번 full history·주기 저장을 포함한다. 최악에는 환경 스캔·edge 수송·가변 VM dispatch·매체 복사·혼잡 graph·history가 병목이다. hot SoA,상태별 batch,arena,dirty 영역을 먼저 쓰고 SIMD는 같은 연산의 연속 배열에 적용한다.

1x=10TPS이므로500x 요청은5000TPS다.100k×32instruction×5000TPS=초당160억 VM instruction만으로도 모든 규모의500x를 보장할 수 없다. MAX는 rendering/UI 빈도를 낮추고 CPU 예산을 simulation에 배분하되 물리 law를 끄지 않는다.1백만tick은20TPS에서13.9시간,200TPS에서83.3분,1000TPS에서16.7분이다.

**경계:** Engineering Contract는 용량·실측 gate. 물리 cap 외 host 부하는 P/E의 선택압으로 넣지 않는다. U는 요청 속도와 달성 TPS를 함께 표시한다.

## 46 실패 모드·예방·대가·판정

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

| 실패 | 관측 신호 | 대응과 예방 | trade-off·금지할 편법 |
| --- | --- | --- | --- |
| 단순 복제자 지배 | 복제만 증가, 기능 후보 정체 | 고갈·지리·운반·차폐의 실제 수익 확인 | 복제자 직접 벌점 금지 |
| 최단 genome만 생존 | 길이 감소와 절차 소실 | 저장·복사·실행 비용을 분리 측정, 유용 절차 회수 기간 검증 | 긴 genome 보너스 금지 |
| 복잡성 증가 없음 | 기능 의존과 재사용 정체 | material witness·변이 이웃 접근성 검증 | 길이·깊이를 기술 점수로 대체 금지 |
| 동일 전략 수렴 | 행동 sketch와 자원 사용 수렴 | 공간 분리·자원 경로·coevolution 비교 | lineage quota는 별도 실험 |
| 통신 없음 | 방출만 있거나 전혀 없음 | 수신 정보가 실제 행동에 유용한 맥락 확인 | 고정된 위험·자원 신호 의미 지급 금지 |
| 구조 spam | 많은 parcel, 기능 대조 효과 없음 | 질량·공간·가공 비용 정산, 투자 회수 측정 | 장식에 기술상 부여 금지 |
| 누적 기술 없음 | 재사용·전달 후속 조합 없음 | 외부 tape·commit·module 상호운용 witness | 자동 import·함수 추출 금지 |
| 발견자 사망 시 소실 | 코드와 제작법이 모두 끊김 | 기록·비친자 전달 경로, 마모 시간 비교 | archive에서 자동 복구 금지 |
| 환경 너무 안정 | 하나의 niche가 장기 지배 | 고정/주기 환경 실행군 비교 | 인구 기반 adaptive challenge 숨기기 금지 |
| 환경 너무 불안정 | 회수 전에 반복 소멸 | 변동 주기와 생애·회수 시간을 비교 | 생존을 위해 무기록 자원 주입 금지 |
| 규모 증가 시 붕괴 | 환경 scan·VM·graph·I/O p95 폭증 | bounded 알고리즘·SoA·active exact update·batch | 화면 밖 물리 중단 금지 |
| 고수준 게임으로 변질 | Job/TechUnlock 분기 등장 | API·의존·rule audit, 금지 enum 검사 | primitive 이름만 바꾼 shortcut 금지 |
| 열·화학·이동 에너지 exploit | ledger 미세 증가 | property test·partial/cancel/death 조합 | saturating arithmetic로 숨기기 금지 |
| 진짜 혁신과 noise 혼동 | 방출 난수·dead code가 novelty 상위 | 반복·사용·기능·대조 증거 | novelty를 fitness로 피드백 금지 |
| detector가 문명 환상 생성 | 자원 옆 경쟁자도 단일 문명 | edge 근거·지속성·tentative 표시 | 집단 색만으로 성공 선언 금지 |
| thread/RNG 재현 불일치 | worker별 hash 차이 | canonical order·domain 분리·golden replay | 차이를 자연 변동이라 설명 금지 |
| 무료 저장·수송 경로 | 열린 작은 parcel이 영구 보존,약한 결합으로 무제한 운반 | pore 교환·경계 산화·결합 하중을 공통식으로 검사 | 저장·운반 보너스 추가 금지 |
| 계산 예산 불일치 | 살아 있는 동안 최대 VM 실행만으로 적자 |65절의 일주기 가용량·야간 재고·실제 명령 수 대조 | 계산 비용 면제 금지 |
| 기록 용량 폭증 | birth/media 로그가 RAM·disk 점유 | cold segment·full checkpoint·budget | 필수 계보 silent drop 금지 |
| 배아·자기 코드 악용 | 무료 복사·할당·retained 누수 | 모든 논리 bytes의 질량·유지·복사 비용 | interning을 무료 지식으로 취급 금지 |
| 재료가 고수준 기계를 대행 | 특정 shape/name에 효율 보너스 | 모든 셀에 같은 국소 반응식 적용·ablation | 도면 hash로 기능 실행 금지 |
| 호환성 때문에 문화 단절 | 복사는 많지만 채택·실행 없음 | tag 충돌·형식·물질 비용을 관찰 | 자동 이름 번역·유용 코드만 채택 금지 |

대응이 PhysicsSpec 변경이면 새 ruleset과 별도 run이다. 성공한 실행만 골라 기존 baseline을 대체하지 않는다. 실험을 통해 원래 가설이 틀렸다고 판단하는 것도 정당한 결과이며 소프트웨어 오류와 구분해야 한다.

**경계:** W/P 수정은 버전이 있는 연구 변경. E의 실패도 보존. U는 실패의 증거를 숨기지 않는다.

## 47 개발 milestone과 완료 gate

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

모든 단계는 실제 실행 가능한 프로그램을 남긴다. milestone의 숫자는 세계 안의 시대 progression이 아니다. 제품 기능 gate가 통과해도 연구 성공을 자동 선언하지 않는다.

| 단계 | 반드시 구현할 것 | 공학 gate | 연구 확인 |
| --- | --- | --- | --- |
| M0 정합성 기반 | 타입·World·원소/에너지 ledger·열/화학·serial phase·RNG·저장 schema | 수치 fixture·보존·참조 오류 0 | 아직 주장 없음 |
| M1 v0 수직 연결 | 고정 생명체·VM·공개 조상·배아·변이·parcel·local signal·기본 Bevy 지도 | visual/headless hash·save/resume 동일 | 생존·전멸·수렴을 관찰 |
| M2 v0 완성 | 병렬 Think·resolve·화면 배속·선택·기본 timeline | worker1/2/8 일치, S2 성능 gate | 20 seed×200k tick 보고 |
| M3 v1 | 외부 tape·learned binding·수평 복사·결합 운반·전도·관계 분석 | 기록 전달·조립체·동시 이동 fixture | 공유·역할 분화·신호 이용 후보 |
| M4 v2 | LocalTrace·기술 evidence·집단·영토·완전 replay UI | 모든 27절 witness·저장·오류·S3 gate | 누적·확산·소실·서로 다른 경로 |
| M5 연구 release | 다중 seed·장기 run·최악 부하·근거 패키지 | 결함 목록·재현 파일·known limits | 성공과 실패를 함께 공개 |
| v3 | 진화 연산자 프로그램·신경 모듈·일반 물리 확장 | 별도 spec와 같은 불변식 | 개방성 확장 연구 |

v0에서도 2D 지도·pause·속도·개체 선택·실제 재고 표시를 붙인다. v2까지 숫자 console만 만든 뒤 화면을 나중에 얹는 순서는 채택하지 않는다. 반대로 고급 기술 UI를 먼저 만들어 존재하지 않는 문명을 채워 넣지도 않는다.

단계 승격의 hard stop은 보존 실패, replay 실패, 형식 오류에 대한 host panic, observer 의존, 가능한 것으로 제시한 witness 실패다. 생태 수렴은 버그로 단정할 hard stop이 아니라 연구 가설 검토 gate다. 다만 누적 기술의 실제 증거가 없으면 제품은 “누적 문명을 입증한 시뮬레이터”로 소개하지 않는다.

기간·인력·목표 OS 우선순위는 주어지지 않았으므로 일정 약속은 하지 않는다. 독립 개발이 가능한 interface는 고정되어 있으나 integration gate는 순서대로 통과해야 한다. 아직 프로젝트 source implementation을 작성하지 않았다.

## 48 수치·경계·적대적 검증 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

아래는 **향후 구현이 통과해야 할 expected behavior**다. 현재 실행한 simulator test나 생태 실험 결과가 아니다. 단일 반응·이동 사례는 해당 stage만 분리하며 별도 명시가 없으면 VM·시도 비용은 이미 낸 상태다.

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| T01 |G1/O1,energy1000,온도300,공간 충분,호흡 |G-1/O-1/C+1/W+1,energy2666,U+3334 |
| T02 |T01에서O0 |모든 재고·energy·U 그대로 |
| T03 |C1/W1,credit10000,광반응 |G+1/O+1/C-1/W-1,credit0,U+5000 |
| T04 |Rx4,X2/G1,θ900이상,O0,충분U |M2/C1/V1,원료 소모,U-1040 |
| T05 |T04에서O2 |Rx4 실행0,에너지·물질 생성0 |
| T06 |M2/O1,반응credit충족,Rx6 |X2,U+5600,원소 수 일치 |
| T07 |W1→V1 증발 조건 |U-440,m·c·원소 수 불변 |
| T08 |pore 부피63,W1 포함,증발 후보 |증기 부피 증가를 수용 못하면 원천W 유지 |
| T09 |q100 계산 비용 |energy-100,U+100,HeatProduced+100;총계 변화0 |
| T10 |열U 증가만 있고 연료·반응 없음 |가용 energy 증가0 |
| T11 |mass30,μ256,동일고도,이동 |소산264q,위치 에너지 변화0 |
| T12 |T11에서고도+2 |actor debit324q,U+264,Ug+60 |
| T13 |T11에서고도-2 |actor debit264q,U+324,Ug-60 |
| T14 |U1000,C100,이동 c10,고도동일 |원천U900,목적지U+100,C 각각-10/+10 |
| T15 |post-VM E104,TRANSFER5,여유3,같은 tile |시도64+성공2×16=96,잔여8,Partial2 |
| T16 |loose총65535에 정상 입고1 |Capacity,원천 유지 |
| T17 |T16 tile에서 죽음으로G1 반환 |residue G+1,합65536,삭제 없음 |
| T18 |DIV ra7/rb0 |ra0,DivZero,pc+1,4q 비용 |
| T19 |ADD 65535+1 |ra0,host overflow 없음 |
| T20 |범위 밖 branch /빈 stack RET |각각 취소·fault /entry pc0·yield |
| T21 |A↔B 단일 이동 |둘 다 이동,UID·열·재고 보존 |
| T22 |A→B→정지C |A/B Blocked |
| T23 |동일목적지 승자가 뒤늦게차단 |패자 재승격 없음 |
| T24 |연결16셀/17셀,목적지빈곳 |각각 성공 가능/Limits |
| T25 |같은slot 재사용 후 옛key |Stale,새 객체 무변경 |
| T26 |frame 실행 중 learned binding 교체 |old frame 유지,새 CALL은새 binding |
| T27 |같은 배아 Finish 재시도 |동일 변이 계획,재추첨0 |
| T28 |출생예약 후 부모 사망 |출생취소,배아·매체·energy 정산 |
| T29 |COPY캡처 후 source revision변경 |Stale,새bytes로몰래교체 안 함 |
| T30 |WRITE/COPY로 채택하지 않은 외부기록 |행동용 binding 자동증가0 |
| T31 |signal a=-3,다른입력0 |next 자기-3,네이웃-1 |
| T32 |signal 기여30000,30000,-30000 |최종30000,중간clamp 없음 |
| T33 |결합M A의signal8,B0 |next A4,B1;능동증폭0 |
| T34 |source budget3,후보2/2 일반분배 |우선순위로2/1,질량생성0 |
| T35 |열전도 hot/cold,양수C |전체 U 불변,온도 역전 없음 |
| T36 |상승기체ΔUg가동반열보다큼 |가능한최대정수량으로축소,음의U 없음 |
| T37 |저장에배아·plan·retained code·credit포함 |resume ReplayStateHash 일치 |
| T38 |observer on/off,카메라변경,worker1/2/8 |동일CoreHash |
| T39 |preflight pool실패/필수historyqueue가득 |완결tick에서정지/기록누락없이대기 |
| T40 |강체·매체·배아 모두가 있는무작위mutation |원소5종·총에너지·참조불변식 유지 |
| T41 |몸체·대사 바꾸는무효opcode |기존InvalidOpcode 의미,숨은 형질 진화 없음 |
| T42 |배아 source를 plan이 pin 중 매체축소 |Busy,물리G원본보존 |
| T43 |정보가든페이지 산화 |G1소모와 page 정보 삭제가 동시,중복잔해 없음 |
| T44 |부모이동 후 예약출생이비인접 |Stale,배아Ready로남음 |
| T45 |snapshot n누락 뒤n+2수신 |완결self-contained page로표시,물리 무변경 |

property test는 국소 intent·파괴·반응·생식·취소·매체 삭제·slot 재사용 조합에서 보존과 접근 범위를 검사한다. parser fuzz는 임의 genome/save가 host panic·무한 loop·과도한 allocation을 만들지 않아야 한다. getter를 그대로 다시 쓰는 test로 이 gate를 대신하지 않는다.

결정론 gate는10seed×50,000tick에서 reference와workers1/2/8, x86_64·ARM64 release를 대조한다. save/resume은 혼잡·복제·매체 수정 중 임의100개 tick 경계에서 확인한다. lazy 최적화는 eager와 최소1백만 국소 상태를 대조한다. 이 숫자는 아직 수행하지 않은 검증 계획이다.

## 49 연구 검증 계획과 수정의 경계

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

이 프로젝트는 공학적으로 닫힌 명세와 실험적으로 열린 가설을 동시에 가진다. 완성된 명세는 어떤 실패 결과도 정의된 방식으로 보여줘야 한다. 원하는 문명이 안 나올 때 그 실패를 프로그램 오류로 숨기거나 코드의 몰래 주입으로 보완하지 않는다.

### 서로 다른 세 gate

| gate | 무엇을 증명하는가 | 증명하지 않는 것 |
| --- | --- | --- |
| 구현 정합성 | 이 ruleset을 정확히 계산·저장·표시한다 | 세계가 흥미롭다는 보장 |
| 기능적 도달 가능성 | primitive 조합으로 유용한 구조·전달이 가능하다 | 진화가 그 조합을 찾는다는 보장 |
| 자연 발견과 축적 | seed scaffold 이후 독립 발생·전달·후속 재사용의 실제 사례 | 무한 개방성·모든 seed 성공 |

최초 탐색군은 seed1..20에서 각200,000tick, 사전 고정 분석 A1과 공개 조상 하나를 쓴다. 이어 validation군은 seed10,001..10,010에서1,000,000tick이다. validation 시작 뒤 규칙·threshold를 바꾸면 새 revision과 새 검증군으로 분리한다. 모두 전멸해도 결과를 보존한다. 시간·자원 부족으로 중단하면 censoring을 표시한다.

ablation은 외부 tape 없음, EMIT 없음, 연결 전도 없음, module 구조 변이 없음, 외부 화학 반응 없음의 실행군으로 나눈다. 제거군의 비용 회계와 초기 자원은 비교 가능하게 명시한다. 꺼진 기능의 에너지를 공짜로 돌려주면 다른 질문을 시험하는 것이므로 그 차이를 기록한다.

핵심 지표는 독립 보유자의 재사용, 발견자 사망 이후 재현, 실제 복사와 호출, 기존 절차를 이용한 후속 조합, 집단별 서로 다른 의존 경로, 유용한 niche 변화, 관계의 지속, 기능 후보 발생 간격이다. 단일 fitness로 합하지 않는다. 분산과 모든 seed 분포를 함께 보고한다.

기능 대조는 사전 정한 관측기간과 주 지표를 사용한다. 보관이면 총 보존 G와 회수 가능한 G, 운반이면 총 이동 질량·거리와 총 q, 절차이면 실제 재현 횟수·비용이다. baseline의 무엇을 개선했다고 주장하는지 먼저 고정한다. 8개 맥락의 paired difference를 전부 공개하고 bootstrap interval·방향 일치율을 함께 제시하되 작은 표본은 탐색적 근거로 남긴다. 확인 주장은 새 맥락 검증에서 반복되어야 한다.

규칙을 바꿀 정당한 사유는 witness 불가능, 명세 모순, 구현 결함, 자원 exploit, 실측 성능 미달, 반복 실험에서 핵심 연구 가설의 반증이다. 단지 원하는 시점에 문명이 나오지 않았다는 이유로 hidden BuildHouse나 협력 보너스를 넣는 것은 정당한 수정이 아니다.

**수정 없는 최종 설계라는 요구는 구현 의미를 먼저 고정하라는 요구로 수용한다. 연구 결과가 나올 때까지 규칙이 영원히 옳다고 선언하는 요구로는 수용할 수 없다.** R5는 그 경계를 명시한 구현 기준이며 실행 증거가 아직 없는 모든 부분을 검증 완료라고 쓰지 않는다.

## 50 원 요청의 추적표

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

| 원 요청 번호·주제 | 구현 기준 절 | 완료 판단 |
| --- | --- | --- |
| 1 primitive 철학·40 경계 | 02,04,15..18,46 | 금지 API·의존 검사 |
| 2 기술·3 성능 | 05,34..38,42,45 | 독립 코어·전체 benchmark |
| 4 world·5 환경 | 07..12 | 배열·보존·물리 witness |
| 6 agent·7 genome·8 제어 | 13..16,43 | compact state·ISA·변이 |
| 9 경제·10 복제 | 09,13,19,20 | 비용·단계적 복제 |
| 11 다양성 | 23,28,49 | 외부 fitness 없는 관찰 |
| 12 통신 | 22,27,30 | 국소 의미 없는 field·증거 |
| 13 artifact·14 건설 | 11,12,18,27 | 조립·영속·실제 효용 |
| 15 기술·16 축적·17 모듈 | 14,21,24,30 | 사용·전달·후속 조합 |
| 18 재조합·19 확산·20 소실 | 20,21,24,30 | 실제 source·접근·단절 |
| 21 문명·22 분업·23 교환 | 25,29 | detected graph·역할 미정의 |
| 24 대립·25 영토 | 13,25,29,32 | 접촉 효과·soft map |
| 26 환경 개조·27 외부 기억 | 10..12,18,21 | 구조·표면·매체가 후대에 영향 |
| 28 혁신·29 기술 UI·30 계통 | 26,28..30,38,40 | 사실과 해석·전수 lineage |
| 31 시각화·32 외형·33 UI | 36..38,54 | 실제 지도·LOD·선택 |
| 34 timeline·35 저장 | 33,38..40 | 과거 tick·정확한 resume |
| 36 modes·37 병렬·38 cache | 31..35,41..45 | 동일 hash·bounded cost |
| 39 실패 | 46,48,49 | 예방·대가·반증 경로 |
| 41 로드맵·42 비교 | 03,47 | 선택 완료·단계 gate |
| 43 이상적 역사·44 성공 | 24,27,49 | 사전 progression 없음·증거 |
| 45 architecture·타입·소유권 | 05,31..35,39,41..44,52,53 | 구현 계약·source 미작성 |

R5의 범위 밖은 무생물에서의 생명 기원, 신체·기관·대사 진화, 연속 유체·정밀 강체·현실 전체 화학, 전기 회로·관성 기계, 큰 신경망, 무제한 몸체, 실행 중 새 host 물리 법칙 생성, 모든 규모의500x, 무한 상세 역사·항상 작은 저장 파일이다. 범위 밖 기능을 빈 enum이나 그럴듯한 UI로 구현 완료처럼 표시하지 않는다.

## 51 근거·결정 기록·현재 상태

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

태양 유입과 지구의 열 방출이라는 에너지 흐름은 현실 유사 모델의 방향을 뒷받침한다. 이 문서의 격자·주기·온도·교환 계수까지 NASA가 검증했다는 뜻은 아니다. [NASA, Earth’s Radiation Budget](https://science.nasa.gov/ems/13_radiationbudget/)

빛으로 물과 이산화탄소를 화학 연료로 바꾸는 발상은 광합성과 태양 연료 연구에 근거한다. 여기서는 이를 고정 생명체의 축약 반응으로 사용하며 실제 효율·생체분자를 재현하지 않는다. [DOE, Solar Fuels](https://www.energy.gov/science/doe-explainssolar-fuels)

화학 경로와 무관한 상태함수 기반 에너지 정산을 채택한다. 표의 h는 프로젝트 모델 값이며 실제 재료 예측에 사용하지 않는다. [NIST, Thermochemistry](https://nvlpubs.nist.gov/nistpubs/sp958-lide/html/093-096.html)

자가 복제 프로그램의 실행에는 [Avida](https://avida.devosoft.org/)라는 선례가 있다. 개방형 진화에서 현재 가능성 안의 탐색과 가능성 자체의 확장은 구별해야 한다. 이 설계의 유한 재료·고정 신체·VM은 제약이며 무한 혁신을 보장하지 않는다. [Taylor, Evolutionary Innovations and Where to Find Them](https://arxiv.org/abs/1806.01883)

| ADR | 선택 | 개정 조건 |
| --- | --- | --- |
| 01 |register VM·가변 유료 tape |ISA/ruleset 명시 개정 |
| 02 |chunked dense SoA·독립 viewer |상태 동등성 유지 |
| 03 |원소·열·화학·Ug ledger |반응·수송 보존 재검증 |
| 04 |국소 transaction·이동 graph |golden replay 재검증 |
| 05 |물리 복사·채택·매체 비용 |provenance와 유효 접근 재검증 |
| 06 |기술·문명·직업은 observer |core 의존 금지 |
| 07 |novelty 관찰 전용 |보상 도입 시 별도 선택 실험 |
| 08 |최종 v2,v3 후속 연구 |scope·gate 명시 변경 |
| 09 |FullHistory·backpressure |손실 범위와 mode 명시 |
| 10 |구현·가능성·자연 발견 구별 |증거 없는 성공 주장 금지 |
| 11 |자동 가공·운반·유료 재료 증폭 제거 |새 장치는 일반 물리 법칙부터 검토 |
| 12 |고정 생명체,행동·문화 진화 유지 |신체·기관·대사 진화는 별도 범위 결정 |

현재 납품물은 **R5 구현 기준 설계**다. 문서·식·ABI·schema·기대값을 감사하고62..67절에 조합 조건과 수정 사항을 기록했지만 실제 Rust simulator,performance benchmark,장기 생태 실험은 아직 수행하지 않았다. 이를 “수정이 절대로 필요 없는 검증된 최종 결과”라고 부르지 않는다. 발견되는 명세 결함은 erratum과 revision으로 수정하고 새 물리 결과를 이전 실행의 증거와 섞지 않는다.

이 문서를 구현하는 목표는 세계·기록·화면·실험 도구를 완성하는 것이다. 그 세계에서 문명과 누적 발명이 실제로 출현하는지는 별도의 기록된 실행으로 판정한다. AAA 게임의 제작 규모·미술 완성도·모든 상호작용을 약속하는 설계가 아니다.

## 52 I/O ABI의 완결 표

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

이 절은15..21절의 실행 의미를 숫자 주소로 고정한다. README식 예시가 아니라 ABI 계약이다. READ의 imm은0만 유효하다. 정의되지 않은 imm/field/domain 조합은 BadAddress와0이다. 효과 opcode의 미정 sub-op는 InvalidTarget이다. 쓰지 않는 register는 읽지 않으며0을 강제하지 않는다. 사용한 enum 인자만 검증한다.

### READ field 번호

| domain | field 번호 | index·보조 인자 |
| --- | --- | --- |
| Self0 |0 energy,1 inventory,2 age_lo,3 age_hi,4 orientation,5 fault,6 embryo_state,7 inherited_len,8 draft_len,9 port_mask,10 last_actual,11 body_g,12 shell_s |inventory의r4=종 0..9,다른index는0 |
| Loose1 |0 stock |r4=종 0..9 |
| Parcel2 |0 matrix,1 c,2 orientation,3 bond,4 anchor,5 pore_capacity,6 present |matrix index0..9,bond0..3,그 외 0 |
| ActorSurface3 |0 present,1 orientation,2 shell_present,3 port_mask |r4=0 |
| Signal4 |0 amplitude |r4=채널0..3,i16 bit표현을u16로반환 |
| PublicTape5 |0 cell,1 logical_len,2 capacity,3 page_present,4 recycle_remaining |cell/page index=r4,그 외0;r6=매체선택 |
| OwnCode6 |0 cell,1 logical_len,2 capacity |r6=bank0 inherited,1 learned,2+retained slot;cell index=r4 |
| OwnDraft7 |0 cell,1 logical_len,2 capacity,3 recycle_remaining |cell index=r4,그 외0 |
| EmbryoTape8 |0 cell,1 logical_len,2 capacity,3 state,4 energy,5 return_stock |cell index또는return종=r4,그 외 0 |
| LocalTrace9 |0 kind,1 relative_direction,2 actual,3 age |r4=최근순slot0..3 |
| PoreStock10 |0 stock,1 capacity |stock r4=종 0..9,capacity는0 |
| Surface11 |0 elevation,1 μ,2 surface_c,3 θ,4 current_light |r4=0,고도는i16 bit표현,큰θ는65535포화 |

PublicTape r6=0은 parcel 매체,1..65535는 해당 tile Dropped media UID 오름차순의 ordinal이다. 이보다많은매체도세계에서삭제하지않지만해당tick 감각으로선택되는범위는앞65535개다. 이 값은 세계에 노출된 물리 매체 선택자이며 실제 UID 자체를 반환하지 않는다. ordinal이 Think 이후 다른 매체를 뜻하게 되면 capture한 key/revision으로 Stale를 판정한다. COPY의 public source selector는 r7,public destination은 r3이다. own code source bank도r7이다. 다른 domain의 해당 selector는0만 유효하다.

present field는 해당 객체가 없으면0/Ok이고, 없는 객체의 다른 field는 BadAddress다. 없는 자기 draft/embryo의 state·len·capacity·recycle_remaining은0/Ok지만 cell은 실패한다. page_present는 capacity 안의 Present면1, hole이면0이며 index 범위 밖이면BadAddress다. recycle_remaining은 cursor가 있으면128-next_cell,없으면0이다. r14/r15는 READ 결과가 아니라 이전 world effect receipt다. Self.fault는 READ 직전 fault를 반환한 뒤 명시적으로 Ok로 지운다. 다른 성공 명령으로는 지우지 않는다.

### WRITE와 구조 조작

| opcode/sub-op | 인자 | 처리 |
| --- | --- | --- |
| WRITE0 Cell |r0=5/7/8,r1/2위치,r4index,r5value,r6공개selector |범위·접촉·봉인·Writing검사,1cell 기록 |
| WRITE1 Resize |r0=5/7/8,r1/2위치,r5목표페이지,r6selector |55절의증설·hole복구·유료재활용,embryo는Writing만 |
| WRITE2 Commit |r5사용rawprefix길이 |OwnDraft prefix검증·PendingAdopt,suffix보존 |
| WRITE3 CancelDraft |추가 인자 없음 |draft전체를Dropped로내려놓음,bytes·비용복구없음 |
| WRITE4 Orientation |r5절대방위0..3 |자기방위변경,위치변경 없음 |
| WRITE5 Port |r5mask |bits0..9/15만허용 |
| LINK/UNLINK |r0=2,r1/2대상parcel,r4면0..3/anchor4/grip5,r5강도 |면결합은인접parcel,grip은자신과parcel |
| WORK0/1 |r0=2/11,r1/2위치,r5일q |Compress/Abrade,ActorSurface3에는Abrade만 |
| WORK2 Heat |r0=0/1/2/3/11,r1/2위치,r5일q |선택된tile의U에실제일추가 |

LINK의 면 번호는 세계 절대방위0..3이다. 양 endpoint가 actor 접촉 반경 안에 있어야 한다. grip는 actor당1개,parcel당1개이며 다른 grip가 있으면Busy다. UNLINK grip는 자기 grip에만 작용한다. 대칭 strength의 실제 증감량을r15에 반환한다. 동일한 요청을 반복해 불변인 경우Noop다.

Compress/Abrade의 r5는 요청한 최대 일이다. 정수 완성 단위가 없는 요청은Noop이며시도비만낸다. ActorSurface abrasion은13절의 부분 progress를허용한다. Heat는1q단위다. WRITE4방위변경은시도64+추가64q,동일방위는Noop이며시도64만낸다. WRITE5는시도64만낸다.

### TRANSFER·COPY·배아 주소

packed local address는bits0..3domain,bits4..6offset,나머지0이다. offset0=자기tile,1앞,2오른쪽,3뒤,4왼쪽이고5..7은무효다. Self/OwnCode/OwnDraft/Embryo는offset0만허용한다. 지리 이동은source와 destination이 각각actor에접촉해야 하며09절열·Ug를정산한다.

TRANSFER source/destination 물질 domain은0 SelfInventory,1 Loose,2 Matrix,3 ActorInventory,8 EmbryoReturn,10 Pore다. 정상 양은r5,종은r3이다. source=3이면상대외피S0을요구하고destination=3이면종port허용을요구한다. source/dest8은자기배아return_inventory만이며몸체·코드원료를이경로로직접빼지못한다. species0..9의정상입고여유·pore접촉·matrix종제약을모두검사한다. source와 destination이같은 저장소면Noop다. 이전물질은actorinventory를중간에무료경유하지않는다.

energy channel15는source0만,destination3수신port또는8배아energy만허용한다. 별도량비용ceil(q/64)와실제보낸q를둘다봉인budget에서낸다. 배아의반환재고와투자energy는같은domain이지만channel로구분한다. energy는질량이없어Ug를바꾸지않는다. 채널10..14는무효다.

COPY는r0source,r1destination,r4source_offset,r5destination_offset,r6count1..4,r7source_selector,r3destination_selector다. 허용source=5/6/7/8,destination=5/7/8이며배아destination은자신의6/7source만허용한다. source bytes≤4cell과revision을Think에서캡처한다. 같은매체의겹침은memmove의캡처값의미다. mutation plan이pin한사적매체와읽기전용실행매체에는쓰기Busy다. 공개재활용에는독점쓰기잠금이없다. 현재logical_len밖읽기금지,capacity내쓰기만허용한다. destinationrevision만성공쓰기transaction당1증가한다. 읽기만한source revision은그대로다. 값이같더라도실제쓰기·비용·revision은발생한다.

REPRO0 Allocate는추가 인자 없음,1 Resize는r5목표페이지,2 Finish는r5raw길이,3 Step은최대4cell,4 Reopen,5 Release는r5상대방향0..3,6 Cancel이다. Release는19절예약이며빈인접tile·최종부모상태·열·Ug·남은actionbudget을모두확인한다. Ready 이후에도energy투자는가능하다. 배아 raw직접WRITE/COPY/Resize는Writing에서만허용한다.

### 비용·실패 순서

VM4q→외부query8q→시도비예약→주소/capability/접촉/generation/형식검사→가용량/공간/점유 재검사→성공량비용 순이다. 외부query domain은1/2/3/4/5/9/10/11이다. 내부query는4회상한에포함하지않고8q추가비도없다. invalid외부주소의시도도query를소모한다. query5회째는4q만내고QueryLimit,world effect제출없음이다. COPY source가외부면이검사를먼저한다.

유효한효과에시도비가없으면효과를제출하지않고Energy receipt를남긴다. 이미 제출한효과의구조적실패에도시도비는 환불하지않는다. phase3의남은energy를actionbudget상한으로봉인하고,다른개체에게서이후받은energy는보유량만증가한다. 실제지불은min(봉인잔량,현재energy)에서한다. 가용에너지상태는한번만차감한다.

TRANSFER/COPY/LINK/UNLINK/WORK만명시된양의prefix부분 성공을허용한다. EMIT·이동·Allocate·Resize증설/복구·Release는원자적이다. Resize축소의cell처리는55절의부분진행을허용한다. 양0또는동일저장소는Noop,허용가능최대량0이면Energy→Matter→Capacity→Blocked순의첫실패다. 양수일부성공은Partial다. r15는이동1,cell복사수,물질량,강도변화량,WORK의실제q를65535범위로반환한다. Resize는완료한페이지변경수이며재활용중Partial에서는0이다. 상태전이는1이다.

Finish/Commit의형식 검증비는검사rawcell당2q이며시도비와별도다. 최초 검사 전전체검증비를원자적으로 확인하고부족하면 Energy다. 형식이깨져도검사비는낸다. 재시도에서 (MediaUID,revision,used_cells,genome_format)과 charged 형식 결과가 같으면 재검사비를 면제하되 시도비는 낸다. used_cells가 바뀌면 새 검증이다. binding·retained 상한과 소유·pin 조건은 매번 다시 검사한다. cache는결과·물리비용과무관한host속도최적화가아니므로검증revision/charged상태를 저장한다. 수정후에는재검사·재지불한다. 실제bytes복사·페이지재활용32q/cell은별도로낸다. prefix뒤보존data에무료정리나자동환급을적용하지않는다.

**경계:** W는접근·비용·원자성, P는숫자로고정한ABI, E는여러tick의프로그램, U는receipt와실제효과의표시다.

## 53 저장·설정·thread 경계의 구현 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

container는 CIVSIM5/major5/minor0다. 정수 LE, bool u8의 0/1,optional은 u8존재 tag와 payload다. 고정배열은길이없이, 가변목록은 u32 count뒤 element를쓴다. section최상위 record_count는 u64다. 문자열 UTF-8,bytes는 u32length뒤내용이다. 미지필수 enum·reserved bit·중복 key·범위밖 offset은 CorruptSave이고미지원 schema는 UnsupportedSchema다. header flags0,directory flags bit0 mandatory만허용한다.

store는 UID순, World는 TileIndex순이다. typed stable UID를 wire참조로사용하고 slot/generation은 Runtime allocator table에별도저장한다. ProgramBlob은 hash/bytes순이며 decodecache는재구축한다. 같은 bytes인물리매체도 MediaUID별로보존한다. 상세 Media/Binding/Embryo/Manifest는 59절의필드표가규범이다.

| section·schema1 | record와정렬 | 검증 |
| --- | --- | --- |
| Rules1 |rule_text, manifest bytes, ancestor rawcells |지원 physics_id·hash·표일치 |
| World2 |width, height, 44byte SoA, Residue목록 |tile수·열용량재구축 |
| Agents3 |AgentHot, VmState, WorkingMemory, BindingTable |점유·prefix·참조 |
| Programs4 |hash, rawprefixcells |bytes와 hash, descriptor재구축 |
| Artifacts5 |Parcel목록, Media목록 |bond·pore·owner·Present pageG |
| Embryos6 |Embryo, MutationPlan, Validation |상태·pin·cursor |
| Fields7 |current signal, ReactionCredit, LocalTrace |tile/site/reaction순 |
| Runtime8 |next IDs, slot/generation/free lists, 입력 queue, RNGcounter |미래 UID·참조·순서 |
| Ledgers9 |43절 Ledger |원소·현재 E전수재합산 |
| Observer10 |A1.4 config, archive, evidence, index |optional, world복구와독립 |
| Inputs11 |tick, sequence, commandpayload또는 logref |순서·분기·외부 ledger |

MutationPlan source pin은 MediaUID/revision/start_cell/count목록이다. interval edit는 kind Copy0/Literal1,source index:u16,start:u16,count:u16,literal cells순이다. Copy는 literal비움, Literal은 source0xffff다. point edit는 destination cell:u16,bit:u8주소순이다. operator0..7은 20절순, NoStructural255다. result≤9216cell, edit목록≤18432,sourcepins≤128이다. 계획의 cursor는저장하되 R3의 suffix cleanup cursor는없다.

검증비 cache는 59절 Validation이며동일 MediaUID/revision/used_cells/genome_format에 대한 charged 형식 결과만 재사용한다. Reopen은 SealedWork에서불가, Invalid/Ready에서 mutation_applied를보존한채 Writing으로간다. 완료된변이를재추첨하지않는다. Cancel·사망에서 pin을해제하고 55절대로매체를남긴다.

ReactionCredit key는 Loose0/Matrix1/Pore2/Media3/PoreBoundary4,owner UID 또는 TileIndex,reaction2..6이다. kind4는 ParcelUID와 Rx2/5/6만 허용한다. 실행 순서는 번호순이 아닌63절의 site 순서다.0credit은생략가능하고없는 key는 0이다. LocalTrace는 kind:u8,world_direction:u8,actual:u16,event_tick:u64,sequence:u32의 16byte payload다. age는 READ시계산하며과거 tick을영구적으로 8bit에잘라저장하지않는다. Media exposure_clock은 Media에한번만저장한다.

RunManifest는 59절의닫힌 schema다. PhysicsSpec의기본값을별도 JSON override로중복받지않는다. worldgen_version3,ancestor Founder-4,format5를각각구별한다. camera·worker수·wall time은운용 metadata이며 CoreHash에들어가지않는다. 수치상한검사는 stage3에서가능한합산까지 checked중간계산으로확인하며 wrap/clamp로물질을잃지않는다.

coordinator만 Simulation을소유한다. worker는 phase ID와 disjoint view/patch를받고예약범위안에서만쓴다. 필수 event queue가가득차면대기하고 render는최대 3개완결 snapshot중최신값을쓴다. worker panic이나불변식실패의부분 tick은정상 save가아니다. 마지막완결 checkpoint+입력으로재현한다. Pause는현재 tickcommit후 ack, Step은정확히 1tick, MAX는 pacing해제다.

**경계:** Engineering Contract는 schema·소유권·오류다. W/P는저장·이어하기전후동일해야한다. E의역사를재현하며 U는복구·누락·호환상태를알린다.

## 54 화면 데이터·좌표·상호작용의 구현 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

세계 좌표는 x가 오른쪽,y가 아래쪽이며 북=(0,-1)이다. Bevy 표시 좌표는 tile 중심(x+0.5,height-y-0.5)로 변환한다. 확대·보간·화면 흔들림은 렌더 전용 float 계산이고 core 좌표를 수정하지 않는다. 선택은 표시 snapshot의 UID lookup을 사용한다.

| message | 필드 | 수명·일관성 |
| --- | --- | --- |
| ViewRequest |sequence:u64,epoch:u64,viewport tile bounds,LOD:u8,overlay:u8,selected UID optional |최신 요청 하나만 유지 |
| RenderSnapshot |epoch,tick,request_sequence,world summary,chunk pages,instances,selection reply |다른 epoch·이전 sequence는 폐기 |
| ChunkPage |chunk_id:u32,revision:u64,source_tick:u64,LOD,payload |단독 복원 가능,불변 Arc |
| TileView |terrain:u8,surface_c:u8,height:i16,stock:[u16;10],θ:u16,P:u16,actor/parcel present flags |관측 포화와 원자료 차이를 범례에 표시 |
| AgentView |uid,tile,orientation,body_g,shell_s,inventory,energy,grip UID,observed lineage color |내부 VM은 선택 상세 응답으로만 |
| ParcelView |uid,tile,matrix[10],pore[10],c,orientation,bonds[4],anchor,media present,θ |실제 상태로만 모양·색 계산 |
| SelectionReply |request_sequence,source_tick,uid,alive,data,event cursor |사망·화면 밖·이전 epoch를 구별 |

renderer가 사용하는 GPU instance는 entity와 별도 연속 buffer다. AgentGpu의 32byte layout은position:[f32;2],facing:u32,color_rgba:u32,energy_fraction:f32,load_fraction:f32,flags:u32,reserved:u32=0이다. 같은 순서의 CPU UID 배열을 따로 둔다. flags는선택bit0,grip bit1,외피유무bit2,body_g는bits16..20이며 그 외는 0이다. body_g는scale을 별도 만들기보다 고정 몸체 glyph의 채움으로 표현한다. 외형 변화가 새 기관을 뜻하지 않도록 범례에 명시한다.

ParcelGpu의 32byte layout은position:[f32;2],color_rgba:u32,flags:u32,volume_fraction:f32,compression_fraction:f32,bonds_packed:u32,orientation_anchor:u32다. bonds_packed는북·동·남·서각u8,orientation_anchor의bits0..1방위,bits8..15anchor,그외0이다. GPU 정렬·attribute 형식은 shader와 같은 명시적 encode를 쓰고 compiler padding을 전송하지 않는다. 변화 없는 instance page는 재사용한다.

chunk 기본 지도는RGBA8 색상 texture와R16Uint 선택 자원 texture를 사용한다. 온도·반응·영토 등의 추가 texture는 선택된 overlay만 만든다. overlay ID는None0/Resource1/Energy2/Signal3/Lineage4/Cluster5/Flow6/Temperature7/Chemistry8이다. 범례·단위·source_tick을 함께 표시한다. None은 세계를 읽기 쉽게 보여주는 기본 색이며 실제 법칙의 지형 enum을 추가하지 않는다.

chunk texture는64×64,원거리 mip는집계단위4/16/64다. 부피·에너지·개체 수는합,온도는열용량 가중 평균,지형색은대표값과혼합비를쓴다. 일부tile만본근사평균을전체값으로표시하지않는다. 미리 집계한summary의tick이render tick과다르면낡은범위를표시한다. 상세instance100k 제한을넘는화면은집계LOD로전환한다.

커서는지도좌표→TileIndex를계산해해당 page의 선택 자료를 조회한다. 기본 클릭은actor우선,Shift+클릭은parcel,빈곳은tile이다. drag로선택 패널을열지않고pan으로처리한다. wheel은커서가리키는세계 좌표를유지하며zoom한다. 표시와core가어긋나죽은 UID를 선택하면 동일 UID의 역사로 전환한다. 지금 그 slot에 있는 새 개체로 선택을 바꾸지 않는다.

계통도·기술도는관찰DAG다. node를누르면실제발생tick·복사출처·사용증거·관련artifact로이동한다. 사실 edge는 실선,유사성·영향후보는점선이다. split/merge와다중부모재조합을표현할수있어야하며미리정한단계순서로강제배치하지않는다. 기술 이름은사용자주석또는근거를붙인임시이름이다.

지도에서의일시적움직임·운반선·열효과는실제Motion/MatterMoved/Reaction사실에서만만든다. MAX는이효과를끄며선택·pause입력은계속처리한다. 수백tick을건너뛴snapshot을수백개의가짜행동애니메이션으로채우지않는다. 구체적인미술asset·palette는구현시제작하되기능이없는집·공장·직업sprite로세계의성공을꾸미지않는다.

**경계:** Engineering Contract는메시지·GPU·선택. W/P 변화없음. E는실제 사건만보이며U는지도·범례·근거·역사탐색이다.


## 55 매체 수명·재활용·손상의 단일 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

R5에서는 정보가 있는 매체를 버리는 것과 그 재료를 회수하는 것을 구분한다. 취소·교체·사망 때문에 소유가 끝난 매체는 같은 UID와 bytes를 유지한 채 현재 tile에 내려놓는다. 페이지 G를 동시에 inventory에 돌려주지 않는다. 실제 재료 회수는 유료 Resize 축소 또는 화학 반응으로만 일어난다. 이것은 새로운 직업·기술 API가 아니라 기존 WRITE/Resize와 물질 반응의 공통 수명 규칙이다.

| 선택 | 장점 | 단점·성능·개방성 | 결정 |
| --- | --- | --- | --- |
| 모든 취소에서 재료 즉시 반환 | 구현과 메모리 사용이 단순 | 기록의 물리적 지속성을 깨고 비용 우회 발생 | 제외 |
| 모든 폐기에 강제 전체 삭제 작업 | 정보 삭제 비용이 일정 | 죽은 개체가 일을 해야 하는 모순, 취소 장기 잠금 | 제외 |
| 기록을 남기고 재활용을 별도 처리 | 사후 기록 발견·파괴·회수가 동일 물리 경로를 가짐 | 외부 기록과 역사 데이터 증가 | 기본 선택 |

### 주소와 페이지

Media는 page_slots 목록을 가지며 각 slot은 Absent0 또는 Present1과 128개의 u16 cell이다. slot 인덱스가 물리 주소다. capacity_cells=128×slot_count, paid_pages=Present 수, logical_len은 마지막 작성 끝 위치다. logical_len≤capacity_cells이고 hole이 있어도 뒤의 주소를 앞으로 당기지 않는다. 실제 G·열용량·유지 비용은 Present만 센다. hole에는 숨은 cell bytes를 보존하지 않는다.

ParcelPublic의 slot_count≤8, 사적·배아·Dropped 매체는≤72다. 사적 매체가 버려졌다는 이유로 8페이지에 맞춰 자르지 않는다. 사적 실행 매체는 형식 검증에 사용한 prefix 안에 hole이 없어야 한다. 외부 기록의 hole 읽기는 BadAddress/0이며 존재하지 않는 cell을 zero 데이터로 성공 처리하지 않는다. COPY는 최초 hole 직전까지만 복사하며 시작부터 hole이면 InvalidTarget, 양수가 복사되면 Partial이다.

빈 parcel 매체 생성은 WRITE Resize, domain5, selector0으로 그 parcel에 첫 페이지를 만드는 경우뿐이다. corpse ordinal로 새로운 매체를 생성할 수 없다. OwnDraft가 None이면 첫 Resize로 만들고, EmbryoTape는 Writing인 배아가 이미 있어야 한다. 최초 페이지 초기화는 G1+128q, 시도비 64q이며 내부에 기록 장치 이름이나 소유권 권한을 만들지 않는다.

### Resize의 정확한 전이

목표 페이지 수 t와 현재 slot 수 n을 비교한다. 한 intent는 최대 한 페이지의 추가·hole 복구·제거만 끝낼 수 있다. 성공한 실제 작업량과 receipt를 아래처럼 정의한다. 모든 에너지는 actor의 봉인된 예산에서 지불한다.

| 조건 | 동작 | receipt와 비용 |
| --- | --- | --- |
| t>n | 끝에 Present 한 페이지 추가, 기존 hole은 유지 | Ok/actual1, G1+128q+시도비 |
| t=n, hole 있음 | 가장 작은 hole 인덱스를 zero Present로 복구 | Ok/actual1, G1+128q+시도비 |
| t=n, hole 없음 | 진행 중 재활용 cursor를 버림, 이미 지운 bytes는 유지 | cursor가 있었으면 Ok/actual0, 아니면 Noop/actual0; 시도비만 |
| t<n, 마지막 slot이 Absent | 마지막 주소 slot 제거, len=min(len, 새 capacity) | Ok/actual1, 반환 G0, 시도비만 |
| t<n, 마지막 slot이 Present | 아래 유료 재활용을 최대 4cell 진행 | 미완료 Partial/actual0, 완료 Ok/actual1 |

유료 재활용은 마지막 페이지의 cell0부터 127까지 순서대로 0을 쓰며 실제 처리 cell당 32q를 낸다. 기존 값이 0이어도 검사·처리 비용은 같다. 한 호출은 남은 에너지 안에서 최대 4cell을 처리하고, 128cell 완료 시 페이지를 제거하며 G1을 같은 tile loose/residue에 반환한다. 한 페이지 전체 재활용은 최소 32회, 4096q 작업+2048q 시도비가 필요하다. VM 비용과 매체 유지 비용은 별도다. 무료 대량 삭제를 숨긴 host GC 경로는 없다.

RecycleState는 page_index:u16,next_cell:u16이다. 다른 개체도 같은 공개 매체의 축소를 이어 할 수 있다. 소유자 UID로 보호하지 않는다. 해당 페이지에 WRITE/COPY 또는 bit 마모가 생기면 cursor를 0으로 초기화한다. 다른 페이지 수정은 cursor를 유지한다. Resize 증설·hole 복구는 cursor를 취소한다. 같은 마지막 페이지를 향한 축소 요청은 t가 달라도 이어간다. 자원이 부족해 실제 처리 0이면 Energy이며 cursor·bytes 불변이다. 일부 처리 뒤에는 이후 비용 부족을 이유로 롤백하지 않는다.

읽기는 재활용 중에도 현재 bytes를 읽는다. mutation source로 pin된 사적 매체와 inherited/learned/retained 매체는 원래 수정 불가이며 Busy다. WRITE/COPY destination이 hole이면 기록하지 못하며 먼저 Resize로 복구해야 한다. 공개 재활용을 잠금으로 이용해 타인의 읽기·쓰기를 영구 봉쇄하지 않는다. 공개 매체에 실행 frame이나 mutation plan을 직접 pin하는 경로는 없다.

### Commit·Finish·폐기

실행 used_cells와 매체 logical_len을 분리한다. Commit/Finish는 r5 길이의 prefix만 실행 프로그램으로 검증한다. prefix 뒤의 데이터는 같은 유료 매체에 남고 OwnCode READ로 읽을 수 있으며 실행되지는 않는다. 채택하면서 뒤쪽 데이터를 무료 삭제하거나 재료로 환급하지 않는다. 실행 prefix가 줄어도 페이지 유지 비용은 줄지 않는다. 실제 축소는 mutable 상태일 때 Resize로 수행해야 한다.

이 결정으로 R3의 Commit suffix cleanup과 Finish 삭제 계획은 제거한다. Commit은 전체 prefix 형식 검사비 2q/cell을 원자적으로 낸 뒤 PendingAdopt가 되고 stage11에서 채택된다. PendingAdopt는 같은 tick에만 존재한다. CancelDraft는 자신이 가진 draft를 통째로 내려놓고 None으로 바꾸며, 이미 낸 비용을 반환하지 않는다. 새 매체에 다시 쓰려면 다시 물질과 비용이 필요하다.

변이 결과가 다른 새 매체로 완성되면 옛 EmbryoRaw는 부모 tile의 Dropped 매체가 된다. 취소하면 raw/result 매체를 모두 그대로 내려놓는다. 몸체 G16/S2와 아직 활성화하지 않은 work-memory G1은 반환 재고로 회수하고, 투자 energy는 열로 전환한다. 고정 작업 기억은 전원이 꺼지면 상태가 사라지는 휘발성 기억으로 정의한다. 이 비용 체계는 실제 Landauer 한계를 계산한 정보 열역학이라고 주장하지 않는다.

교체된 learned 매체는 frame·pc·mutation source가 남으면 AgentRetained로 유지한다. 마지막 참조가 사라진 stage11에 현재 tile로 내려놓는다. inherited 매체는 agent 생존 동안 유지한다. owner 변경은 bytes·logical_len·used prefix를 바꾸지 않지만 revision을 1 증가시켜 옛 접촉 캡처를 무효화한다. 외부로 바뀌는 순간 exposure_clock=0, reaction credit=0이다.

### 자연 손상과 삭제

외부 매체의 Rx2 산화는 가장 큰 Present page index부터 한 페이지씩 소비한다. 소비한 G1에 대한 반응물 O1, 생성물 C1/W1과 열 5000q를 같은 transaction에서 처리한다. 페이지는 hole이 되고 뒤의 cell 주소는 유지한다. 산화 또는 마지막 페이지 재활용으로 모든 Present가 없어지면 Media를 제거하고 참조를 끊는다. 이 경우 G를 loose에도 반환하지 않는다. 페이지가 없어지면 해당 재활용 cursor를 취소한다.

bit 마모의 페이지 선택은 Present index 목록에서 균일 추첨한다. cell0..127,bit0..15를 차례로 추첨하며 아직 logical_len 밖이어도 물리 cell 자체는 손상될 수 있다. 마모는 logical_len을 늘리지 않는다. 한 due의 외부 화학이 끝난 뒤 살아남은 외부 매체에만 bit 마모를 적용한다. 새로 버려진 매체도 이후 due부터 같은 규칙을 받는다.

revision은 u64이며 bytes·페이지 존재·logical_len·owner가 실제 바뀐 transaction마다 한 번 증가한다. 읽기와 COPY source 접근만으로는 증가하지 않는다. 위치 변화는 별도 캡처 좌표로 Stale를 검사하며 내용 revision을 올리지 않는다. 같은 매체 내부 복사는 destination 변경으로 한 번만 증가한다. 자연 반응의 여러 페이지 손실은 site transaction 하나의 revision 증가와 페이지별 damage 사실을 남긴다. overflow 직전은 IdExhausted다.

**경계:** W는 물리 매체·작업·손상·주소. P는 기존 READ/WRITE/Resize/COPY. E는 기록 회수·보존·재사용·파괴와 지식 소실. U는 bytes 출처·손상 구간과 유효성 표시다.

## 56 동일 tick 상호작용·비용·실패의 판정표

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

구현자는 현재 상태, Think의 캡처, 미래 예약을 구분한다. immutable Sense는 같은 tick 모든 agent에게 같은 환경을 제공하지만 Self.energy에는 자기 VM 실행 중 지출을 반영한다. action budget은 stage3에서 봉인하며 다른 agent가 나중에 준 에너지가 그 상한을 늘리지 않는다.

| 동시 상황 | 규범적 결과 | 보존·정보 경계 |
| --- | --- | --- |
| A/B가 같은 공개 기록을 읽어서 서로 다른 draft로 COPY | source 수정이 없으면 둘 다 성공 가능 | 읽기 때문에 source revision 증가 0 |
| A가 기록 수정, B가 옛 bytes COPY | 정렬 순서에서 수정이 먼저면 B Stale; COPY가 먼저면 옛 bytes 복사 | Think 캡처를 새 bytes로 바꾸지 않음 |
| 두 agent가 같은 빈 matrix에 입고 | 먼저 생성한 parcel과 최신 용량을 재검사, 두 입고 합산 가능 | 부재를 캡처한 주소는 TileSite 의미 |
| 기존 parcel을 캡처한 뒤 파괴·재생성 | 새 parcel이 같은 tile이어도 Stale | stable object identity 검사 |
| 같은 draft 두 요청 | agent당 effect1이라 불가능 | 외부 agent는 사적 draft 쓰기 불가 |
| 서로 다른 agent가 같은 공개 페이지를 재활용 | serial 순서대로 cursor를 이어 진행 | 각자 실제 cell 비용 지불 |
| source 재고가 먼저 줄어듦 | TRANSFER 최대 가능량으로 축소 | 옛 재고량을 예약해 복제하지 않음 |
| source G가 후속 화학으로 소비됨 | 이미 stage4에서 이동한 양은 목적지에 있음 | stage8은 최신 재고만 사용 |
| 돈처럼 energy를 돌려 보내 같은 tick 더 많은 일 수행 | 수신량은 남지만 봉인 budget 증가 0 | 다음 tick부터 사용 가능 |
| 출생 예약 뒤 목적지에 이동 agent 진입 | stage10 출생 Blocked | 배아는 Ready, 투자 유지 |
| 출생 예약 뒤 목적지 actor가 나감 | 최종 비어 있어도 최초 예약 실패자는 부활하지 않음 | stage4 성공 예약만 재검사 |
| 출생 예약 뒤 부모가 밀림·사망 | 인접성 실패 Stale, 사망이면 취소 | 성공 receipt는 stage10에서만 확정 |
| 검증 후 pending draft의 부모 사망 | adoption 취소, 매체는 사체로 남음 | dead agent binding 생성 0 |
| 여러 필드가 동시에 상한 초과 | 정의된 실패 우선순위 | 일부 필드만 commit 금지 |

주소 캡처 종류는 TileSite와 ObjectKey다. Loose/Surface는 항상 TileSite다. 빈 Matrix/Pore 생성 요청 중 Matrix 입고만 TileSite 생성 자격을 갖는다. 빈 Pore, ActorSurface, 존재하는 Parcel/Media는 ObjectKey이며 해당 객체가 바뀌면 Stale다. 같은 tick 빈 slot에 생성한 매체를 다른 agent가 Think에서 미리 읽는 경로는 없다. 경계 밖 주소는 생성 후보가 아니다.

일반 효과는 VM4q, 외부 source query가 있으면 8q, 기본 시도비 순으로 요구한다. 실패 사유 판정은 주소→capability→접촉/차폐→key/revision→상태/형식→energy→matter→capacity→occupancy다. 구조 검사 통과 뒤 partial의 최대량은 원천·수용량·봉인 energy·현재 energy를 모두 만족하는 최대 정수다. 지불에는 시도비를 다시 포함하지 않는다.

amount0은 정상 주소·접근 검사를 거친 뒤 Noop다. 잘못된 주소를 amount0으로 우회해 성공시키지 않는다. 부분 가능한 최대량이 0이면 최소 양 1에 대한 energy, matter, capacity, occupancy 검사를 차례로 수행하고 첫 실패를 보고한다. 다른 제약이 동시에 실패해도 우선순위를 바꾸지 않는다. 실제 계산은 정해진 단위량 feasibility 판정으로 구현하며 outcome을 host 순회 순서로 정하지 않는다.

물질·가용 에너지·heat의 delta는 서로 다른 필드다. TRANSFER energy15의 q는 수신자 energy에, overhead ceil(q/64)와 attempt는 송신 tile U에 들어간다. Heat 작업의 실제 q는 목적 tile U에만 넣고 송신 U에 다시 더하지 않는다. 일반 가공·마찰의 일은 접촉/목적지에 10·18절대로 분배한다. 수신 energy가 cap을 넘는 분량은 전송하지 않으며 초과량을 사라지게 하지 않는다.

몸체·재고·코드·배아·운반 parcel의 질량은 이동 비용에 각각 한 번 들어간다. Release의 출생물은 몸체, work memory, raw 매체, 반환 inventory이며 embryo.energy는 질량 0이다. 같은 tile 안에서 사망하거나 매체를 내려놓으면 열량 이동 0, C 합 변화 0이다. 출생으로 다른 tile에 분리하면 그 물질의 열용량과 Ug를 정확히 옮긴다.

반응 credit은 실제 에너지가 아니며 site가 남으면 같은 주소의 축적 상태로 유지한다. parcel/매체가 사라지면 그 site credit도 제거한다. 새 UID에 옛 credit을 붙이지 않는다. 변환된 고체 때문에 부피·bond 최대가 달라지면 같은 transaction에서 검증·조정을 마친다. 화학 반응으로 actor와 parcel의 점유가 모순되는 경우 해당 extent를 거절하며 몰래 actor를 밀어내지 않는다.

**경계:** W는 동시성·접근·비용 정산. P 추가 없음. E는 충돌·경쟁·교환과 실패. U는 원인별 receipt와 실제 결과를 분리한다.

## 57 공개 조상·기능 fixture·에너지 예산

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

초기 프로그램도 세계 밖에서 작성한 가정이다. Founder-4는 생존·국소 확보·자기 복사만 가진 공개 시작 조건이며 일반 agent와 같은 VM·비용·감각을 사용한다. 아래 표는 구현할 bytecode의 행동 명세다. host가 상태 번호를 읽어 자원을 대신 모으거나 복제를 끝내서는 안 된다. 상태·threshold·방향 순서도 genome의 일반 상수와 분기이며 후손 변이로 바뀔 수 있다.

### 시작 조건과 영속 변수

entry tag=0, point_threshold=64, structural_threshold=2048, 구조 가중치=[4,2,2,2,1,1,1,1]이다. 레지스터·작업 기억은 0, orientation=0,port_mask=0,embryo=None로 시작한다. 모든 조상은 동일한 프로그램을 갖고 AgentUID만 다르다. 조상에게 지어진 구조·외부 기록·통신 규약·재료 가공 절차를 주지 않는다.

작업 기억의 용도는 [0]복제 단계, [1]복사 cursor, [2]조상 raw prefix 길이, [3]요구 페이지 수, [4]국소 탐색 인덱스, [5]Release 방향, [6]수리 횟수, [7]대기 잔여 tick, [8]직전 효과 종류, [9]마지막으로 읽은 빛 값,[10]탐색 대상종(G/S),[11]해당 다섯 위치 탐색 완료 flag다. 나머지는 0으로 시작하며 프로그램이 사용할 수 있다. 매 반복마다 raw prefix 길이를 Self.inherited_len에서 읽어 [2]를 갱신하고 [3]=ceil([2]/128)로 계산한다. 이 값은 host가 후손에 맞춰 고쳐 주지 않는다.

국소 확인 순서는 자기, 앞, 오른쪽, 뒤, 왼쪽이다. 외부 query4회 상한 때문에 다섯 위치를 한 tick에 전부 읽으려고 하지 않는다. 빛 READ도 같은 한도에 포함한다. Ok이면 재고를 사용하고,Boundary/Occluded이면 해당 위치를 사용 불가로 처리하여 탐색 인덱스를 늘린다. QueryLimit이면66절대로 현재 index를 보존하여 다음 tick에 이어간다. 고정 주소의 BadAddress는 조상 구현 오류이며 변이된 프로그램에서는 일반 fault다. [4]를 저장하여 다음 tick에 이어간다. 성공 여부는 다음 tick r14/r15로만 확인한다. COPY cursor는 요청량이 아닌 실제 r15만큼 늘린다. 실패한 효과를 성공한 것으로 가정해 다음 단계로 넘어가지 않는다.

### 우선순위와 상태 전이

아래 행을 위에서부터 적용한다. 대기·효과 실행 뒤에는 해당 tick을 끝낸다. 행의 조건을 VM 분기와 LOAD/STORE로 구현하며 조상 전용 opcode를 추가하지 않는다. used prefix 길이가 9216을 넘거나 페이지 수가 72를 넘으면 복제를 중지하고 일반 생존 반복으로 돌아간다.

| 우선·조건 | 프로그램이 하는 일 | 성공 후 상태·실패 처리 |
| --- | --- | --- |
| 0. 미소비 효과 receipt 있음 | COPY는 cursor+=actual; 종류별 결과 처리 뒤 소비 표시 | Partial도 실제량만 인정,대기보다 먼저1회 |
| 1. 대기 잔여>0 | 잔여-1, YIELD | 다른 효과 없음 |
| 2. 32tick 간격의 빛 점검에서 current_light<8192 | 대기 잔여 31로 설정 후 YIELD | 밤에도 대사·유지·물리는 계속 |
| 3. energy<4096 | 대기 잔여 7로 설정 후 YIELD | 미래 에너지 무료 예측 없음 |
| 4. inventory G<24 | 국소 G 재고가 양수인 첫 위치에서 최대 4 TRANSFER | 한 바퀴 실패하면 아래 이동 행 |
| 5. 배아 None이고 S<2 | 같은 순서로 S 최대 2 TRANSFER | G와 같은 탐색·실패 방식 |
| 6. 배아 None, G≥24,S≥2,E≥49152,빛≥8192 | REPRO Allocate | 성공하면 cursor0,수리 0 |
| 7. Writing, capacity<128×요구 페이지 | REPRO Resize, 목표 요구 페이지 | G 부족이면 확보 행, 에너지 부족이면 대기 |
| 8. Writing, cursor<raw 길이 | OwnCode bank0→Embryo COPY, 양=min(4,남은길이) | 다음 receipt에서 cursor 갱신 |
| 9. Writing, cursor=raw 길이 | Finish(raw 길이) | 상태를 다시 읽고 분기 |
| 10. SealedWork | REPRO Step | Matter면 G 확보, Energy면 8tick 대기 |
| 11. Invalid, 수리 0 | Reopen | 성공하면 cursor0,수리 1,자기 raw를 다시 기록 |
| 12. Invalid, 수리 1 | Cancel | 매체는 남고 비용 환불 없음 |
| 13. Ready, 배아 energy<16384,E≥32768 | energy15를 min(1024,부족량) 이전 | 실제량만 투자 |
| 14. Ready, 배아 energy≥16384 | 앞/오른쪽/뒤/왼쪽 순서로 Release 시도 | 성공하면 cursor·방향·수리 0,실패면방향+1 mod4 |
| 15. 확보 다섯 위치 실패, E≥8192,빛≥8192 | RANDOM 하위 2bit 방향으로 DRIVE Translate | 성공 여부와 무관하게 탐색 인덱스 0 |
| 16. 그 외 | 8tick 대기 | 비용 없는 host sleep 없음 |

탐색 대상종이 바뀌면 index와 완료 flag를0으로 한다. 다섯 위치가 모두 Ok/재고0 또는 Boundary/Occluded로 사용 불가임을 확인했을 때 완료 flag를1로 하고 확보 행을 건너뛰어 이동 행으로 간다. 이동 시도 또는 실제 확보 뒤 두 값을0으로 초기화한다. flag가 설정된 상태에서 밤·에너지 때문에 이동할 수 없으면 대기 후 새 탐색을 시작한다. 일반 receipt는 새 효과를 제출하기 전에 소비 표시를 바꾼다.

Writing에서 G 확보를 반복해도 부족하면 프로그램은 대기하거나 이동할 뿐 배아 페이지를 공짜로 늘리지 않는다. raw가 짧아진 후손은 앞 prefix만 쓰고 Finish한다. 기존 긴 suffix는 55절대로 유료 매체 안에 남는다. 변이 결과를 수리해도 이미 완료한 변이를 재추첨하지 않는다. Ready에서 행동이 막힌다고 배아를 자동 순간이동시키지 않는다.

빛 점검은 Self.age_lo AND31=0인 반복 또는 방금 대기를 마친 반복에서 Surface.current_light를 실제 READ한다. 저장한 직전 빛 값은 내부 기억이며 현재 세계에 대한 확정 정보가 아니다. Founder에 환경 tick이나 전역 낮·밤 phase를 직접 넣지 않는다. 각 상태 사이의 보통 연산이 32 instruction을 넘으면 VM pc에서 정상 중단되고 다음 tick에 이어진다.

### bytecode 산출과 적합성

구현 첫 단계에서 이 표를 일반 ISA의 분기·메모리·효과 명령으로 조립한 founder.vmg를 만든다. 원본 assembly, disassembly, raw cell 목록, header/descriptor, BLAKE3 hash를 함께 보존한다. 조립기는 branch 주소 계산과 descriptor 길이만 계산하며 유용한 루프·오류 복구·신호 정책을 자동 발명하지 않는다. optimizer는 끄고 실제 산출 명령 수와 실행 trace로 비용을 계산한다.

조상 적합성 fixture는 각 행의 진입 상태에서 한 번의 효과와 그 다음 receipt 처리를 대조한다. 특히 query 상한, COPY partial, Release 실패, Invalid 두 번, 밤 대기, energy 부족을 검사한다. 표와 다른 추가 확보·복제·구조 행동이 들어가면 다른 조상 revision이다. 정확한 바이너리와 hash는 아직 생성되지 않았으므로 이 항목은 문서 설계 완료와 별도의 첫 구현 산출물이다.

### 먼저 계산할 생존 예산

유지 비용 M=32+자기 work page1+자기가 보유한 모든 Present 매체 페이지+배아 work page와 Present 페이지다. 예를 들어 inherited6페이지, 배아 없음이면 M=39q/tick이다. 대기 루프가 평균 16q/tick을 사용한다는 조건에서 총 55q/tick이다. 2560tick 동안 광반응·섭식 없이 버티려면 140800q가 필요하다. E65535와 G46의 호흡 에너지는 142171q라 이 단순 에너지 부등식은 만족하지만, 산소·온도·배출 공간과 실제 VM 비용은 별도 조건이다.

이 계산은 생존 실험 결과가 아니다. 실제 조상이 밤에 페이지를 더 유지하거나 평균 명령 비용이 크면 여유가 사라진다. 조립된 조상의 실측 대기 비용을 식에 대입해 단위·일주기 예산을 먼저 확인한다. 낮의 광반응도 1G당광 credit10000이 실제 들어와야 하므로 평균 가용 에너지를 무한하다고 취급하지 않는다.

길이 Lcell 프로그램의 무변이 복제 열 소산 하한은 ceil(L/4)×64+32L+(128+64)ceil(L/128)+2L+Allocate256+Finish256+Release256+에너지 이전 시도비와 overhead다. 몸체 G17/S2,코드 Gceil(L/128), 최소 자식 에너지 16384와 모든 VM·유지·이동 비용은 따로 필요하다. L=256이면 복사 시도 4096,복사 8192,페이지 초기화 256,Resize 시도 128,검증 512,세 상태 시도 768의 합은 13952q다. 이전을 1024q씩 16번 하면 이전 시도 1024와 overhead256이 추가되어 열 소산 하한 15232q다. 자식 energy16384까지 포함한 총 지출은 31616q다. 이 항목을 빼고 복제가 싸다고 평가하지 않는다.

### 화학 가공의 단계 순서 검증

R3의 'θ≥900이면 가공 가능' 설명에는 stage8 외기 냉각이 먼저라는 조건이 빠져 있었다. 외부에 그대로 노출된 작은 혼합물은 sky 교환으로 280까지 식을 수 있다. witness는 가열 직후 온도뿐 아니라 실제 Rx4 검사 시점의 온도·산소·credit·생성물 용량을 만족해야 한다.

계산 검증용 한 tile은 matrix S61/X2/G1,c255,loose 전 종 0,actor 없음, 고도 0,네 이웃 P256으로 둔다. s64,P1,X1,C1215다. 밤 due에 U=1215×910=1105650이면 solar0,sky k8,방출 5040 뒤 U1100610,θ905다. Rx4 credit65520에서 증가 16으로 65536이 되어 extent1을 실행한다. 이후 matrix S61/M2,loose C1/V1,U1099570,원소·총 에너지가 보존된다. 이 fixture의 미리 채운 credit은 한 단계 정산 시험용이며 원료에서 시작하는 제조 witness로 세지 않는다.

완전 제조 witness는 credit0에서 시작하고 실제 agent의 G 확보·에너지·Heat·Compress·Abrade·TRANSFER만 사용한다. 초기 M0,명시한 원료·열만으로 credit 누적과 가공 후 회수까지 보여야 한다. P1일 때 Rx4 증가는 due당 16이라 4096회의 유효 due가 필요하다. 이 시간과 열 유지 비용이 감당되지 않으면 witness 실패로 보고하며, 성공을 위해 fixture 전용 반응 속도를 높이지 않는다.

**경계:** W/P는 일반 규칙. Founder는 공개 시작 프로그램, fixture는 기능 확인용 외부 설정이다. 이를 E의 발명으로 세지 않는다. U는 분석적 가능성·실행 성공·자연 발견을 따로 표시한다.

## 58 관찰 알고리즘의 입력·인코딩·추정 한계

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

이 절은 분석기의 재현성을 고정한다. 분석 결과가 달라도 Core의 다음 tick은 같아야 한다. 기본 분석 버전은 A1.4이며 float clustering, LLM 이름 생성, 외부 novelty 보상은 기본 경로에 없다. 분석 순서는 사실의 tick/stage/sequence로 정하며 wall time 도착 순서로 결과를 바꾸지 않는다.

### 행동 token과 비교

행동 창은 [256k, 256(k+1))이다. 각 resolved effect를 8byte token으로 쓴다. 순서는 IntentKind:u8,Outcome:u8,direction:u8,channel:u8,magnitude:u8,delay:u8,flags:u8,reserved0:u8이다. direction은 행위자 기준 0..3,방향 없는 경우 255다. channel은 물질 0..9/energy15/신호 0..3이고 kind와 함께 해석하며 해당 없으면 255다. magnitude=min(16,floor(log2(1+actual)))다. 성공량 0도 token을 남긴다.

delay는 같은 agent의 직전 effect tick과의 차이에 따라 0,1,2..3,4..7,8..15,16이상의 bin0..5다. 창 첫 token은 flags bit0=1,delay=0이고 이전 창 token과 3-gram을 이어 붙이지 않는다. 나머지 flags는 0이다. 효과가 3개 미만인 창은 sketch가 비며 ShortTrace로 표시한다. 빈 sketch를 완전히 새로운 기술로 승격하지 않는다.

3개의 token 24byte 앞에 ASCII 'behavior-a1.4'와 0byte를 붙여 BLAKE3 hash하고 처음 8byte를 LE u64로 읽는다. 가장 작은 서로 다른 128개를 정렬 보관한다. 동률 hash는 sketch 수준에서 충돌 가능성이 있는 근사라는 표시를 유지하고 중요한 후보는 원본 token으로 재검사한다.

두 sketch의 단순 교집합/합집합 비율을 원본 행동 집합의 Jaccard로 표시하지 않는다. 두 목록을 합쳐 가장 작은 min(128,합집합수) hash 집합 U를 만들고, U 안에서 양쪽에 있는 hash 수 I를 세어 Jhat=I/|U|로 비교한다. 두 원본 집합이 모두128개 미만이면 완전 집합 비교이며 그렇지 않으면 bottom-k 추정이다. 후보 판정 Jhat<1/4는4I<|U|의 정수 비교를 사용한다. 합집합 bottom-k에서 교집합 비율을 세는 방법은 [Thorup, Bottom-k and Priority Sampling](https://arxiv.org/abs/1303.5479)의 표본 비교식에 따른다. 이 근거가 본 프로젝트의 token 의미나 탐지 threshold를 검증한다는 뜻은 아니다.

후보 검색은 가장 작은 8개 hash의 역색인이다. 매칭 후보를 공유 hash 수 내림차순, window ID 오름차순으로 최대 1024개 가져온다. 별도로 같은 context의 고정 hash 표본 256개와 전체 active archive의 고정 hash 표본 256개를 합쳐 비교한다. 중복은 제거한다. 이 bounded 검색이 전역 최근접 이웃을 보장한다고 쓰지 않는다. 후보를 놓친 정도는 오프라인 전수 비교 표본에서 측정한다.

context는 window 마지막 agent tile의 G loose+residue, 순간 light, 반경 2 actor 수, 직전 256tick 성공 이동 수를 사용한다. 재고 bin은 0/1..3/4..15/16..63/64이상, 빛은 0/1..8191/8192..32767/32768이상, 혼잡은 0..2/3..6/7이상, 이동은 0/1..7/8..31/32이상이다. context는 분석용 입력이며 agent에게 새 감각을 주지 않는다. agent가 창 끝 전에 죽으면 마지막 성공 관측 tick과 그 context를 쓰고 flag를 붙인다.

새 대표와 Jhat≥3/4인 관측만 같은 BehaviorFamily 후보에 넣는다. 3개 서로 다른 창, 2개 AgentUID 이상이면 RepeatedBehavior다. 같은 개체의 직전 3창을 독립 3개체로 세지 않는다. novelty 기준<1/4와 family 동일성≥3/4 사이의 중간 영역은 기존 유사 사례로 남기고 억지로 둘 중 하나로 분류하지 않는다. 가족·집단의 계통 UID는 이 비교 입력이 아니다.

### artifact의 동일성과 반복

패턴 footprint는 중심 기준 Manhattan≤2인 13tile이다. dy 오름차순, 같은 dy에서 dx 오름차순으로 직렬화한다. tile당 경계밖 flag, parcel 존재, 10종 matrix quantity bin, 10종 pore bin, c>>4,orientation, 네 bond strength bin, anchor bin, 공개 매체 유무를 순서대로 u8에 넣는다. quantity/strength bin은 min(16,floor(log2(1+n)))이며 없는 parcel 필드는 0이다. 절대 좌표·UID·제작자·관찰 기술 ID는 내용 hash에 넣지 않는다.

회전된 구조는 별도 패턴이다. 역할이 없는 회전 대칭 family 통합은 UI의 추가 유사성 분석일 뿐 기본 ancestry를 바꾸지 않는다. bucket hash 일치 뒤 실제 field 자료를 비교하여 정밀 일치인지 coarse similarity인지 구별한다.

동일 occurrence는 (pattern hash, footprint 안 ParcelUID 집합)이다. 이동·카메라 재방문·snapshot 반복은 새 제작이 아니다. 한 family의 반복 수를 셀 때 물질 UID 집합이 겹치는 두 occurrence는 독립 사례로 동시에 세지 않는다. candidate는 3개 독립 occurrence와 2개 이상 실제 기여 actor가 필요하다. 기여 actor는 해당 구성의 성공한 matrix 입고·압축·결합 사실에서 가져오고 가장 가까이 있던 agent를 제작자로 지정하지 않는다. 자연 반응만으로 형성됐으면 NaturalOrigin flag다.

### 관계 graph와 집단

observer node key는(type:u8,uid:u64)이며 Agent0,Parcel1,Media2로 구분한다. 4096tick 창마다 방향별 실제 resource 이전 총질량, 공개 정보 복사 cell 수, artifact 성공 접촉 횟수를 각 채널별 합산한다. 각 관계의 창 weight는 min(255,floor(log2(1+합계)))다. 한 번의 transfer를 여러 작은 사건으로 쪼갰다고 합계 weight가 커지지 않는다.

artifact 접촉은 matrix/pore transfer, link/unlink, work, 그 매체의 copy/write에 한정한다. 단순 근접·READ·같은 계통은 확정 관계 weight0이다. 관계 종류는 Resource0,Record1,Use2다. 같은 사건이 두 종류에 해당하면 각각의 의미와 raw 수치를 보존하되 관계 종류 수를 늘리려고 중복 새 종류를 만들지 않는다. signal 상관은 TentativeSignal이며 문명 후보의 두 종류 조건에 포함하지 않는다.

최근 4창의 weight 합을 무방향 LPA 입력으로 쓰되 원래 방향은 UI에 남긴다. cluster 판정의 agent 수는 창 끝에 살아 있는 구성원만 센다. 초기 label은 node key, pass 순서는 BLAKE3('group-a1.4', window ID, node key) 값과 node key순이다. 최대 20pass, 현재 label 우선 동률 규칙은 29절을 따른다. 누락된 관계 창이 있으면 해당 범위의 문명 승격은 보류하고 IncompleteEvidence다.

집단 ID 매칭은 agent Jaccard의 분자·분모를 정수 교차 곱으로 비교한다. 동률은 이전 ID, 새 후보의 최소 node key순이다. agent 교집합 0이나 artifact의 재사용이 이어지면 별도 ContinuationCandidate를 만들며 같은 문명이 확정됐다고 ID를 강제로 유지하지 않는다. 영토는 이동·물질·구조 이용 heatmap을 따로 보이고 높은 방문 수 하나로 모든 영향의 지배를 선언하지 않는다.

### 복사 계보·덮어쓰기·기술 상태

매체의 출처는 [start, end) cell 구간의 최신 작성 사실로 관리한다. COPY는 source의 캡처 revision과 구간을 참조하고 destination 겹침 부분만 교체한다. WRITE는 새로운 LiteralWrite 발생이며 READ 이력이 있더라도 CopiedFrom으로 승격하지 않는다. bit 마모와 산화는 DamagedFrom/DestroyedRange로 구간을 갱신한다. 재활용으로 0을 쓴 부분은 ErasedRange다. 버리기만 한 매체의 정보 출처는 owner 변경과 함께 유지한다.

DraftCommitted에는 prefix 길이·raw hash·descriptor·사용 매체 revision을 남긴다. 실행 증거는 그 commit occurrence와 연결한다. 같은 hash의 다른 장소 기록을 같은 전달 사건으로 합치지 않는다. immutable host blob 공유는 출처 edge를 만들지 않는다. module 변이의 source interval과 point edit를 저장한 경우에만 MutatedFrom/RecombinedFrom 사실을 만든다. ancestry 순서는(tick, stage, sequence, event ID)이며 같은 hash 반복도 DAG cycle을 만들지 않는다.

전달 증거는 실제 CopySpan→수신자의 Commit→해당 module 실행을 모두 요구한다. Transferred는 비친자 전달이 확인된 경우와 유전 전달을 배지로 구별한다. 기능 증거가 없는 실행은 FunctionSupported가 아니다. 기술 상태 LiveUsers는최근 4096tick 사용 증거, DormantRecord는유효 보유 기록만 확인, ArtifactOnly는기록없이 관련 구조 확인이다. 3창 사용 미관측이면 NotObserved로 옮기되 gap이 있으면 Unknown으로 표시한다. 증거가 다시 생기면 회복할 수 있다.

사용자가 보는 Recent Innovations는 event_tick과 detected_tick을 모두 가진다. 알림은 같은 family·증거단계에 대해 4096tick cooldown을 쓰되 사건 기록을 삭제하지 않는다. 무작위 신호·짧은 noise sketch·한 물건의 이동을 새로운 문명 기술로 반복 알리지 않는다.

### 저장량과 backpressure

필수 사실과 정수 관계 집계는 완결 창 단위 segment로 저장한다. 상세 행동 표본과 화면 알림은 제한 가능하다. 분석 queue가 가득 차면 FullAnalysis는 기다리고 LiveAnalysis는 Gap을 기록한다. archive RAM256MiB는 비용 상한이며 오래된 확정 record는 디스크로 내린다. 동일 입력의 offline 분석은 같은 A1.4 결과를 재생성해야 한다. 디스크 부족으로 전수 입력을 잃었으면 정확한 전체 문명·기술 수라는 표기를 중지한다.

**경계:** 새로운 W/P 없음. E는 세계에서 일어난 상호작용·재사용·변화다. U는 재현 가능한 근사 분석과 증거이며 선택압이나 기술 해금으로 되돌아가지 않는다.

## 59 Ruleset·저장 스키마·생성 ID·예약의 완결 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

### 실행 규칙을 읽는 단일 경로

R5의 기본 physics는 임의 JSON 키로 수식을 수정하는 범용 물리 엔진이 아니다. PhysicsSpecId='civ-grid-5'의 불변 규칙 집합이다. 05..22,31..35,39..44,52..57,59,63..66절의 정수·수식·단계·공학 경계와 조상·fixture 의미가 해당 규칙의 규범 문서 집합이다. 환경 크기·seed·초기 개체 수·capability만 RunConfig로 선택한다. 실험용 수치 변경은 새 PhysicsSpecId와 별도 실행을 요구한다. UI 설정에서 유지비나 반응률을 조용히 바꾸지 않는다.

Rules section에는 위 규범 절을 번호순으로 이어 붙인 UTF-8 rule_text를 저장한다. 정규화는 CRLF→LF, 행 끝 공백 제거, 각 절 끝 개행 하나이며 내용의 공백·숫자를 추가 해석하지 않는다. physics_hash=BLAKE3(rule_text)다. 파일의 문장을 실행하는 것이 아니라 지원 executable이 가진 동일 rule_text hash와 비교한다. hash가 같다는 사실이 구현 정확성 증명은 아니며 수치 gate를 함께 통과해야 한다. 설명을 수정해서 hash가 달라진 경우도 별도 ruleset으로 취급하는 보수적인 재현 방식이다.

이 방식은 R3의 '모든 상수를 개별 JSON 키로 넣는다'는 미완결 interface를 대체한다. 실행 중 누락 키에 최신 기본값을 적용하지 않는다. 명시하지 않은 physics override 키가 있으면 ConfigInvalid다. species/반응 표의 기계 판독 사본은 inspect용이며 원본 규칙과 다르면 load를 거절한다. 실행 코드의 제 2 설정 원천으로 쓰지 않는다.

| manifest 경로 | 형식·필수 값 | 의미 |
| --- | --- | --- |
| format_version |u32=5 |컨테이너 major5 |
| run_id, parent_run_id, parent_tick |hex16,optional hex16,optional u64 |branch이면 부모 두 필드 모두필수 |
| created_by_build |hex32 |실행 binary/build 식별 |
| physics_id, physics_hash |'civ-grid-5', hex32 |고정 규칙과 rule_text checksum |
| run_config.width, height |u32,64배수, 기존공간상한 |Run 시작 뒤 불변 |
| run_config.seed |hex16 |16byte seed |
| run_config.initial_agents |u32 |초기 입력의 개체 수 |
| run_config.worldgen_version |u32=3 |WorldGen-3 수식 유지 |
| run_config.capability, run_purpose |u8 0..2,u8 0..3 |v0/v1/v2, Natural/Fixture/Ablation/Intervention |
| ancestor.raw_length, raw_hash, section_ref, revision |u32,hex32,u16=1,'founder-4'또는명시이름 |실제 원본 bytes가 함께 있어야 함 |
| history.mode, checkpoint_interval, full_every |u8 0..1,u64≥1,u32≥1 |기본 0,10000,10 |
| history.segment_records, segment_bytes |u32≥1,u64≥1 |기본 65536,16777216 |
| toolchain.rustc, target, cargo_lock_hash, commit, dirty_hash |문자열, 문자열, hex32,문자열, optional hex32 |빌드 재현 정보 |

그 외 top-level key는 거절한다. operational 설정과 AnalysisConfig는 별도 문서다. JSON optional은 명시적 null 또는 값이며 branch가 아닐 때 parent 두 필드는 null이다. 같은 초기 상태의 두 실행은 다른 run_id를 가질 수 있지만 run_id·생성 wall time은 CoreHash와 RNG에 넣지 않는다. physics_id/ancestor 이름 문자열은 길이 1..64 ASCII이며 arbitrary 경로를 실행하지 않는다.

### 저장 필드의 추가 정의

컨테이너 CIVSIM5/major5/minor0의 header128byte와 directory72byte는 39절을 따른다. R3/R4 파일을 이 구조로 자동 해석하거나 다른 physics로 조용히 이어 실행하지 않는다. R5에서는 각 section schema=1이며 page hole·재활용 cursor·실행 prefix·소유 종류를 빠짐없이 저장한다.

| type | wire 필드 순서 | 검증 |
| --- | --- | --- |
| Media |uid:u64,owner_kind:u8,owner_value:u64,logical_len:u16,slot_count:u16,slots, revision:u64,exposure_clock:u64,RecycleState? |slot_count≤owner상한, logical_len≤128n |
| PageSlot |kind:u8,Present이면 128×u16 |Absent에는 cell payload 없음 |
| RecycleState |page_index:u16,next_cell:u16 |마지막 Present 페이지, 0..127 |
| ProgramView |media_uid:u64,used_cells:u16 |유효한 prefix, 빈 hole없음 |
| ModuleSlot |slot:u16,tag:u16,media_uid:u64,start_word:u16,length:u16 |ProgramView 범위, slot 재사용 조건 |
| BindingTable |inherited:ProgramView, learned:ProgramView?, entry_tag:u16,revision:u64,slots 목록, active 목록 |활성 tag유일, pc/frame/pin의 참조 일치 |
| ActiveBinding |tag:u16,module_slot:u16 |tag 오름차순 |
| Embryo |uid:u64,parent_uid:u64,ordinal:u64,state:u8,body_g:u16,shell_s:u16,work_g:u8,energy:u32,raw_media_uid:u64?, result_media_uid:u64?, used_cells:u16,mutation_applied:u8,plan?, cursor:u16,return_inventory10×u16,validation? |정상 checkpoint에출생예약 없음 |
| Validation |media_uid:u64,revision:u64,used_cells:u16,genome_format:u16,result:u8,charged:u8 |result0유효/1무효, charged=1만보존 |
| SurfaceWork |tile:u32,kind:u8,credit:u32 |R5 Compress/Abrade는완성단위만처리하여정상목록비움 |

MediaOwner0..3은 agent UID, 4..5는 embryo UID, 6은 parcel UID, 7 Dropped는 TileIndex를 u64로확장한 값이다. Corpse라는 표시는 UI에서 사망 유래를 설명할 수 있으나 code 접근 권한을 바꾸지 않는다. owner가같은 두 Record가 같은물리페이지를 공유할 수 없다. host interning은 ProgramBlob에만 적용한다.

유효한 stored BindingTable의 slots는 slot오름차순이고 active는 tag오름차순이다. inherited/learned의 descriptor는 각 ProgramView의 rawprefix에서 재구축한다. retained slot은 해당 media가 실제존재해야 한다. refcount와 decodecache는재구축한다. 실행 prefix밖 suffix는 ProgramBlob 실행 bytes에포함하지않고 Media에남긴다.

PendingAdopt, PendingBirth, UidTicket, action budget, patch, sense overlay는 tick내 scratch라완결 checkpoint에는없다. 반면 RecycleState, MutationPlan 진행 cursor, 검증비 cache는여러 tick에영향을주므로저장한다. load는모든 store를파싱한뒤참조·owner·pageG·열용량·보존식을검증하고성공한 state만공개한다.

### 병렬 생성과 UID 예약

생성 후에야 UID를 정하면 같은 tick의화학·매체·사건순서를정할수없다. stage3에서필요한 UID ticket을먼저정한다. potential create를(kind, actor_uid, embryo_ordinal, intent ordinal, within-intent ordinal)순으로정렬하고각 type의 next_uid에서단조배정한다. ID 종류의 고정순서는 Agent0,Parcel1,Media2,Embryo3다. 실패한생성의 ticket은다시쓰지않아 UID에빈번호가생길수있다.

potential create 상한은 Allocate에 Embryo1,처음/추가 Resize에 Media1,Step에결과 Media1,빈 Matrix 입고에 Parcel1,Release에 Agent1이다. 기존객체가있는 Resize/Step에도 ticket을예약할수있으며사용되지않으면폐기한다. 페이지는별도역사 UID가없고 pool slot만예약한다. 이고정규칙을현재메모리상황에따라바꾸지않는다.

ID counter갱신은 stage3의모든예약과수치상한검사가성공한뒤다. 실패하면 Wt의 counter도그대로다. 사용될 pool slot은동일정렬순서로작은 free slot부터예약하고, 남는예약은 stage11에서반환한다. 공개되지않은예약 slot의 generation은증가시키지않는다. 이번 tick삭제된 slot은다음 tick부터재사용한다. public 참조는사용된 UID와 generation만가진다.

CoreHash에는미래 UID할당에영향을주는 next_uid와 RNG counter도포함한다. slot layout·free list·generation은 ReplayStateHash에추가한다. 단순히현재그림이같다는이유로미래난수가달라지는 state를동일 CoreHash로판정하지않는다.

### 첫 mutation 뒤 allocation 실패를 막는 범위

stage3은 agent intent뿐 아니라 이번 due의환경·사망·매체 손상에서생길최대 residue/사실 record와 11단계해제목록을예약한다. agent당배아·매체 수가유한하므로사망시에는현재소유목록길이로정확한상한을계산한다. preflight는승리할 agent를예측하지않고허용된모든후보의상한을합친다. 예약이과하면실측후 allocator를최적화할수있지만물리적실패로대체하지않는다.

worker는예약된 store slot·page span·event span·bounded patch를받는다. Vec push가 capacity를넘기거나새 hashmap allocation이필요한경로는 InvariantViolation이다. ProgramBlob interning은예약영역에후보를쓰고 coordinator가정렬병합하며중복메모리를회수한다. host panic이난 tick의부분 state를정상 checkpoint로저장하지않는다.

**경계:** Engineering Contract는파일·ID·소유권·예약이다. W/P 의미를보존하며 E의결과가 thread·allocator·schema추측에따라달라지지않게한다. U는호환불가·정지·복구를정확히보여준다.

## 60 추가 수치 사례·상호작용 검증·납품 gate

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

T01..T45와 아래 T46..T72는 모두 미래 구현의 기대값이다. 표를 작성하거나 계산식을 대조한 것은 simulator test 실행과 다르다. 테스트에서 시도비를 이미 냈다고 적은 경우만 이를 제외한다.

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| T46 |공개 2page, cursor없음, Resize1을 32번, 에너지충분 |G1회수, cell처리 128,작업 4096+시도 2048q;VM별도 |
| T47 |T46에서 10회후일반 WRITE가마지막 page수정 |cursor0,기존삭제 40cell의과거비용환불 0 |
| T48 |3page에서중간 page1산화 |page1hole, page2주소 256유지, G1반응만계상 |
| T49 |T48에서 Resize목표 3 |page1만 zero복구, G1+초기화 128+시도 64q |
| T50 |source COPY만 두번, source수정없음 |source revision불변, 두 destination복사성공가능 |
| T51 |같은 tape 겹치는 COPY |Think원본 prefix사용, destination revision1증가 |
| T52 |CancelDraft로 2page버림 |같은 UID/bytes의 Dropped매체, G2즉시환불 0 |
| T53 |배아 Cancel:bodyG16/S2,workG1,raw2page, result1page, E1000 |몸체재료 G17/S2반환, 매체 3page남음, U+1000 |
| T54 |learned2page교체, oldframe1개 |old물리매체유지·유지비계상, 마지막 RET후 Dropped |
| T55 |usedprefix100,logical_len180,capacity256의 Commit |뒤 80cell보존, 유료 2page유지, 실행 prefix100 |
| T56 |COPY source_offset126,count4,page1hole |최대 2cell복사, Partial/actual2 |
| T57 |P1/X1/C1215,U1105650,밤 due, Rx4credit65520 |sky후 1100610,반응후 U1099570,M2/C1/V1 |
| T58 |C1032,U=900C, X256,밤 due, 작은노출혼합물 |sky가 280C까지냉각, Rx4 실행 0 |
| T59 |창합계물질 16을 1회또는 16회같은상대에게이전 |집계 mass동일, 관계 weight동일 |
| T60 |유사성 sample U길이 8,그중양쪽공유 2 |Jhat1/4,novelty의엄격<1/4는 false |
| T61 |같은 parcel집합을 10회이동·관측 |동일 pattern의독립제작 occurrence1 |
| T62 |빈 sketch둘, 또는 effect2개창 |ShortTrace, 새기술승격 0 |
| T63 |전달후 COPY영역중간 WRITE |그구간 CopiedFrom제거, 나머지출처유지 |
| T64 |thread1/8에서같은 potential생성, 절반실패 |사용 UID·빈번호·next_uid모두동일 |
| T65 |stage3 pool예약실패 |VM·RNG·UID counter포함 Wt완전불변 |
| T66 |media반환·사망발생후저장 |UID1회, 물리 G1회, Corpse/Dropped목록중복 0 |
| T67 |FullAnalysis queue가득/LiveAnalysis queue가득 |각각대기/Gap;Core결과동일 |
| T68 |L256 무변이복제, 1024q씩 16회투자 |기술된열소산하한 15232q+자식 16384q, VM/유지/이동별도 |
| T69 |E65535,G46,호흡조건계속유지 |총가용잠재량 142171q;생존보장주장아님 |
| T70 |PublicTape가매체 owner변경뒤같은 tile에남음 |revision증가, 옛 COPY캡처 Stale |
| T71 |존재하던 parcel파괴뒤동일 tile재생성 |옛 ObjectKey효과 Stale, 새 UID에권한상속 0 |
| T72 |amount0이지만 Boundary주소 |Boundary, Noop로우회불가 |

효과 조합 검증은 적어도 WRITE/COPY/Resize/반응/owner변경의모든순서쌍과, Cancel/Death/Release/Commit의모든순서쌍을작은 tile fixture에서실행한다. 비용·UID·bytes·고정 tick이같은 serial oracle와비교하고, 전체 프로그램을단순히두번호출하는시험으로독립 oracle를대체하지않는다.

경제 회계 oracle는 10종물질위치의원소·h·c·질량과 tile U, agent/배아 energy, photo_credit, Ug를모두합산한다. 원자 transaction마다전후차이와외부입출력을비교한다. 음수 U·잃어버린페이지 G·중복코드물질을 saturating연산으로숨기면실패다. 수식의정수 floor경계는 a×b가 64bit를넘는경우까지 u128중간계산으로검사한다.

파일 gate는모든장기상태가활성인 checkpoint를사용한다. pagehole, RecycleState, next UID의빈번호, 진행중 MutationPlan, retainedframe, 배아 Invalid, signal, reactioncredit, RequiredHistory경계를하나의 fixture에포함한다. load직후 hash와이후 1000tick의 hash열을대조한다. unknownschema, 중복 UID, 잘못된 owner, 과대한 length, 잘린압축 stream은현재실행을훼손하지않고거절한다.

성능 gate에는 55절로늘어난외부매체와 provenance비용을포함한다. 버린매체를자동삭제해서좋은 TPS를만들지않는다. S3 report는 median/p95/p99뿐아니라매체수, Present page수, hole수, drop율, 필수 event bytes/tick, 예약량/실제사용량비를함께낸다. 실측전에는이수명모델의 2..5GiB예산을확정치로표시하지않는다.

**경계:** Engineering Contract는검증과납품조건이다. fixture는 E의자연발견이아니다. U는통과·실패·미실행을구별한다.

## 61 이전 R4 감사 기록과 남아 있던 구현 gate

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

이 절은 R4에서 수행한 문서 수정의 이력이다. 현재 실행 규범은 R5 본문과62..67절이며 아래의 이전 구현 순서는67절로 구체화했다.

| 결함·빈틈 | R4 변경 | 남는 확인 |
| --- | --- | --- |
| 취소·교체로기록 G무료회수 | 모든폐기매체보존, 유료재활용통일 | T46..T55와메모리실측 |
| 페이지손상뒤주소재배치미정 | hole보존·높은 index부터산화·복구규칙 | T48/T49/T56 |
| COPY source revision증가의잘못된규정 | 쓰기/페이지/소유변경에만 revision증가 | T50/T51/T70 |
| suffix정리와채택의복잡한장기잠금 | 실행 prefix와기록길이분리, 실제 data보존 | T55와 save/resume |
| hot반응전외기냉각조건누락 | 실제반응시점·단열·누적 credit비용검증 | T57/T58와완전제조 witness |
| 초기조상행동의예외미정 | 우선순위·partial복사·수리·대기표 | 실제 bytecode조립·trace·생존실험 |
| sketch비교·artifact중복·edgeweight편향 | 정확한표본비교식·독립 occurrence·창합계 | offline oracle·오탐률 |
| 필드별 physics JSON명세미완결 | 불변 RulesetID와규범 text hash | executable정합성 gate |
| 병렬생성 UID확정시점불명확 | stage3 ticket·실패빈번호·slot예약 | T64/T65와멀티스레드대조 |
| 환경·사망이벤트예약범위누락 | due상태까지상한예약 | 최악부하·memory gate |

이개정은문서에서닫을수있는실행조건을추가로닫은것이다. 물리규칙을실행해본증거, 조상바이너리, UI실물, release build, 성능측정은아직없다. 페이지수나검증항목수로최종구현준비율을계산하지않는다.

첫구현순서는원소·에너지 oracle→페이지수명과주소→serial효과 resolver→VM/조상조립→복제·저장→기본 2D지도→병렬동등성이다. 조상·witness가실제일주기에서실패하면그실패의원인을분류하고규칙또는시작프로그램을명시적으로개정한다. 고수준행동·보상·완성구조를몰래주입해성공처럼보이게하지않는다.

문서동결의기준은알려진필수상태의누락 0,모순된 ABI0,각외부효과의비용·실패·소유권·저장필드가모두연결됨이다. 실행동결의기준은 60절·48절의 test, determinism, save/resume, performance gate통과다. 문명과누적기술에대한연구주장은 49절별도실험을통과한범위에서만한다. 이세상태를하나의'완성'표시로합치지않는다.

**경계:** W/P개정은 Rulesetrevision으로기록한다. E는관측된실행결과만을뜻한다. U와문서의완성도평가는증거의수준을명확하게표시한다.

## 62 조합 가능성·유용성·자연 발견을 분리하는 설계 gate

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

이 개정은 primitive의 수를 늘리는 대신 기존 연산 사이의 물질·힘·정보 경로가 닫히는지 검사한다. 저장소를 만들 수 있다는 말은 산소가 들어오는 경로와 내용물을 다시 꺼내는 경로까지 정의됐다는 뜻이어야 한다. 모듈을 복사할 수 있다는 말은 그 모듈을 호출할 수 있고 필요한 입력·재료를 확보할 수 있다는 뜻과 다르다.

| 판정 | 필요한 증거 | 이 판정만으로 말할 수 없는 것 |
| --- | --- | --- |
| R0 규칙상 도달 가능 | 금지된 상태 전이 없이 primitive 절차를 기술하고 불변식을 대조 | 실행 성공·효율·자연 발견 |
| R1 실행 가능 | 동일 ruleset의 실제 VM witness가 시작 상태에서 종료 조건 달성 | 지속적인 생존 이득 |
| R2 재사용 가치 | 제작·운영·유지·회수·실패를 포함한 대조 실험 | 변이가 그 절차를 쉽게 발견함 |
| R3 자연 발견 | 공개 조상으로 시작한 미개입 실행에서 발생, 투입 경로 공개 | 장기 누적·사회적 확산 |
| R4 누적·전파 | 비친자 전달·조합·실행·선행 기록 소실 뒤 재현 증거 | 무한한 새 기능 또는 모든 문명 현상 |

이 표의 R0..R4는 증거 등급이며 문서 개정 번호와 무관하다. 현재 simulator가 없으므로 아래 witness의 상태는 전부 R0 설계 또는 아직 미확인이다. 절차표를 쓴 것만으로 R1에 올리지 않는다. 프로그램이 유용한 중간 행동을 거쳐 복잡해질 수 있다는 연구가 있어도 그 연구의 보상 체계와 본 세계의 내생 선택압을 동일하게 취급하지 않는다. [Lenski 등, The evolutionary origin of complex features](https://cse.msu.edu/~ofria/pubs/2003LenskiEtAl.pdf)

### 기본 세계에서 열려 있는 경로와 막힌 경로

| 목표 기능 | 필요한 저수준 연결 | 현 설계의 한계·검사 |
| --- | --- | --- |
| 공간 이동·재료 채취 | 감각→DRIVE→TRANSFER | 비용·고도·용량·경쟁을 모두 통과해야 함 |
| 보관·공유 재고 | 배치→pore 입고→교환·반응→출고 | 자동 소유권·완전 밀폐 없음;63절 적용 |
| 운반·통로 | 결합→하중 전달→이동→해체; 표면 압축 | 힘 전달 64절, 전체 비용 65절; 운반 효율 보너스 없음 |
| 열 가공 | 원료 혼합→열 공급→손실보다 긴 유효 반응→회수 | 고온 1회와 지속 가능한 생산은 다른 gate |
| 도구 사용 | 특정 물체의 이동·차폐·용량·열·접촉 효과 | 지렛대·기어·날·원격 조작을 아직 표현하지 못함 |
| 신호 중계 | READ→산술·기억→유료 EMIT 반복 | 의미·주소·재전송·동기화는 프로그램의 관례 |
| 지식 전달 | 유료 기록→COPY→형식 검사→binding→실행 | tag·register·entry 호환성과 비용을 66절에서 검사 |
| 거래·분업·집단 | 반복되는 개별 이전·기록·공간 이용 | 계약 이행과 직업 배정의 host 처리는 없음 |
| 수소 연료·수소 환원 | H의 최초 생성 또는 외부 유입 | 기본 세계에는 생성 경로가 없어 도달 불가 |
| 자동 기관·발전·컴퓨터 회로 | 물리적인 힘·에너지 변환·상태 갱신의 추가 법칙 | 기본 v2에 없음; 외형만으로 구현했다고 하지 않음 |

WorldGen-3의 H=0이며 Rx0..Rx8 중 H를 생성하는 반응은 없다. 전이·분할·복제도 H를 만들지 않는다. 따라서 물질 개입이 없는 기본 실행에서 H의 전 세계 총량은 항상 0이고 Rx3/Rx5는 일어나지 않는다. 이 불변식은 문서만으로 증명할 수 있다. 해당 종·반응은 현재 10종 ABI의 예비 경로와 물질 투입 fixture에 남기되 기본 문명의 발명 가능성으로 세지 않는다. H 투입 또는 새 반응 도입은 별도 RunPurpose/PhysicsSpec의 명시적 변경이다. 반응 표에 행이 있다는 이유로 생산 경로가 있다고 주장하지 않는다.

무제한 혁신을 설계상 보장하지 않는다. 유한 공간·유한 프로그램·고정 물성으로 가능한 상태에는 상한이 있다. 목표는 관측 기간 동안 새 조합·용도·관계가 얼마나 계속 생기는지 검증하는 것이다. 지원하지 않는 기계가 목표 달성에 필수라고 판명되면 그때 일반 물리 확장을 별도 결정으로 검토한다. 자동 공장·직업·문명 보너스를 대신 넣지 않는다.

**경계:** W는 도달 가능한 상태 공간과 비용이다. P는 기존 명령만 사용한다. E는 실제 실행의 재사용·발견·축적이다. U는 위 증거 등급과 지원 한계를 표시한다.

## 63 저장 공간의 투과·표면 반응·정수화 계약

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

R4에는 pore와 loose 사이의 자연 교환이 없었다. 적은 matrix에 G만 넣으면 압축도와 무관하게 외부 산소를 차단할 수 있는 결함이다. 또한 자연 flux와 반응 credit을 매번 floor하면 작은 양의 흐름이 영원히 0이 될 수 있었다. R5는 같은 세계 법칙으로 두 경로를 보완한다. 이것은 Warehouse 기능이 아니라 재료가 차지한 공간과 그 경계의 수송·반응이다.

| 대안 | 장점·open-endedness | 단점·성능·복잡도와 선택 |
| --- | --- | --- |
| pore를 항상 밀폐 | 저장 의미와 실행이 단순 | 작은 재료로 무료 밀폐, 환경과 단절; 제외 |
| 매개변수 있는 국소 교환 | 배치·압축·용량·내용물의 조합이 영향을 줌 | 교환 순서·표면 반응 추가, O(활성 parcel); **선택** |
| 연속 압력·다상 유동 | 더 풍부한 유체·용기 가능성 | 전 세계 압력 해법과 새로운 열역학 필요; 후속 연구 |

### 손실 없는 양자화

자연 수송 후보 N/D의 정수화는 SR(N, D)=floor(N/D)+[r<N mod D]다. r은 0..D-1의 균일 정수이고 대괄호는 0 또는 1이다. 실제량은 min(32,SR), 원천 재고·목적지 공간·열·고도 예산으로 다시 제한한다. N=0이면 0이다. 양수 소수의 기대 흐름을 남기지만 임의의 tick에서 이동을 보장하지 않는다. 이미 제한된 양을 다시 반올림하지 않고 남은 소수량을 물질로 저장하지 않는다. 이 규칙은 질량을 생성하지 않는다.

tile 간 기체는 N=abs(nA-nB)×minP, D=2048이다. 같은 고도의 W는 D=8192를 쓰고 N은 동일하다. 내리막 W는 N=32×minP, D=256이다. 오르막 W는 여전히 0이다. 기존 edge bucket·네 color·열 이동·위치 에너지 지불은 유지한다. 예를 들어 기체 재고 차 1,P256이면 후보 1의 확률은 1/8이다. 입력 r=255이면 1,r=256이면 0이다. “차가 8보다 작으면 영원히 섞이지 않음”은 제거된다.

### pore와 같은 tile loose 사이 교환

stage7은 신호 next 계산→due parcel의 pore 교환→기존 네 color edge 수송 순서다. pore due는 현재 소재 tile의 16tick due와 같다. 같은 tile 안의 교환은 U·Ctile·Ug·화학 에너지 총량을 바꾸지 않는다. 부피 용량 K는 11절 그대로이며 K0이면 pore가 비어 있어야 한다.

기준 외부 부피 Vref=64는 농도 비교용 모델 상수이며 loose의 실제 저장 상한을 64로 바꾸지 않는다. 각 종에 대해 d=nLoose×K−nPore×64를 계산한다. d>0이면 loose→pore, d<0이면 반대다. 기체 C/O/H/V 순서로 N=abs(d)×P, D=64×2048, 다음 W는 D=64×8192를 쓴다. 각 종의 실제 변경 후 다음 종을 계산한다. 기체의 유효 부피 4와 pore의 전체 여유를 적용하며 G/X/M/S/I는 자연 이동시키지 않는다. 극미량에서 반올림 1이 연속 농도 평형을 한 양자 넘어설 수 있으므로 이 모델을 연속 유체의 정확한 해라고 부르지 않는다.

예를 들어 matrix S1,c0이면 P256,K64다. loose O8,pore O0,빈 공간 충분이면 N/D=1이므로 O1이 pore로 들어온다. pore에 G64가 있으면 여유 0이므로 기체 입고는 0이다. **그렇다고 꽉 찬 유기물이 공기 중에서 산화하지 않는 것은 아니다.** 아래 공통 표면 반응이 이 경계를 처리한다.

### 내부 반응과 노출 표면

외부 반응 site 순서는 Loose0→Matrix1→Pore2→PoreBoundary4→Media3의 UID순이다. 번호의 크기가 실행 순서인 것은 아니다. Pore 내부는 자기 기체를 사용한다. PoreBoundary는 Rx2/Rx5/Rx6만 대상으로, 응축 반응물은 pore, 기체 반응물은 같은 tile loose에서 받는다. 기체 생성물은 loose, 응축 생성물은 pore로 보낸다. 따라서 G64로 가득 찬 pore도 표면의 G1+외부 O1→pore W1+loose C1 반응이 가능하다. 원래 물질·원소·에너지·용량 검사는 모두 적용한다.

Rx2/Rx5/Rx6의 내부와 경계 반응은 서로 다른 물리 접촉 경로와 credit이다. 재고를 소비한 다음 site는 갱신된 재고를 검사한다. Rx4처럼 같은 compartment의 응축 원료끼리 반응하는 경우는 boundary에 중복 등록하지 않는다. Rx4/Rx5의 O≤1은 해당 반응이 기체를 접촉하는 저장소 기준이다. Matrix/Loose/Boundary는 loose O, Pore 내부는 pore O를 검사한다.

P≥32는 agent의 능동 pore 입출력 접근 조건이다. 자연 교환과 외부 기체 반응은 이 문턱을 사용하지 않는다. matrix와 노출 매체의 외부 기체 접촉도 P32에서 갑자기 끊기지 않는다. X=max(1,floor((Pself+Σmin(Pself,Pneighbor))/5))를 적용하므로 지도 모서리에서도 P와 X는1 이상이다. 차폐는 속도·접근을 바꾸며 완전 보존 특권을 부여하지 않는다. 일반 site의 반응 credit 증가는 SR(rate×X×min(16,가능 extent), 256)이다. PoreBoundary는 내부까지의 추가 투과 경계를 통과하므로 SR(rate×X×P×min(16,가능 extent), 65536)를 쓴다. 고유한 저장 보너스나 재료별 용도 판정 없이 같은 P를 사용한다. 두 식 모두 threshold65536,잔여 cap65535,site·반응당 최대 4extent를 유지한다. 온도·산소·수용량·U 조건이 실패하면 가능 extent0이다.

현재 모델의 X는 열·빛·반응에 공유하는 노출 근사다. 실제 확산은 물질·온도·농도 등에 의존한다는 원리는 참고하지만, 위 정수 수식·상수는 본 프로젝트의 설계값이며 실측 계수가 아니다. [NIST, Diffusivity and Mobility Data](https://www.nist.gov/publications/diffusivity-and-mobility-data)

### 추가 저장량 없이 결정론 유지

FluxRng-1도 Philox4x32-10을 쓴다. key=BLAKE3(ASCII 'civ-flux-v1'+0 byte+seed16+domain u32 LE+selector u32 LE)의 처음 8byte이며 counter=(object_lo, object_hi, tick_lo, tick_hi)다. lane0 AND(D-1)로 r을 얻는다. 본 절의 D는 모두 2의 거듭제곱이다. 일반 VM RNG의 주소 파생식과 혼용하지 않는다.

domain8=EdgeFlux, selector=species, object=(minTileIndex<<32) OR maxTileIndex; domain9=PoreFlux, selector=species, object=ParcelUID; domain10=ReactionRound, selector=(site_kind<<8) OR reaction, object=Loose는 TileIndex, Matrix/Pore/Boundary는 ParcelUID, Media는 MediaUID다. tick마다 해당 site·reaction의 credit 증가는 한 번만 계산한다. site 종류와 domain이 키를 구분하므로 같은 숫자의 UID도 충돌하지 않는다.

키는 유한한 domain/selector 조합별 derived cache에 둘 수 있다. edge마다 hash 파생·잔여 배열·event counter를 저장하지 않는다. 계산을 생략해도 다른 객체의 난수열이 밀리지 않는다. parcel이 제거되면 그 UID의 내부·경계 credit도 제거하며 새 parcel이 이를 상속하지 않는다. 이 credit은 물질·에너지 자체가 아니다. 새 authoritative 자료는 PoreBoundary의 최대3개 u16 credit뿐이며 한 parcel당 payload 최대 6byte다. 실제 sparse key/index 비용은 별도 측정한다.

**경계:** W는 투과·경계 반응·정수화다. P는 기존 TRANSFER/WORK만 있다. E는 상대적인 보존·차폐·배출·가공 가능성이다. U는 보관 효과를 실제 손실·회수량으로 판단한다.

## 64 연결 구조의 하중 전달과 조작 범위

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

R4는 grip 강도를 검사했지만 중간 parcel 결합이 전달할 수 있는 하중을 명시하지 않았다. 그 상태에서는 strength1 연결 하나로 큰 조립체를 운반할 수 있었다. R5는 각 물질의 이동 저항이 실제 연결을 통해 작용점으로 전달되는지 검사한다. 고급 도구 enum이나 운반 능력치는 추가하지 않는다.

| 방식 | 장점·open-endedness | 단점·성능·복잡도와 선택 |
| --- | --- | --- |
| 연결 유무만 검사 | O(16), 간단 | 결합 강도가 운반에 무의미; 제외 |
| 고정 spanning tree 하중 | O(16), 중간 약한 결합 반영 | 루프의 병렬 경로가 탐색 tree 선택으로 사라짐; tree에서만 최적화 |
| 작은 graph의 용량 판정 | 루프·보강·약한 연결이 실제 영향 | cycle에서 flow 계산 추가;16 parcel 상한 아래 **선택** |

각 parcel v의 실제 질량 m에는 matrix, pore, 붙어 있는 tape를 포함한다. 이동 후까지의 Manhattan 거리 d와 목적지 마찰 μv, 고도차 Δz로 demand(v)=ceil(m×d×μv/960)+max(0,m×Δz)를 계산한다. 이는 결합에 요구하는 하중의 모델 단위다. action의 실제 에너지 차감은 기존 10절의 합산식이며 demand를 열이나 가용 에너지로 다시 계상하지 않는다. parcel별 ceil과 내리막 상쇄 금지 때문에 보수적인 강도 검사다.

활성 face bond는 양방향 용량 16×strength를 갖는 graph edge다. 각 parcel에서 sink로 demand(v)의 edge를 두고 총 demand가 모두 전달되는지 검사한다. Translate/Rotate에서 source→grip parcel 용량은 16×grip_strength다. actor 자신의 몸체·재고·배아 이동 비용은 actor가 지불하지만 parcel grip를 거쳐 전달되는 하중에는 넣지 않는다. R4의 전체 actor 질량까지 grip에 넣던 규정은 이 정의로 교체한다.

PushParcel에서는 source가 actor와 접촉하는 앞 parcel이며 source 용량 16×min(255,bRoot)다. 접촉 지지에는 재료의 같은 cohesion 상한을 쓰므로 grip 검사 전체를 공짜로 우회하는 무한 힘이 아니다. 밀기는 한 칸 앞으로만 가능하고 끌기·회전에는 grip가 필요하다. actor의 에너지 상한, 목적지, anchor, 다른 grip와 footprint 검사는 그대로다. 여러 actor의 힘 합산은 현재 지원하지 않는다.

모든 demand를 보내는 flow가 없으면 Blocked/actual0이다. 시도비와 계산비만 지불하고 위치·결합·재고는 불변이다. 실패를 자동 파괴·마모로 해석하지 않는다. 결합을 약화시키는 실제 UNLINK·재료 손실은 별도 규칙이다. 이 graph는 준정적인 스칼라 하중의 근사이며 휨·회전 토크·연속 응력·충격 해법이 아니다.

예를 들어 root demand0,끝 parcel demand80,grip strength10,두 parcel의 bond strength4이면 source160이 있어도 중간 용량 64여서 실패한다. bond strength5이면 80을 전달할 수 있다. 모든 강도·질량이 같아도 anchor가 남으면 이동은 실패한다. 루프의 두 경로가 각각 48과 32의 잔여 용량을 제공하면 합 80이 가능하며 임의 tree 하나만 골라 실패시키지 않는다.

### 구현·기하 계약

연결 성분 1개는 직접 비교, tree는 root 기준 subtree demand 합과 edge 용량을 O(n)에 대조한다. cycle은 최대 16개 parcel과 source/sink에서 정수 Edmonds–Karp 방식의 독립 reference를 쓴다. residual BFS의 이웃 순서는 node의 ParcelUID 오름차순, source/sink는 고정 마지막 순서다. 양방향 edge는 각각 cap16s의 directed arc와 각 residual reverse arc로 표현한다. 반대 방향 flow가 동시에 있으면 net flow로 상쇄할 수 있고 용량 합으로 증폭되지 않는다. flow 배치 자체는 세계에 저장하지 않는다.

모든 중간합은 u128로 검사하고 total demand가 source 용량을 넘으면 즉시 실패한다. 최대 크기의 고정 scratch를 worker별로 재사용한다. 모든 최적화는 graph reference의 통과/실패와 정확히 같아야 하며 화면상의 구조 이름으로 fast path를 고르지 않는다.

LINK의 두 endpoint가 모두 actor의 Manhattan1 이내여야 한다는 ABI를 유지한다. 4방향 인접한 두 blocking parcel은 actor가 어느 endpoint에도 설 수 없어 그 자리에서 새 bond를 만들 수 없다. 먼저 통과 가능한 상태에서 연결한 뒤 바깥에서 압축하는 절차는 가능하다. 이 선행조건을 건설 witness에 포함한다. structure를 클릭했기 때문에 내부 연결을 자동 완성하지 않는다.

grip가 있는 DRIVE Rotate는 grip parcel 중심으로 actor와 연결 parcel들의 위치를 함께 회전한다. 전·후 위치를 예약하고 actor와 모든 parcel의 orientation에도 같은 quarter-turn을 더한다. 일반 WRITE Orientation은 actor의 감각·진행 방위만 바꾸며 물질 위치·grip·parcel orientation을 바꾸지 않는다. 중간 회전 궤적을 통과하는 충돌은 이 격자 모델에 없다.

**경계:** W는 cohesion·연결 용량·준정적 이동이다. P는 기존 LINK/UNLINK/DRIVE다. E는 하중에 맞춘 조립·보강·우회 경로다. U는 실제 용량·질량·실패 지점을 표시한다.

## 65 저수준 절차의 비용·회수·생존 예산

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

다음은 실제 simulator 실행을 대신하는 결과표가 아니라, 구현 후 확인할 witness의 입력·절차·기대 회계다. RunPurpose=Fixture의 손작성 절차를 기본 Founder에 넣지 않는다. 시작 재료·상태·에너지를 manifest에 공개하고, 측정 중 자연 수송·대사·손상·동시 행동을 모두 켠다. 특정 stage를 분리한 산술 시험은 전체 기능 witness와 따로 표시한다.

### 보관: 생성·접근·손상·회수까지

고도 0의 빈 대상 tile에 loose S64/G8을 두고, 접촉 actor가 실제 TRANSFER로 S64를 matrix에 넣는다. WORK Compress로 c224를 만들고 G8을 pore에 넣는다. 최종 s64,P32,K8이고 actor는 밖에 있어야 한다. 직접 loose→matrix/pore 이전을 사용할 때 세 효과의 작업·시도 비용은 1088+14400+192=15680q다. VM·감각·대기·유지는 별도다. 이 가격에는 물질 S64가 소비되어 해당 위치에 묶이는 기회비용도 추가로 보고한다.

G8을 담은 채 c225로 압축하려 하면 K7이 되어 Capacity다. c224에서는 P32이므로 G를 다시 꺼낼 수 있다. natural gas 입고가 용량 때문에 멈춰도 63절의 표면 산화는 가능하다. 압축이 모든 물리 작용을 차단하는 스위치는 아니다. 화학·물 교환 뒤 바뀐 내용물을 그대로 회수하며 처음 넣은 G8을 host가 되돌려주지 않는다.

대조는 같은 총 S/G·초기 열·공간 범위에서 G를 loose에 둔 경우와, 같은 parcel의 열린 c0 상태를 각각 둔다. 보관 기간, 남은 G, 실제 회수 G, 건설·출고 q, 회수 불능 비율을 측정한다. 압축에 의한 외부 환경 변화와 pore 자체의 효과를 구분한다. 더 나은 결과가 없으면 보관 기술의 기능 gate 실패다. 구조가 생겼다는 사실만으로 성공시키지 않는다.

### 통로: 전체 비용의 회수 시점

μ256의 빈 표면을 c128로 압축하면 μsurface192다. 실제 이동 질량 960인 개체의 동일고도 한 칸 이동 소산은 512→448q로 64q 줄어든다. 이동 시도비 64는 양쪽에 같아 차이에 영향이 없다. 압축 128×64+시도 64=8256q이므로 건설의 작업·시도비만 회수하려면 129번 통행해야 한다. VM·유지·우회·다른 개체의 변경·정체 비용을 포함하면 회수 시점은 더 늦거나 오지 않을 수 있다.

이 현상은 RoadType 판정이 아니라 surface_c가 마찰식에 들어간 결과다. 다른 지형·하중·통행 수에서는 손익이 달라진다. 물질을 배치한 낮은 parcel 경로도 별도 비교하되 건설 재료·결합·압축 비용을 빼지 않는다. 선형 구조라는 이유만으로 UI가 길을 확정하지 않는다.

### 운반: 용량과 에너지 효율을 분리

운반 witness는 matrix/pore/공개 매체가 있는 parcel을 제작하고, 실제 재고 입고→LINK grip→DRIVE→UNLINK→출고한다. 각 이동마다 64절의 결합 경로와 모든 cargo 질량을 검사한다. 최대 용량이 커진 것과 같은 물량을 더 적은 q로 옮긴 것은 다른 주장이다. 어떤 조립체는 빈 귀환의 추가 질량 때문에 여러 번 직접 들고 가는 것보다 비쌀 수 있다.

같은 총 배송량·출발/도착 재고·거리·지형에 대해 직접 운반과 조립 운반의 완료 시각, Σq, 연료 소모, 실제 도착량, 제작 재료, 유지 기간을 대조한다. 중간에 G를 호흡했다면 그만큼 배송량에서 빠진다. 운반한 G를 배송 실적으로 세고 동시에 연료로 세지 않는다. 회전이나 Push로 grip만 우회해 결합 용량 검사를 건너뛸 수 없다.

### 열 가공: 태양과 외기까지 포함

57절의 S61/X2/G1,c255,초기 M0 조건은 Rx4의 정산 fixture다. 완전 witness에서는 credit0에서 시작하고 실제 원료 배치·압축·가열·회수 과정을 포함한다. P1,X1,가능 extent1이면 고온 due당 Rx4 credit16이므로 4096회의 유효 due가 필요하다. 모든 due가 유효하더라도 첫 credit 갱신부터 마지막까지 65520tick이며, 임의 시작 위상을 포함해 65536tick 규모다. 중간 냉각이 있으면 더 길어진다.

밤에 θ900,네 이웃 θ280,해당 edge의 κ1을 유지한다고 가정하면 외기 손실은 4960q/due=평균 310q/tick, 네 edge 전도는평균 310q/tick으로 합 620q/tick이다. 이것은 16/8tick 일정의 조건부 평균이며 실제 한 tick 비용이 620으로 고정된다는 뜻이 아니다. 낮에는 실제 solar_in이 가열에 기여하므로 그 에너지를 빼놓고 불가능하다고 단정하지 않는다. 반대로 가열 직후 θ900만 보아 장기 생산이 가능하다고 하지 않는다.

생산 비용에는 초기 U의 외부 투입, 실제 solar_in, Heat 작업, 연료·산소 수송, 마찰, 냉각, 부산물, 금속의 재산화, 회수에 필요한 Abrade를 모두 기록한다. 금속 M은 현재 열전도·cohesion·수동 signal 전도에만 물성 차이를 준다. 만들어졌다는 이유로 발전기·모터·도구 능력치를 해금하지 않는다.

### 일주기 가용 에너지의 상한

한 위치에 머무르고 body G16,X256,항상 광반응 온도 적합, 모든 원료·출력 공간 충분이라는 낙관 조건을 둔다.4096tick의 모든 due에서 absorbed=L-floor(L/4)를 합산하면 Lpeak32768에서 1572864q, Lpeak65535에서 3145728q다.16개 due 위상 모두 동일한 합이다. 광 credit 누출·cap·종료 시 잔량을 무시한 장기 호흡 에너지 상한은 absorbed×1666/(10000×4096), 즉 각각 63.9744와 127.9488q/tick이다.

초기 G·초기 에너지·외부 G 수입은 이 자급 상한에 포함하지 않는다. 태양의 화학 저장과 이후 호흡을 두 번 계산한 값도 아니다. 실제 값은 누출·밤·산소·온도·포화·자식에 넘긴 물질 때문에 더 낮을 수 있다. 이동해 빛을 추적하거나 다른 개체의 생산물을 얻는 전략은 별도 비용·물질 수지로 평가한다.

inherited6페이지와 work1페이지의 유지 39q에 32명령×4=128q를 더하면 167q/tick이다. 감각·외부 행동이 없어도 위 최대 자급 상한보다 크다. 따라서 instruction32는 지출 가능한 상한이지 매 tick 항상 쓸 수 있는 무료 계산량이 아니다. 저광량 위치에서 평균 대기 계산 16q와 유지 39q를 빼면 낙관적 여유도 8.9744q/tick뿐이다. 복제·기록·보관 투자의 회수 시간을 이 여유와 비교해야 한다.

이는 프로그램·행동의 실패 원인을 분리하기 위한 진단이다. 조상의 일주기 생존, 탄소·물·산소 순환, 야간 재고, 후손 투자, 고정 조상 대비 실제 변이 후 생존을 차례로 확인한다. 실패를 덮으려고 계산 보너스나 희귀 전략 지원금을 넣지 않는다. 자원 흐름을 통한 분업의 이익이 나타날 가능성은 열려 있지만 이 계산만으로 분업의 출현을 증명하지 않는다.

**경계:** W는 기존 비용·물질·에너지 수지다. P는 일반 명령으로 이어진 절차다. E는 비용을 회수하는 실제 사용과 생존 전략이다. U는 단순 제작·기능·효율·자연 발견을 구별한다.

## 66 정보의 조합: 감각 오류·중계·모듈 호환성

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

### 오류 상태를 실제 프로그램에서 읽을 수 있어야 한다

R4는 성공한 산술 명령이 VmFault를 Ok로 덮도록 했다. 그런데 READ Self.fault를 준비하려면 주소 register를 CONST로 바꿔야 해서 원래 오류를 읽기 전에 지울 수 있었다. R5에서 VmFault는 **마지막 오류를 유지하는 latch**다. 오류가 나면 최신 nonzero fault로 바꾸고, 보통 성공 명령은 그대로 둔다. READ Self.fault만 직전 값을 결과 register에 돌려주고 fault를 Ok로 지운다. 이 READ 자체의 인자가 무효면 BadAddress를 기록한다.

에너지 부족으로 다음 instruction 비용 4q를 낼 수 없으면 EnergyStop을 latch하고 pc를 유지한다. intent의 energy 실패는 여전히 별도 Outcome이고 r14/r15로 전달된다. 새 latch는 이미 존재한 VmState.fault의 의미 변경이며 추가 감각 권한이 아니다. 형식이 깨진 opcode·query 초과·차폐 실패를 host가 유용한 행동으로 복구하지 않는다.

Founder-4의 탐색은 Ok 뒤 재고를 확인하거나 Boundary/Occluded로 그 위치가 사용 불가임을 확인했을 때만 탐색 index를 진행한다. 그 전에 이전 fault를 명시적으로 읽어 지우고, READ 뒤 준비 연산을 거쳐 다시 fault를 읽을 수 있다. QueryLimit이면 현재 탐색 index를 유지한 채 다음 tick에 이어간다. 효과 receipt는 대기 잔여 검사보다 먼저 한 번만 처리한다. 대기 중이라고 COPY actual을 잃거나 같은 receipt를 두 번 더하지 않는다.

### 능동 중계의 최소 경로

금속 전도 없는 세계의 같은 행에 송신 actor A x0,중계 B x3,C x6,수신 D x9를 둔다. A가 tick t에 channel0,a200을 EMIT하면 W(t+1)의 x1 값은 100이다. B는 반경 2의 x1을 READ하고 산술로 두 배를 만든 뒤 a200을 EMIT한다. C는 다음 tick에 x4 값 100을 같은 방식으로 처리하며 D는 W(t+3)의 x7 값 100을 읽을 수 있다. 각 actor가 필요한 명령을 해당 Think budget 안에서 끝냈을 때의 최소 지연이다. 실제 VM이 더 오래 걸리면 실제 tick을 기록한다.

A/B/C의 방출만 3×(128+2×200)=1584q이며 READ·산술·유지비는 추가다. 수신 D는 A의 모든 직접 kernel 셀을 감각 반경 안에서 읽을 수 없으므로 직접 관측을 중계로 오인하지 않는다. B 또는 C의 해당 방출을 제거한 대조에서 D의 pulse가 사라지는지 검사한다. 이 고정된 방향·증폭 절차는 Fixture용이며 Founder에는 넣지 않는다.

packet 길이, 시작 기호, 재전송, 발신자, 수신자, 시간 기준과 channel 의미는 제공하지 않는다. 기록·기억·신호열로 그런 관례가 생길 가능성을 제공할 뿐이다. signal의 진폭 증가는 에너지 생성이 아니며 실제 sender의 계산·방출 비용으로 지원된다. 잡음을 없애 주는 host decoding도 없다.

### 매체를 복사한 다음 실행할 수 있는 조건

Commit의 entry_tag는 승인된 draft header의 entry_tag다. 새 binding을 stage11에서 채택해도 현재 pc와 기존 frame은 그대로다. 다음 CALL은 새 mapping을 사용하고, 빈 stack RET 또는 규정된 entry 복귀가 발생해야 새 entry로 간다. 새 모듈이 있다는 이유로 현재 루프를 중단하거나 즉시 실행하지 않는다. 그러므로 RET가 전혀 없는 기존 프로그램에서는 채택만 하고 실행하지 않을 수 있다.

CALL의 tag는 임의 16bit 값이다. tag가 같다고 두 집단의 procedure 의미가 같다는 뜻은 아니다. register·작업 기억·입출력 단위·효과 receipt 사용 관례도 프로그램의 일부다. engine은 같은 기능을 추론해 tag를 맞추거나, 필요한 다른 module을 찾아 설치하거나, 호출 인자를 자동 변환하지 않는다. 원치 않는 entry 교체·tag 충돌·유효하지만 무익한 코드도 가능하며 실제 비용·실행 결과로 선택된다.

검증 cache의 identity는 (media_uid, revision, used_cells, genome_format)이다. 같은 revision이라도 used_cells가 다르면 다른 검증이며 2q/cell을 다시 낸다. cache는 bytes 형식 결과만 재사용한다. 현재 binding·slot·retained 상한, 실제 owner와 source pin은 Commit마다 다시 검사한다. cache hit가 새로운 채택을 무조건 승인하는 권한이 아니다.

128cell의 유효 prefix를 공개 매체에 발행하는 작업·시도비 하한은 Resize192+32회 COPY×192=6336q다. B가 자기 draft로 받아 채택하는 하한은 Resize192+COPY6144+외부 query256+형식 검사 256+Commit 시도 64=6912q다. 합 13248q, 매체 G2이며 publisher가 쓴 지지 구조·실제 VM·유지비는 별도다. B의 inherited와 learned는 각각 물리 매체로 계속 비용을 낸다. 지식을 받았다는 이유로 원래 code 페이지를 무료 대체하지 않는다.

### 전수·조합·소실 witness의 종료 조건

| 단계 | 실제 primitive 과정 | 통과 조건 |
| --- | --- | --- |
| A의 공개 기록 | 자신의 procedure bytes를 물리 tape에 COPY | 유효 prefix·유료 페이지·CopySpan 확인 |
| 비친자 B의 획득 | READ/COPY→OwnDraft→Commit→CALL/entry | 실제 출처와 실행을 연결;단순 보유는 제외 |
| B의 재조합 | 복사한 module과 자기 module, 새 wrapper의 bytes·descriptor 작성 | header/구간/호출 tag가 실제 작성으로 유효;host 의미 수선 0 |
| C로 전수 | B가 재조합된 bytes를 공개하고 비친자 C가 획득 | 두 출처와 wrapper 작성 사실, 실제 실행 |
| 원 창작자 제거 | 별도 실험 branch에서 A/B와 그 기록의 제거를 기록 | C의 매체·자원·프로그램만으로 같은 기능 재현 |
| 기록·의존물 상실 | 사용 가능한 마지막 기록 또는 필요한 실제 공급을 별도 제거 | 재현 실패와 검색 범위를 보고;archive 복구 0 |

조합 확인용 예시는 앞의 G를 읽어 register에 두는 procedure와, 그 값을 사용해 실제 G를 이전하는 procedure를 두 tag로 나누고 wrapper가 순서대로 CALL하는 것이다. descriptor 작성·경계·register 관례·RET와 모든 비용을 실제 VM에서 검증한다. 이 예시 자체는 조상에도 주어진 확보 행동과 관계되므로 새로운 자연 기술로 계수하지 않는다. 의미 있는 기술 누적 주장은 별도의 자연 실행에서 새로운 기능·재사용·비친자 전파가 확인된 경우에만 한다.

**경계:** W는 fault latch·비용·binding·물리 기록의 수명이다. P는 기존 READ/COPY/WRITE/CALL/RET/EMIT다. E는 오류 처리·통신 관례·호환성·전수와 재조합이다. U는 작성·복사·사용·기능 증거를 따로 표시한다.

## 67 R5 구현 변경 목록·추가 기대값·동결 조건

**적용 범위: Grid-5 기준·회귀 명세다. 현재 제품의 고정 신체/물리 규칙이 아니다. Matter-1의 교체는76절, R7의 교체는81..94절, R8 계약은95..107절, R9 교체는108..120절, 최신 R10 교체는121..138절을 따른다.**

R5의 PhysicsSpecId는 civ-grid-5다. Genome format2,WorldGen-3,Founder-4의 목적, Observer A1.4를 유지한다. Founder의 receipt·fault·탐색 세부 의미는 이 개정의 명세대로 조립한다. 저장 컨테이너는 CIVSIM5/major5/minor0이며 이전 physics·fault 의미의 checkpoint를 조용히 새 규칙으로 이어 실행하지 않는다. 변환이 필요하면 원본을 보존한 명시적 Intervention branch다.

규범 rule_text의 절 집합은 05..22,31..35,39..44,52..57,59,63..66이다. 기능 fixture와 조상의 절도 포함하여 실행 의미와 시작 조건의 해석을 함께 고정한다. 이 전체 문서의 hash가 정확한 executable을 대신하지는 않는다. 원본 조상 bytes/hash, 검증 trace, 빌드 정보를 별도로 보존한다.

### Rust 계약에 반영할 구체적 변경

| 소유 crate·module | 추가·변경할 자료형 또는 system | 소유권·성능 경계 |
| --- | --- | --- |
| civ_core::transport | exchange_pores, quantize_flux, natural_edges | stage7 coordinator가 tile/parcel을 분할;1 parcel/1 tile로 교환 병렬 |
| civ_core::chemistry | ReactionSiteKind::PoreBoundary=4,site_rates | FieldStore의 sparse credit;물질 delta는 같은 tile transaction |
| civ_core::motion | BondLoadGraph, check_load_paths | worker별 고정 scratch, authoritative flow 없음 |
| civ_core::rng | FluxRngKey, FluxDomain8/9/10 | seed·domain·selector cache는 derived;tick/object가 counter |
| civ_vm::execute | VmFault latch/ack | VmState.fault 저장 의미 수정, 새 opcode 없음 |
| civ_vm::bindings | entry 선택, ValidationKey | cache에 used_cells와 format 포함;상한은 채택마다 검사 |
| civ_storage::schema | CIVSIM5,ReactionCredit site4 | header와 manifest의 major5;키를보존하여 resume |
| civ_cli::headless::fixtures | storage, load, relay, copy_chain, budget | 손작성 controller는 Fixture에만 탑재 |
| civ_observer::evidence | Reachability/Observed/Function 구분 | 이 상태를 world READ·번식 보상에노출하지 않음 |

BondLoadGraph는 nodes≤16의 ParcelUID·demand 배열과 bond capacity 목록, source_capacity를 갖는 scratch다. world pointer를 장기 보유하지 않는다. tree인 경우 subtree 합, cycle인 경우 최대 18 node residual matrix를 쓰며 18×18×8=2592byte의 u64 용량 공간과 BFS 배열을 worker별로 예약하면 된다. 중간 u128 검사 후 source 용량 이하의 판정에 필요한 값만 유한 용량으로 넣는다. 자료형의 실제 padding·전체 scratch size는 첫 build에서 측정한다.

PoreBoundary credit의 순서는 Rx2,Rx5,Rx6이고 다른 reaction key는 거절한다. wire는 기존 ReactionCredit의 (kind, owner, reaction, credit)이며 kind4의 owner는 ParcelUID다. sparse 기록을 밀집 sidecar로 최적화해도 같은 checkpoint를 쓴다. 최대 payload 증가는 1M parcel에서 6MB, 약 5.72MiB이고 인덱스 비용은 별도다. 새 반올림을 위해 수천만 edge에 영속 RNG state를 할당하지 않는다.

### 추가 기대 사례

T01..T97은 모두 미래 구현의 expected behavior이며 이 문서를 쓰면서 simulator test를 실행했다는 뜻이 아니다. r을 지정한 수치 사례는 quantizer에 대한 입력이며 정상 실행에서 agent가 RNG 출력을 지정할 수 있다는 뜻도 아니다.

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| T73 |edge 기체 차 1,P256,D2048,r255 또는 256 |후보 각각 1/0;실제 이동은재고·열·용량도제한 |
| T74 |S1,c0,K64,pore O0,loose O8,여유충분 |due pore교환 O1,같은 tile U/C/Ug불변 |
| T75 |T74 형태에 pore G64,외부 O1,θ280,Boundary Rx2 credit65535 |기체입고 0이어도표면반응 G63/W1,loose C+1/O-1,U+5000 |
| T76 |s64,c224,pore G8에서압축 1 |K8→7이므로 Capacity, 재고·c불변, 시도비만 |
| T77 |Matrix rate1,X1,extent1,D256,r0 또는 1 |credit증가 1/0;floor만써항상 0으로만들지않음 |
| T78 |기체차 1,P256에서모든 r0..2047열거 |후보 1이 256개, 후보 0이 1792개;물질복제 0 |
| T79 |root demand0,leaf80,grip10,bond4→5 |각각 load실패/성공;두경우 source160 |
| T80 |두경로잔여 48/32,끝 demand80 |함께있으면성공, 둘중하나제거하면실패 |
| T81 |Push root b4,total demand65 |source64라 Blocked, grip유무우회 0 |
| T82 |grip parcel회전축, 그 parcel demand0,actor에이동비발생 |actor비용은납부, 그몸체하중을 grip에중복요구 0 |
| T83 |actor접촉 1로동시도달못하는두 blocking endpoint LINK |접촉검사실패, 자동내부결합 0 |
| T84 |μ256,surface c128,이동 mass960 |이동소산 512→448,작업·시도 8256의단순회수 129회 |
| T85 |Lpeak32768,4096tick, 모든 16due위상 |흡수합 1572864,낙관자급상한 63.9744q/tick |
| T86 |Lpeak65535,6code page, 32명령매 tick |흡수상한 127.9488<지출 167;자급지속을선언하지않음 |
| T87 |H0초기, 외부물질개입 0,허용반응전체 |모든완결 tick의총 H0,Rx3/Rx5횟수 0 |
| T88 |동일 media/revision, used_cells변경후검증 |새 key, 새 2q/cell지불, 옛결과자동승인 0 |
| T89 |BadAddress뒤 CONST로 Self.fault주소준비, READ |원래 BadAddress반환후 fault Ok;준비중소실 0 |
| T90 |oldframe유지중 entry7인 learned채택 |즉시 pc변경 0,빈 stack RET부터 entry7 |
| T91 |복사 module의 CALL tag누락또는 register관례불일치 |MissingTag또는실제실행결과, host호환수선 0 |
| T92 |x0/3/6/9,각 Think내완료하는 pulse중계 |D는 W(t+3)에 100;중간방출제거시 pulse0 |
| T93 |128cell공개발행과비친자획득·Commit |열비용하한 6336+6912=13248q, 새페이지 G2 |
| T94 |형식검사 cache hit지만 retained slot상한초과 |Commit Limits, 기존 binding·entry유지 |
| T95 |손작성중계·전수 witness의성공 |Fixture기능증거, 자연발명·문명진행보상 0 |
| T96 |Boundary credit·fault latch가있는 checkpoint와동일 seed |resume/worker1·8의 flux·반응·CoreHash일치 |
| T97 |지도 모서리의 Pself1,두 유효 이웃 P1,바깥 P0 |X=max(1,floor(3/5))=1,외기·반응의 무료 영구 차단0 |

수치 검증은 실제 VM의 한계와 중간 비용을 포함하는 통합 witness로 이어져야 한다. 예를 들어 T75의 site 반응 성공이 장기간 보관 효율을 증명하지 않으며, T93의 복사 비용 계산이 호환된 실행을 증명하지 않는다. 불리한 결과도 기록하고, 같은 세계의 고수준 이름으로 통과 판정을 바꾸지 않는다.

### 다음 구현에서 중단 없이 진행할 순서

1. 원소·에너지 oracle와 국소 수송·반응을 구현하고 T73..T78을 기존 보존 사례와 함께 확인한다.
2. primitive serial resolver와 약한 결합·점유·이동의 실패 경계를 구현한다.
3. VM latch·binding·매체·복제·Founder 실제 bytes를 만들고 일주기 유지 비용을 측정한다.
4. 기본 지도에서 실제 세계를 관찰하면서 보관·운반·중계·전수의 witness를 실행한다.
5. 저장·이어하기·thread 동등성과 큰 world 성능을 확인한다.
6. 미개입 여러 seed에서 생존·전략 차이·자연 발견·전파를 판정한다.

현재 단계에서 완료한 것은 명세의 오류 수정, 정수 회계 대조, 기대값과 구현 경계의 정리다. simulator·조상 바이너리·물리 witness·UI·성능·장기 진화 실험은 아직 없다. 설계가 길어졌다는 이유로 검증된 최종 결과라고 부르지 않는다. 문서에서 닫을 수 있는 구현 의미와 실행으로만 판단할 수 있는 연구 가설을 계속 구분한다.

**경계:** W/P의 변경은 명시된 physics revision이다. Engineering Contract는소유권·phase·저장·검증을고정한다. E는실제발견만을뜻하고, U는미실행·실패·성공과증거범위를표시한다.

## 68 Matter-1 기반: 기술 공간을 넓히는 물질 세계

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

**최종 추천은 형상을 가진 물질, 접촉과 관성, 열역학적 압력, 전하와 전도, 표면 반응을 하나의 에너지 회계로 연결하는 Matter-1 물리 계층이다.** Wheel, Gear, Engine, Generator, Switch, LogicGate라는 제작 대상이나 agent 명령을 추가하지 않는다. 그런 이름은 실제 구조와 입출력 관계를 확인한 관찰기에서만 사용한다. 물리 해법 내부의 Contact, Constraint, ThermalCell, ChargeSite 같은 자료형은 허용한다. 이것들은 기능이 정해진 완성 장치가 아니다.

R5의 문제는 primitive 수가 적다는 사실만이 아니다. 움직이는 물체에 지속 운동량이 없고, 결합이 모든 자유도를 잠그며, 열이 압력 일을 하지 않고, 물질의 전기적 상태가 없다. 이 경계에서는 프로그램이 아무리 복잡해져도 물리적 기계의 일부를 표현할 수 없다. 반대로 물성 값을 많이 추가해도 힘·에너지·정보의 경로가 서로 연결되지 않으면 기술 공간은 거의 늘지 않는다.

Matter-1은 R6에서 선택한 물리 기반이다. 기존 civ-grid-5와 **civ-matter-1-draft**는 기준 모델로 보존하고, 현재 최종 목표는81절의 civ-matterlife-1-draft다. draft는 완성된 실행 형식이 아니다. 이 문서에서 설계 선택·상태 소유권·물리적 계약은 정하지만, 실제 물성 데이터의 균형·수치 해법의 안정성·성능은 실행으로 승인해야 한다. 실험 결과 없이 draft 접미사를 제거하지 않는다.

현재 최종 제품 목표는 **v2-MatterLife**이며 이 절의 Matter-1을 기반으로 한다. v0/v1/Grid-5는 개발·회귀 기준이며 사용자가 요청한 확장 세계의 완제품으로 대신 납품하지 않는다. 물리 기능은 세계 시작 시 함께 존재한다. tick, 세대, 문명 등급에 따라 법칙을 해금하지 않는다. 개발 중 서로 다른 profile에서 관찰한 발견을 하나의 연속된 자연사로 편집하지 않는다.

### 세 가지 확장안 비교

| 방식 | 장점·개방성 | 단점·성능·복잡도 | 결정 |
| --- | --- | --- | --- |
| 기존 격자에 축·비율·전력 포트 추가 | 저렴하고 기계 예시를 빨리 만들 수 있음 | gear ratio나 변환 효율을 부품 속성으로 주면 장치 기능을 개발자가 제공하게 됨; 형상 발명 공간이 작음 | 제외 |
| 전 세계 분자·전자·연속 유체 해석 | 매우 넓은 물리 표현 가능성 | 요구 규모와 양립하기 어렵고 생명·재료 모형까지 새 연구가 됨; 실제 무한성도 보장하지 못함 | 제외 |
| 조형 가능한 물질과 희소 다중 물리 해법 | 재료 배치·형상·국소 상호작용에서 기능을 만들 수 있음; 기존 VM·관찰 구조 유지 | 운동·유체·전기 사이 수치 결합이 어렵고 Grid-5보다 상당히 비쌈 | **선택** |

최소라는 말은 재료의 이름이 몇 개뿐이라는 뜻으로 제한하지 않는다. 같은 규칙이 여러 용도로 재사용되고, 새 기술을 위해 별도 예외를 넣지 않는 것이 기준이다. 열팽창 하나가 압력 조절·온도 반응·형상 변화에 모두 쓰일 수 있어야 한다. 특정 발명을 쉽게 만들기 위해 그 조합만 비용을 줄이지 않는다.

유한 해상도·유한 자원·제한된 연산량의 세계에는 여전히 표현 한계가 있다. 물리 확장의 목표는 **구체적인 표현 차단을 제거할 설계**이며, 끝없는 발명이나 현실의 모든 기술을 보장하는 것이 아니다. R7은 고정 생명체 조건을 철회하고83..85절의 형태·대사 진화를 추가한다.

**경계:** W는 물질의 상태와 상호작용이다. P는 국소 관측·배치·작업·힘·결합·기록이다. E는 기계·제어·생산 체계의 실제 형성이다. U는 기능과 증거를 바탕으로 붙이는 이름이다.

## 69 새 세계 법칙의 범위와 상호 연결

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

아래 M1은 Matter-1 물리 기반이고 ML1은 R7에서 추가한 필수 범위다. M2만 후속 연구로 남긴다. 최종 범위는81..94절을 함께 적용한다. M1의 수치 해법이 준비되지 않았다는 이유로 해당 기능을 장치 prefab으로 대체하지 않는다.

| 물리 계열·범위 | Hard-coded World Rule | Agent Primitive | Emergent Behavior / UI Interpretation |
| --- | --- | --- | --- |
| 형상·부피 M1 | 물질 위치, 실제 점유, 노출 면적 | 부분 위치에 물질 이전·제거 | 틈·톱니·벽·용기 / 구조 후보 |
| 병진·회전 M1 | 질량·관성·힘·토크·운동량 | 접촉점에 유한 힘 | 축·지렛대·회전체 / 기구 후보 |
| 접촉·마찰 M1 | 비관통, 법선력, 정지·미끄럼 마찰 | 누르기·밀기·방향 변경 | 견인·제동·맞물림 / 힘 경로 |
| 결합·탄성 M1 | 제약하는 자유도, 변형 에너지, 감쇠 | 국소 접합·절단 | 힌지·스프링·연결 구조 / 결합 graph |
| 응력·파괴 M1 | 강도·변형·균열·마모 | 실제 접촉 일·가열 | 날·분쇄·성형·붕괴 / 도구 후보 |
| 중력·지형 M1 | 공개된 높이장의 위치 에너지 | 힘·운반·퇴적 | 경사 이동·낙차 이용 / 지형 흐름 |
| 기체·압력 M1 | 상태방정식, 유동, 경계의 압력 일 | 공간을 만들고 물질·열을 가함 | 압력 저장·왕복 운동 / 압력 장치 후보 |
| 액체·수송 M1 | 부피·질량 수송, 점성, 압축성 근사 | 국소 배치·경계 변경 | 수로·유압·냉각 / 유체망 후보 |
| 열·팽창 M1 | 유한 열용량, 전도, 팽창, 열 손실 | 국소 작업·가열 | 온도 반응·열 교환·열기관 / 에너지 흐름 |
| 상변화 M1 | 압력과 온도에 따른 평형·잠열 | 동일한 물질·열 조작 | 응축·증발·냉각·열 수송 / 상 지도 |
| 화학 M1 | 원소·전하·화학 에너지, 반응 장벽 | 반응물 접촉·분리·가열 | 연소·환원·부식·물질 순환 / 반응망 |
| 혼합·표면 M1 | 접촉 면적, 확산, 혼합비, 표면 상태 | 배치·마모·압축 | 복합 재료·표면 가공 / 재료 family |
| 전기 M1 | 전하 보존, 전도, 절연, 정전용량 | 도체·절연체 배치와 기계적 일 | 충전·방전·배선·접점 / 전기망 |
| 전기·운동 결합 M1 | 같은 전기장 에너지에서 힘과 역작용 계산 | 기존 힘·형상 조작 | 정전 구동·발전·접점 제어 / 변환 장치 후보 |
| 물리적 기억 M1 | 변형·전하·위치·조성의 실제 수명 | 기존 배치·힘·전하를 바꾸는 물리 과정 | 래치·진동·계수·논리 / 계산 장치 후보 |
| 빛·복사 M1 | 태양 유입, 실제 차폐, 흡수와 열 방출 | 표면·형상·배치 | 열 수집·보온·그늘 / 복사 지도 |
| 표면장력·젖음 M2 | 계면 에너지와 접촉각 | 기존 물질 조작 | 모세관 수송·액적 제어 / 유체 미세구조 |
| 자기·유도 ML1 필수 | 자속·자화·역기전력·역작용 | 도체·자성 재료 배치 | 유도 발전·자기 기억 / 장치 후보 |
| 기하 광학 ML1 필수 | 굴절·반사·스펙트럼·흡수·방출 | 표면·투명 재료 배치 | 집광·광 신호 / 광학 장치 후보 |
| 구조 화학 ML1 필수 | 가변 molecular graph·촉매·고분자 | 물질·일·발현 | 새로운 재료·대사 / 화학 family |
| 형태 발생 ML1 필수 | compartment·합성·분할·국소 수송 | 발현·복사 투자·조직 활성 | 기관 유사 기능·체형 / 형태 family |
| 생태 순환 ML1 필수 | 실제 흡수·성장·섭취·분해 | 기존 힘·물질·발현 | 먹이망·공생·재배 / 동식물·자원 해석 |
| 지질·고도층 M2 | 유한 지층, 침식·응력·다층 접촉 | 굴착·퇴적·하중 | 입체 구조·지하 공간 / 지층·구조 해석 |

기체 흐름·전류·접촉 운동은 서로 다른 저장량이다. Matter-1의 유료 축약 EMIT는 ML1에서89절의 실제 빛·용질 방출로 교체한다. 물리적 배선의 신호 전달은 전기 상태를 실제로 변화시켜야 하고, EMIT payload로 기계가 직접 작동하지 않는다.

Matter-1의 정전기·저항·정전용량에 ML1의 자속·자화·유도 경로를 추가한다.86절은 역작용까지 포함한 준정적 모델이며 무선 전파를 포함하지 않는다. 정전 발전과 유도 발전은 각각 별도의 제작·작동 witness를 통과해야 한다.

각 물리 계열은 단독 사례, 두 계열의 결합, 세 계열 이상의 순환에서 검사한다. 예를 들어 열→압력→운동과 운동→전하→접점까지 각각 성공해야 열에서 출발하는 자동 제어의 후보 경로가 열린다. 두 표에 기능명이 함께 있다는 이유로 그 경로가 구현되었다고 판정하지 않는다.

**경계:** 표의 W/P가 구현 대상이다. E/U 열의 단어는 성공 목표와 해석 예시이며 레시피·enum·점수로 Core에 들어가지 않는다.

## 70 공간·물질·형상의 재설계

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

### 조밀한 지도와 희소한 물체

지도는 64×64 chunk의 조밀 배열을 유지한다. 기하학적 위치는 tile 내부를 표현할 수 있게 바뀐다. 구조의 기본 제작 해상도는 **tile당 8×8 material site**다. 이것은 화면 픽셀이 아니라 충돌·물질·전기 접촉의 해상도다. 단단한 물질 한 단위가 site 하나를 차지하며, G/X/M/S/I가 들어갈 수 있다. 기체·액체는 열린 부피의 유체 저장소에 속한다. 물질이 없는데 shape bit만 채우는 것은 금지한다.

기존 matrix 재고와 site 재고를 동시에 authoritative하게 보유하지 않는다. ShapeRecord의 물질 site가 원본이고 합계·질량·질량 중심·관성·노출 면적은 derived다. 균일하고 정지한 bulk 형상은 압축 표현할 수 있지만 물리적으로 같은 질량·경계·접촉 결과를 만들어야 한다. 사이트는 생명체의 세포나 진화하는 기관이 아니다.

물체의 pose=(x,y,angle), momentum=(px,py,Lz)를 저장한다. 형상은 물체의 지역 좌표에 붙는다. 결합된 여러 조각은 크기와 무관하게 하나의 기술 객체가 되지 않는다. 영구 용접 성분을 하나의 rigid body로 합치는 최적화는 질량·관성·외부 접촉·내부 파괴 상태를 보존할 때만 허용한다. 기본 reference backend는 조각과 결합을 그대로 푼다.

8×8은 무한한 형상 표현이 아니다. 여러 조각을 이어 큰 톱니나 긴 레버를 만들 수 있지만 1/8tile보다 작은 날과 간극은 표현되지 않는다. 면 밖의 기준 두께는1, tile 체적은1, 가득 찬 site의 체적은1/64 모델 단위다. 이 부피와 기존 inventory의 추상 volume64를 같은 단위로 혼용하지 않는다. 제작 해상도를 바꾸는 것은 새 PhysicsSpec이며 실행 도중 성능 설정으로 바꾸지 않는다.

유체 재고는 hierarchy의 leaf cell에만 존재한다. coarse parent의 species와 그 fine 자식의 species를 둘 다 재고로 세지 않는다. body의 물질과 해당 위치 유체 volume도 서로 다른 공간을 점유한다. body가 통과하는 cell의 geometric fluid fraction을 갱신하면서 이전 fluid 재고를 무상 유지·삭제·복사하지 않는다.

### 좌표·점유·접근

단일 tile의 parcel_slot/actor_slot 대신 그 tile과 겹치는 BodyKey의 정렬 bucket을 둔다. 실제 비관통 판정은 형상을 쓴다. 같은 tile에 여러 작은 조각이나 개체가 있을 수 있다. 기존 I03의 한 actor/tile은 Matter-1에서 **물체 내부의 허용되지 않은 겹침 금지**로 바뀐다. 관찰용 tile population은 여전히 계산할 수 있다.

ML1 agent의 신체·질량·대사는83..85절의 실제 조직·발생·반응 상태에 따라 바뀐다. 위치와 외부 운동량은 다른 물체와 같은 기하 공간에 있다. VM의 orientation0..3은 감각·행동 좌표의 방향이며 물리적 몸체 angle과 구분한다. 시선만 돌려 매달린 조립체를 회전시키지 않는다. READ는 상대 tile 영역에서 거리, BodyUID, site index 순으로 국소 대상을 고르고, 인접한 표면의 접근 가능한 site를 지정한다. 보이는 내부 구조 전체를 원격 편집할 수 없다.

Matter-1 reference의 sensor 거리2tile·작업 거리1tile는 ML1에서 조직의 실제 감각·작업 표면과89절의 국소 주소 계약으로 재승인한다. 쿼리 한도와 유료성은 유지한다. 물리적 접촉점은 이 거리 안이면서 가려지지 않은 표면이어야 한다. ray가 다른 고체에 먼저 막히면 Occluded다. 같은 국소 위치를 읽었어도 움직여서 접촉점이 사라지면 resolver는 Stale/Blocked를 반환한다.

### 물질 이전·출생·사망도 운동량을 보존한다

TRANSFER·절단·결합·화학 반응이 질량과 관성을 바꾸므로 pose만 추가하고 기존 반환 코드를 그대로 쓸 수 없다. 분리되는 site의 속도는 원래 body의 v+ω×r에서 얻는다. 옮기는 질량·운동량·열·전하를 하나의 packet으로 만든다. 목적지의 질량 중심과 관성을 다시 계산하고 총 선·각운동량을 보존한다. 비탄성 합류에서 줄어든 운동에너지는 열로 간다. 강제로 다른 속도나 위치에 놓기 위해 필요한 양의 일은 작업 예산에서 낸다. inventory 안에 물질을 넣거나 꺼내는 행위로 운동에너지를 복제하지 않는다.

배아 Release 때도 부모 내부에서 함께 움직이던 배아의 질량·운동량을 분리한다. 자식의 속도를 임의로0으로 만들거나 부모의 기존 총운동량을 그대로 두지 않는다. 위치를 벌리는 작업·ΔUg·충돌을 함께 검사한다. 사망은 VM·대사의 종료이며 운동량 소거가 아니다. 몸체 물질을 site나 유체로 반환할 때도 packet을 사용하고, 붕괴·혼합 손실만 열로 바꾼다. 수동 물체를 잡는 순간의 충격과 놓는 순간의 상태도 같은 경로로 처리한다.

### 고도와 표현 한계

M1은 지면 위 2D 운동과 기존 높이장 z(x,y)의 위치 에너지를 사용한다. Ug=Σm g z, g=1 모델 단위다. 높이장은 tile 중심 값을 연속적으로 보간하고 물질 이동 때 같은 함수로 ΔUg를 정산한다. 완전한 3D 비행·다층 건물·교량 밑 통과·수직 터빈을 지원한다고 표시하지 않는다. 평면 회전축은 화면에 수직인 축이며 현실의 모든 축 방향을 대신하지 않는다.

지면은 기반 질량·열 저장소로 모델링하지만 무한 채굴 대상이 아니다. M1의 환경 수정은 실제 유한 재고의 이동·퇴적·압축과 접근성 변화다. 지층을 파서 재료를 얻는 확장은 먼저 bedrock의 유한 물질·부피·Ug를 저장해야 하며, elevation 숫자만 내리고 광석을 생성하는 방식은 금지한다.

**경계:** W는 형상·접촉·질량·고도다. P는 국소 site의 물질 이전과 작업이다. E는 형상에 의존하는 기능이다. U는 구조의 이름·가시화·단면 표시다.

## 71 공통 에너지·전하·물질 회계

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

아래는 Matter-1 기반 식이다. ML1은88절에서 Eagent/Eembryo/Ephoto를 새 상태로 교체하고 자기·광을 포함한다. Grid-5의 에너지 회계에 운동·탄성·압축·전기장 저장량이 추가된다. **Etotal=Echemical+Ethermal+Eagent+Eembryo+Ephoto+Ug+Klinear+Kangular+Eelastic+Efluid_motion+Ecompression+Eelectric+Esurface.** 같은 물질의 열용량, 화학 h, 질량을 body와 grid에 중복 계산하지 않는다. 각 에너지 항목에는 소유 객체가 정확히 하나 있어야 한다.

Klinear=p²/(2m), Kangular=Lz²/(2I)다. 지면과 외기·빛은 명시된 외부 저장소이며 교환량을 ledger에 기록한다. 마찰·저항·점성·소성 변형으로 소산한 일은 열로 간다. HeatProduced 통계를 Etotal에 더하지 않는 기존 규칙은 유지한다.

힘과 속도의 곱, 토크와 각속도의 곱, 압력과 부피 유량의 곱, 전위차와 전류의 곱은 모두 에너지 전달률이다. 이 네 연결을 공통 TransferLedger에서 정산한다. 서로 다른 solver가 같은 경계 일을 각각 자기 에너지 증가로 기록하면 실패다. 내부 경계의 출금과 입금은 같은 transfer ID로 연결한다.

열을 일로 바꾸려면 온도 차, 화학적 비평형 또는 다른 소모되는 자유에너지가 있어야 한다. 에너지 합만 맞추고 균일한 열을 무한히 전력으로 바꾸는 모형은 허용하지 않는다. 폐쇄 순환의 엔트로피·가용 일 검사를 추가한다. 두 일정 온도 열원만으로 작동하는 주기 장치의 효율은 1-Tcold/Thot를 넘을 수 없다. 이 상한은 비주기 과정·연료 소비를 무시하고 모든 장치에 단순 적용하지 않는다. [OpenStax 열기관과 Carnot cycle](https://openstax.org/books/university-physics-volume-2/pages/4-5-the-carnot-cycle)

### 수치 표현의 선택

| 방식 | 장점 | 비용·위험 | 선택 |
| --- | --- | --- | --- |
| 전부 기존 정수 격자 | 원소·q 회계가 단순 | 작은 회전·압력·전하·제약을 충분히 표현하기 어려움 | Grid-5에 유지 |
| 모든 상태 f64 | 연속 해법의 개발이 쉬움 | 아주 작은 물질·전하 오차가 누적되고 회계와 실행 비용이 섞임 | 제외 |
| 정확한 양과 결정론적 연속 해법의 결합 | 원소·전하·VM 비용은 정확하게, 기하는 연속으로 표현 | 경계 반올림·잔여량·에너지 오차 검사가 필요 | **Matter-1 선택** |

유체의 종별 양은 기존 한 단위의 1/65,536을 최소량으로 하는 정수다. 일반 환경 저장은 u64 Amount, inventory 입출력 ABI는 정수 단위와 포화 관측을 유지한다. 같은 flux의 정수량을 송신·수신에 정확히 적용하며 소수 잔여는 edge owner의 signed accumulator에 보존한다. 반응 extent도 이 최소량을 사용하므로 원소 수는 최소량 단위에서 정확하다. 전하는 signed i64 최소 전하량으로 보존하고 전자 이동과 이온 이동을 구분한다.

pose·momentum·압력 해법은 엄격한 f64 산술을 사용한다. fast-math·자동 FMA·비결정적 reduction·환경별 libm 호출을 금지한다. 초월함수·sqrt·sin/cos·exp/log는 고정된 portable math 구현과 golden vector를 갖는다. 이것은 임의 Rust build에서 비트 재현성을 보장한다는 말이 아니다. 승인된 compiler·target·kernel 조합만 replay-compatible으로 표시한다.

f64에서 에너지 보존은 정수 q의 대수적 정확성과 같지 않다. NumericalResidual을 별도로 기록하되 어떤 agent나 장치도 그 잔액을 쓰지 못한다. 오차를 열 저장소에서 몰래 빼거나 넣어서 0으로 만들지 않는다. 수치 오차가 큰 step은 되돌리고 더 작은 물리 substep으로 재시도한다. 전체 run의 양의 에너지 오차 예산은 기본 **1q**, 한 tick의 추가 양의 오차는 **10^-6q**를 승인 목표로 둔다. 이 기준은 아직 달성 수치가 아니며, gate 실패를 threshold 완화로 자동 해결하지 않는다. 음의 오차도 총량·위치·시간을 기록해 효율을 왜곡하는지 검사한다.

VM의 q와 연속 물리 저장 사이에는 정수 energy escrow와 소수 잔여가 있다. 출금된 q를 즉시 열로 바꾸지 않는다. 실제 양의 기계적 일에 사용된 부분과 소산 부분을 나누고, 사용하지 않은 부분은 원 소유자에게 돌려준다. 이미 지급한 추진 일을 다시 이동 비용으로 차감하지 않는다. agent 몸체는 회생 제동으로 energy를 충전하지 않으며 음의 근육 일은 열로 간다. 이는 Matter-1의 고정 actuator 기준이다. ML1은83·85절의 조직·carrier 변환 경로로 교체하며 재충전은 실제 반응·일 결합이 있을 때만 허용한다.

에너지와 엔트로피를 함께 다루는 구조적 접근은 [Lohmayer 등, Exergetic Port-Hamiltonian Systems](https://arxiv.org/abs/2008.04091)를 참고한다. 해당 이론을 인용했다는 이유로 본 구현이 자동으로 수동성·안정성을 증명받았다고 주장하지 않는다.

**경계:** W는 보존·소산·가용 에너지다. P는 유한 예산의 물질·일 전달이다. E는 에너지 획득·저장·변환 체계다. U는 실측 흐름과 오차·외부 입력을 함께 보여준다.

## 72 역학: 접촉·회전·탄성·파괴

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

### force가 효과의 원본이다

Matter-1의 DRIVE는 목적지로 옮기는 명령이 아니다. ML1에서는83·89절의 실제 자기 조직 활성 요청으로 연결한다. 한 tick 동안 접촉점 또는 몸체에 유한 힘을 가하는 request다. torque는 τ=(contact_position-center_of_mass)×F에서 생긴다. 직접 angle을 원하는 값으로 고정하는 명령, 연결물 전체를 순간 회전하는 명령, 목표 속도를 강제하는 motor joint를 agent에게 주지 않는다.

초기 actuation 후보 상한은 |F|≤64, 일 escrow≤1024q/tick이며 run 중 불변이다. 힘 방향은 orientation 기준4방향, 크기는 r5=0..255로 양자화한다. 실제 출력은 접촉과 예산에 의해 제한된다. 뻣뻣한 물체에 버티기만 해도 activation heat=ceil(8×r5/255)q/tick를 낸다. force=0은 idle이며 시도 비용만 적용한다. actuator가 같은 tick에 여러 대상에 force를 복제하지 못한다.

고정 actuator 상한은 자연 법칙이 아니라 생체의 공통 능력이다. 이동과 밀기·가열을 모두 수행하던 Grid-5 신체를 새로운 물리 공간에 접속하기 위한 제약이다. 이 수치에서 조상이 유지·이동·복제 가능한지는 새 Founder-5 witness로 다시 측정한다.

### 접촉과 마찰

겹칠 가능성이 있는 swept AABB를 broad phase에서 수집하고 (min BodyUID,max BodyUID,site pair)로 정렬한다. 한 쌍이 여러 chunk에서 발견되면 동일 contact key로 deduplicate한다. narrow phase는 material site로 만든 convex 경계와 continuous sweep을 사용한다. 빠른 물체가 한 substep 사이 벽을 지나가는 것을 종점 검사만으로 허용하지 않는다.

법선 접촉은 비관통, 접선 접촉은 Coulomb 상한 |Jt|≤μ Jn을 사용한다. 기본 reference는 비탄성 접촉 e=0이다. 반발은 저장된 탄성 에너지에서 나오며 단순히 e>1을 주지 않는다. 마찰 계수는 두 표면의 조성·압축·마모 상태에서 계산하고 역할 이름을 참조하지 않는다. 접촉 소산량은 충돌 전후 운동량·운동에너지로 구해 해당 접촉의 열로 이동한다. 지면에 전달된 운동량은 고정 기반의 반작용으로 기록한다.

칼날과 뭉툭한 면의 차이는 ToolType이 아니라 접촉 면적과 응력 집중이다. 압력이 작용한 면적을 최소 site 면적보다 작게 가정하여 무한 응력을 만들지 않는다. 접촉으로 재료가 손상될 때 빠진 조각과 운동량·열·전하도 실제로 남는다.

### 일반 결합

LINK는 두 노출 면의 국소 anchor를 붙이고 어떤 상대 자유도를 구속할지 선택한다. 허용 표현은 위치 두 축, 상대 각도, 그리고 유한 rest length다. 용접은 위치와 각도를 모두 묶고, 점 결합은 위치만 묶는다. 점 결합 주위의 회전은 solver 결과이며 HingeMachine을 배치하는 동작이 아니다. 거리 결합은 인장에만 작동하거나 인장·압축 모두에 작동하는 물성으로 구별한다. 결합 강성·감쇠·파단 한계는 실제 사용한 접합 재료의 양과 물성에서 유도한다.

결합은 공짜의 질량 없는 핀으로 생성하지 않는다. 최소 접합 물질1단위를 BondRecord에 귀속시키고 질량·열용량·h에 포함한다. 소비된 재료와 사용된 작업량을 기록한다. 결합 삭제·파단 때 그 물질을 해당 접촉 위치에 반환한다. rest 상태를 바꾸어 Eelastic이 증가하면 작업 예산을 먼저 요구하고, 감소분은 열 또는 물체 운동으로 정산한다. 잡기·놓기의 과정에서 이전 변형 에너지가 사라지지 않는다.

기본 탄성 에너지는 Eelastic=Σkδ²/2이다. 소성은 yield를 넘은 응력에서 rest 상태를 이동시키며 저장 에너지 감소분을 열로 바꾼다. 파괴는 강도·누적 손상 기준으로 발생한다. 매 tick 누적되는 damage는 strain history의 상태이며 단순 age에 따른 임의 붕괴로 대체하지 않는다. k=∞는 쓰지 않고 안정성 영역 내의 유한 강성으로 모델링한다.

지렛대의 힘 이득은 접촉점의 거리와 토크 평형에서, 기어 비율은 이빨의 실제 접촉 기하에서 나와야 한다. RatioConstraint, GearLink, AutoRotate를 agent가 생성할 수 없다. 작업 거리는 여전히1tile이다. 멀리 있는 물체가 움직이는 것은 연결된 물질을 통해 힘이 전달된 결과다.

### solver의 승인 범위

reference solver는 고정 순서 semi-implicit 적분과 순차 impulse 제약 풀이로 시작하되 접촉·탄성 에너지 오차를71절에서 독립 검사한다. 8 substep/tick, 각 substep 12회 constraint sweep을 기본 후보로 둔다. 부족하면 해당 island의 전체 tick을 이전 상태부터 16/32/64/128 substep으로 재계산한다. 실행 시간 때문에 중간 iteration에서 멈추지 않는다. 128에서도 비관통·수치 gate를 만족하지 못하면 NumericDomainError로 run을 일시 정지한다. 물체 삭제나 강성 자동 하향으로 숨기지 않는다.

같은 수치기법이 모든 강체·탄성 구조에서 충분하다는 증거는 아직 없다. 선·각운동량의 수치 residual도 외부 지면·유체·agent 반작용과 분리해 기록하고, energy gate와 함께 허용 오차를 동결해야 한다. 솔버 선택의 승인 gate에는 레버, 작은 이빨, 긴 유연 사슬, 조밀한 충돌, 압력으로 구동되는 경계가 포함된다. [Box2D의 공식 simulation 문서](https://box2d.org/documentation/md_simulation.html)는 contact·joint·고정 step의 구현 비교 기준으로 사용한다. Box2D의 기성 motor 기능을 이 세계의 primitive로 수입하지 않는다.

**경계:** W는 운동·접촉·마찰·변형·파괴다. P는 접촉점 힘·접합·절단이다. E는 도구·레버·회전 기구·반복 운동이다. U는 전달된 일과 기능을 근거로 장치 후보를 분류한다.

## 73 유체·열·상변화의 결합

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

Grid-5의 재고차 확산은 압력 기관의 근거가 될 수 없다. Matter-1은 부피·열·물질과 운동량을 가진 유체 cell을 사용한다. gas compartment를 Warehouse나 Cylinder로 분류하지 않는다. 빈 공간과 고체 경계가 체적과 흐름을 결정한다.

### 부피와 기체

기체 혼합의 기본식은 pV=nRT, Uthermal=Σn_i cvi T다. R=1 모델 단위로 시작하고 cv는 MaterialSpec에 둔다. 비어 있는 cell에서 n=0이면 gas pressure=0이며 T는 조회 불가 상태로 표시한다. 숫자 0K를 정상 기체 온도로 사용하지 않는다. 열·물질 유입 뒤에만 온도를 정의한다. parcel 압축도가 pore의 수용량만 줄이는 대신 실제 열린 체적·투과 면적을 바꾼다.

움직이는 벽에서 유체가 하는 일은 ∫p dV다. 같은 크기의 반대 일이 경계 body의 운동으로 들어간다. 닫힌 단열 기체에서 이 관계를 만족하지 못하면 piston 후보 시험은 실패다. 압축으로 온도가 오르고 팽창으로 식는 경로가 있어야 하며 WORK Heat를 호출한 경우에만 온도가 변하는 모델은 사용하지 않는다.

### 수송의 reference 해법

기체 reference는 species별 양, 운동량 두 성분, 총 유체 에너지를 보존 변수로 하는 2D finite volume다. 여기서 저장되는 총 유체 에너지는 열·흐름 운동·화학·압축 에너지의 합이다.71절의 global ledger로 보고할 때 각각을 분리하므로 이 총량을 다시 중복 가산하지 않는다. EOS에는 운동·화학·압축 항을 뺀 내부 열량을 전달한다. 첫 버전의 수치 flux는 1차 Rusanov를 선택한다. F*=0.5(FL+FR)-0.5 amax(UR-UL), amax=max(|un|+cs)다. 공간 reconstruction은 piecewise constant, CFL≤1/4, positivity 검사를 적용한다. 같은 face flux를 한 번 산출하여 한쪽에서 빼고 반대쪽에 더한다. 이는 개발 기준 해법이며 수치 확산이 커서 작은 기구의 효율을 낮출 수 있다. 더 높은 차수의 해법은 같은 보존·수렴 witness를 통과한 후 새 numerical revision으로 승인한다.

종별 flux는 정수 Amount로 commit하고 remainder를 저장한다. 한 cell의 총 출금이 보유량을 넘으면 모든 outgoing proposal을 같은 비율로 제한하고 남는 최소량을 FaceKey순으로 배분한다. 종별 이동량을 제한했으면 동반 운동량·enthalpy도 그 실제 이동량에 맞춰 재계산하며, 빈 질량으로 운동량이나 열만 보내지 않는다. 소스별 제한 전의 반올림으로 질량을 새로 만들지 않는다. 반응·압력 일·열전도·마찰열은 명시된 source term이며 두 solver에 중복 적용하지 않는다.

고체 경계에는 법선 방향 상대 유속0과 실제 벽 속도를 사용한다. 벽이 유체 공간을 침범하면 swept volume을 이웃 유체 cell로 이동시키고 압축 일을 낸다. 갈 공간·수치적으로 유효한 압축 상태가 없으면 물체가 그만큼 진행하지 못해야 한다. 내용물을 삭제하거나 주변에 무상 순간이동하지 않는다. 막힌 틈의 누설은 실제 투과성 또는 수치 오차로 구별하고 표시한다.

### 액체·다상 해법의 선택과 gate

M1 액체는 약한 압축성을 가진 유한 체적 유체로 다룬다. incompressible 전역 projection보다 국소 보존과 결정론 구현이 단순하지만 압력파 때문에 작은 substep이 필요하다. EOS의 후보는 p=K(V0/V-1), 압축 에너지는 Ecomp=K[V-V0-V0 ln(V/V0)]다. V0는 같은 질량의 무압력 부피다. V≥V0에서는 인장 압력 대신 빈 공간·증기 상을 만들며 압축 에너지0으로 이어지는 가지를 쓴다. K의 실제 데이터와 유효 Mach 범위는 material fixture에서 승인한다.

기체와 액체의 부피율·공통 압력·각 상의 열량을 함께 푸는 mixed cell closure가 필요하다. 단일 기체 Rusanov flux를 물에 그대로 적용했다는 이유로 이 gate를 통과한 것으로 보지 않는다. **혼합 EOS, 움직이는 cut-cell의 positivity, 큰 압력비의 안정성은 R6의 수치 확정 전 필수 연구 항목**이다. 그 결과가 없으면 액체 압력 기관을 지원한다고 표시하지 않는다.

기본 해상도는 열린 지역1tile, 고체의 유체 경계와 그 한 tile halo는1/8tile다. refinement는 기하·상 경계와 밀도/압력 gradient 조건으로 결정되며 camera·FPS·worker 수를 참조하지 않는다. 후보 기준은 인접 값의 차이가 두 값 중 큰 값의1/4을 넘으면 refine, 모든 면에서1/16 미만이64tick 유지되고 형상·상 경계가 없으면 coarsen이다. 양쪽0은 차이0으로 정의한다. 결정 시점은 완결 tick 경계이며 retry 중에 기준을 바꾸지 않는다. coarse/fine interface flux는 fine face 합을 사용한다. 분할 시 양·에너지를 체적에 맞게 나누고 정수 나머지를 CellKey순으로 배분한다. 합칠 때 단순 평균으로 운동에너지를 버리지 않고 보존 변수의 합을 쓴다. 재격자화의 인공 에너지·가용 일은 별도 측정한다. 서로 다른 해상도가 비트 단위로 같은 물리를 준다고 주장하지 않는다.

### 열·상변화

물체·유체·기반 토양은 각각 유한 열 저장을 가진다. 더 이상 한 tile의 모든 구성물이 항상 같은 온도는 아니다. 열전도 Qdot=k A (TA-TB)/d는 실제 접촉 면적·두께를 사용하며 이동 열량은 차가운 쪽과의 온도 역전을 제한한다. 물체를 옮길 때 그 물체의 U가 함께 움직이고, 주변 tile U의 비율을 다시 가져오지 않는다.

열팽창은 rest geometry의 온도 의존성으로 둔다. rest_length(T)=rest_length(Tref)[1+α(T-Tref)]이며 strain energy와 내부 에너지의 변화가 같은 물성 모형에서 정산되어야 한다. α만 추가하고 물체를 무상 팽창시키는 방식은 금지한다. 과도한 수치 변형은 파괴 또는 유효 범위 오류로 처리하고 cap으로 에너지를 삭제하지 않는다.

W/V/I 상변화는 온도 고정 if 문을 pressure-dependent phase model로 교체한다. 잠열은 h의 차이와 해당 열역학 데이터에서 나온다. 반응이 진행되려면 그 열량이 실제로 있고 새로운 상의 부피·압력 상태도 유효해야 한다. 같은 물이 압력과 무관하게 항상374에서 끓는 Grid-5 기준은 Matter-1에 적용하지 않는다. 열원·냉각면·누설을 포함한 주기 시험이 열기관의 첫 기능 gate다.

수송 에너지에 enthalpy와 질량 흐름을 함께 정산해야 한다는 원칙은 [Modelica Fluid connectors](https://doc.modelica.org/Modelica%204.0.0/Resources/helpWSM/Modelica/Modelica.Fluid.UsersGuide.ComponentDefinition.FluidConnectors.html)를 참고한다. 이 문서도 국소 연결 근사가 모든 유속에서 정확하다고 간주하지 않는다.

**경계:** W는 상태방정식·흐름·압력 일·열·잠열이다. P는 재료·형상·열·힘 조작이다. E는 용기·수로·압력 기구·열 순환이다. U는 밀폐성·유량·열효율·기능 후보를 해석한다.

## 74 화학·재료·빛: 이름보다 상태와 경로

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

### 반응과 물성의 데이터 계약

MaterialSpec는 formula, charge_number, phase, molar_mass, thermal_model, formation_energy, entropy_reference, conductivity, permittivity, friction, elastic_modulus, yield, fracture_energy, permeability, expansion, surface_parameters를 가진다. 물성은 허용 온도·압력·변형 범위도 선언한다. 범위를 벗어났을 때 임의로 마지막 값을 유지하지 않고 새 상, 파괴, 또는 NumericDomainError 중 명시된 경로를 쓴다.

R5의10종과 원소5종은 초기 재료 계열로 유지하지만 h·열용량을 제외한 미존재 물성을 현실 상수처럼 추측해서 채우지 않는다. MaterialSpec-1은 모델 단위의 **후보 데이터셋**이며, 기계·열·전기 witness와 함께 승인해야 한다. 실제 철·물·목재를 정확히 재현했다고 이름 붙이지 않는다. 같은 substance라도 혼합비·온도·압축·표면·손상으로 반응이 달라진다. ToolGrade, MachineTier, TechMaterial 같은 속성은 없다.

여러 고체의 혼합은 실제 site 조성으로 표현한다. 유효 물성의 reference는 질량·열용량의 가산, 전기·열전도의 실제 연결망, 탄성의 국소 결합이다. 모든 물성을 단순 평균해 금속0.1%만 섞여도 전체가 금속처럼 도통하게 만들지 않는다. 특별한 합금 레시피는 제공하지 않는다. 재료 상태 변화가 유용할 가능성은 있되 현실의 강철·반도체 성능이 자동으로 나오는 것은 아니다.

ReactionSpec는 stoichiometry, surface requirements, forward/reverse barrier, rate law, equilibrium model, energy destination, charge transfer를 가진다. 원소·전하 보존을 정적으로 확인하고 Δh와 평형 정보를 같은 데이터에서 계산한다. 열만 충분하면 모든 원하는 생성물이 생기는 Process 명령을 넣지 않는다. 촉매적 표면은 반응 장벽과 접촉 확률을 바꿀 수 있지만 Δh·평형을 바꾸거나 촉매를 무제한 생성하지 못한다.

열역학적으로 닫힌 reaction cycle에서 순방향만 여러 번 돌려 일을 무상 추출할 수 없어야 한다. 상세 평형과 활동도·압력 의존성을 승인하기 전에는 화학 열기관의 효율을 확정하지 않는다. R5의 허용 반응 표는 보존 oracle의 출발점이며 완전한 열역학 데이터베이스가 아니다.

### 수소·전기화학 경로

R5에서 H가 초기0이고 생성 반응이 없었던 막힘을 그대로 두고 수소 기술을 약속하지 않는다. M1 목표에는 전극 표면의 반응과 닫힌 전하 경로를 통해 2W→2H+O가 가능한 경로를 포함한다. 기존 h를 사용할 때 이 반응의 화학 에너지 증가는 **6000q/extent**다. 그 밖에 온도·압력·전기장·손실의 에너지도 따로 정산한다. 6000q라는 수치만으로 가역 전압을 정하지 않는다. 전압 임계에는 자유에너지와 전하량이 필요하다.

이온 경로를 닫기 위해 Hp=(H,+1), OHm=(OH,-1) 두 이동종을 추가하는 **12종 후보**를 선택한다. 최소 half reaction은 cathode:4W+4e→2H+4OHm, anode:4OHm→2W+O+4e다. 합하면 위 전체 반응과 같고 원소·전하가 각각 보존된다. e는 별도의 원소가 아니라 전극의 전자 전하다. 물의 해리·재결합과 이온 수송에도 같은 화학·전기 에너지 회계를 적용한다.

이 half reaction의 실제 활동도·표준 자유에너지·과전압 계수는 MaterialSpec-1 승인 항목이다. 수치가 아직 확정되지 않았으므로 전기분해를 **설계상 추가할 경로**로 분류하며 R1 기능 증거로 부르지 않는다. 원소가 맞는 반응식만 적었다고 실제 작동 경로가 완성된 것은 아니다. 이는 Matter-1 fixture의 한계다. ML1은82절의 구조 화학으로 교체하며 이12종을 전체 종 목록의 상한으로 삼지 않는다.

산화·부식은 표면에서 물질을 실제 이동시키고 전도 경로·강도·표면을 바꾼다. 불은 Fire 객체가 아니라 발열 반응·열 수송·새 반응의 연쇄다. 폭압은 기체량·온도·부피와 경계 파괴의 결과이며 Explosion 명령은 없다. 피해도 물리적 가열·힘·파괴에서 온다.

### 빛·외기·환경 변화

태양 주기와 외기 경계는 외부 에너지 조건이다. ML1은87절의 실제 광 packet·차폐·반사·굴절·흡수·광반응으로 교체한다. 반사·투과·흡수 비율의 합은1이며 광 에너지가 복제되지 않는다. 예전 photo credit를 동시에 지급하지 않는다.

열 방출 모델을 선형 외기 교환에서 복사식으로 바꿀 경우 같은 run에서 섞지 않는다. 방출량은 보유 열에 의해 제한하고 외기·우주로 나간 양을 기록한다. 물리적으로 막힌 표면과 열린 면은 같은 차폐 함수를 통해 열·빛·유체 접촉에 일관되게 반영해야 한다. 단열이 완전 밀폐나 완전 전기 절연을 자동으로 뜻하지 않는다.

기본 세계에서 자원 재충전·물질 생성형 비·임의 재난은 추가하지 않는다. 강수와 침식이 필요하면 물의 보유량·상·운동량과 퇴적물을 실제로 이동시켜야 한다. 계절 변화·외부 천체 에너지 변경은 공개된 forcing이며 실행 입력으로 기록한다. 새로운 niche는 가능한 물질 경로의 변화에서 생겨야 한다.

**경계:** W는 조성·표면·상·반응·외부 에너지다. P는 기존 국소 조작이다. E는 가공·재료 선택·연소·부식 대응·화학 순환이다. U는 재료·반응·환경 변화의 사후 이름이다.

## 75 전기·물리적 기억·자동화

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

### 부품 목록 대신 전하가 있는 물질

전기 reference는 **기하에서 유도한 저항·정전용량 network**다. agent가 Resistor, Capacitor, Battery, Transistor를 배치하지 않는다. 도체 site의 접촉, 절연체의 배치, 서로 떨어진 표면의 거리와 면적이 network를 만든다. node와 edge는 solver의 derived 자료형이고 영구 전하·재료 상태가 원본이다.

인접 도체 사이 전류는 I=G[(φa+χa)-(φb+χb)]로 시작한다. G는 접촉 면적·거리·온도·조성으로 계산한 양의 conductance다. χ는 재료 표면의 전자 화학 퍼텐셜을 나타내는 모델 값이다. 전하별 허용 범위와 표면 에너지 ΣχQ를 포함하지 않은 상태에서 χ 차이로 전류를 만드는 것은 금지한다. 표면이 산화·마모되어 χ가 바뀌면 그 에너지 변화도 반응·작업에 정산한다.

정전용량 network는 Q_i=Cself_i φ_i+ΣCij(φ_i-φ_j), Eelectric=ΣCself_i φ_i²/2+ΣCij(φ_i-φ_j)²/2다. Cij≥0, Cself>0을 요구해 저장 에너지가 음수가 되지 않게 한다. self capacitance의 반대 전하는 외부 기준 저장소에 기록하고, 그 저장소의 전위·교환 에너지 조건을 manifest에 고정한다. 기준 저장소는 무한 전력 공급이 아니다.

정전 결합의 기본 후보 범위는 표면 간2tile다. 범위 끝에서 Cij와 그 미분이 연속으로0이 되는 kernel을 사용한다. hard cutoff에서 에너지가 갑자기 사라지거나 튀면 실패다. 긴 물질 배선으로 더 먼 곳까지 전달할 수 있지만 이 준정적 축약은 실제 전자기파·빛의 속도를 재현하지 않는다. 커패시턴스의 geometry kernel·self 계수·접촉 전도 데이터는 MaterialSpec-1과 함께 수치 승인해야 한다.

절연체도 표면 전하를 가질 수 있고 이동하면 전하가 함께 움직인다. 절단·파괴 때 Q를 새 물질 면적 또는 전하 site의 실제 귀속에 따라 정확히 분할한다. 물체 삭제, shape 병합, host cache eviction이 전하를 없애지 않는다. 유전체 파괴는 전계·재료 한계에 따른 실제 전도·가열·손상이며 무상 reset이 아니다.

### 전기와 운동은 같은 에너지에서 연결한다

형상이 바뀌면 C(q)가 바뀐다. 고정 전하 조건의 힘은 F=-∂Eelectric/∂q이고 같은 에너지 함수가 역작용을 정한다. 단순 2단자 축약에서 E=Q²/(2C)다. 전하1,C1에서 C를1/2로 줄이면 E는1/2에서1로 늘어나므로 최소1/2의 기계적 일이 필요하다. 전기장에서1/2를 얻고 이 기계 일을 생략하면 발전 경로가 아니라 에너지 버그다.

외부 전압원에 연결된 상태는 고정 전하 조건과 다르다. 움직이는 도체에서 발생한 전류·전원과의 일까지 함께 정산해야 한다. force를 고정 전압 식으로 계산하고 에너지를 고정 전하 식으로 계산하는 혼용은 금지한다. 기술 분류와 무관하게 모든 구조가 같은 결합식을 따른다.

agent에는 ChargeBattery, EmitPower, SetVoltage 명령이 없다. 최초 전하 분리 경로는 실제 재료 표면의 접촉·분리와 전기화학 반응이다. 접촉 대전이 지속 에너지원이 되려면 분리에 필요한 일이 공급되어야 한다. 초기 강제 충전은 Fixture 또는 공개 외부 개입이며 자연 발명으로 세지 않는다.

저항에서 줄어든 전기 에너지는 I²/G의 Joule 열로 들어간다. 열린 회로, 짧은 회로, 부식, 마모, 절연 파괴가 모두 실제 상태에서 생긴다. 물체가 움직여 도체 접촉이 닫히고 열리는 것은 자동 제어의 최소 경로다. solver가 Switch 상태를 무료로 설정하지 않는다.

### 수동 상태와 계산

artifact에 임의 bytecode를 무료 실행시키지 않는다. 물리적 장치의 기억 후보는 전하 저장, 탄성 에너지, 안정한 두 위치, 잔류 변형, 실제 재료 조성이다. 각각 누설·잡음·마모·발열·유지 조건을 갖는다. 프로그램 실행은 기존 agent VM 또는 실제 물리 상태 변화에서만 일어난다.

도체 접점·힘·탄성으로 만들어진 래치와 전하 저장을 조합하면 계수·논리 기능을 표현할 가능성이 열린다. NAND 같은 진리표가 관찰되더라도 코어에 NAND enum을 추가하지 않는다. 한 번 상태가 바뀐 것만으로 계산 장치라고 부르지 않고, 입력 조합·초기화·읽기·반복·오류·전력 소모를 검증한다. 범용 계산 능력은 별도의 구성 증거가 필요하며 이 문서에서 입증하지 않았다.

Modelica의 [Analog.Basic](https://doc.modelica.org/Modelica%204.0.0/Resources/helpDymola/Modelica_Electrical_Analog_Basic.html)은 전기 에너지 관계의 참고 구현이다. 그 라이브러리의 이름 붙은 EMF·전원·스위치 모델을 이 세계의 제작 부품으로 제공하지 않는다.

**경계:** W는 전하·전도·장 에너지·힘·소산이다. P는 물질과 접촉·형상의 변화다. E는 배선·충전·정전 구동·래치·제어다. U는 입출력 시험에서 확인된 기능 이름이다.

## 76 기존 구현 계약의 유지·교체 지도

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

01..05절은 공통 원칙이고06..67절은 Grid-5 기준이다. 아래는 Matter-1 기반으로 옮긴 계약의 지도이며, 최종 MatterLife-1은81..94절의 교체를 추가 적용한다. 같은 run에 서로 다른 ruleset의 규칙을 임의로 혼합하지 않는다.

| R5 절·계약 | Matter-1 결정 | 이유·새 승인 조건 |
| --- | --- | --- |
| 01..05 목적·분리·고수준 금지 | 유지; 최종 제품은 v2-MatterLife | core/observer/viewer 방향 동일 |
| 06..12 world·물질·물리 | 70..75절로 물리 상태·해법 교체 | 실제 형상·압력·전기 필요 |
| 13 고정 생명체 | ML1은83..85절의 발생·대사·조직으로 교체 | 조상 template·생존 예산 새 검증 |
| 14..15 genome·VM | format2·instruction 의미·fault latch 유지 | opcode 번호나 기존 순수 산술 변경 없음 |
| 16..18·52 world I/O | Matter-1은 ABI-M1, 최종 ML1은89절 ABI | 서로 다른 ABI binary의 혼용 금지 |
| 19..22 복제·매체·신호 | 유료 복사·변이·의미 없는 신호 원칙 유지 | ML1은 실제 분할·빛·용질·물리 backing |
| 23..30·58 관찰 | 보상 없는 분석 유지, 새 물리 feature 추가 | 가능·유용·발견·축적 등급 유지 |
| 31..35·56·59 실행 | coordinator·UID·pool 유지, 물리 stage 확장 | 전체 tick의 tentative state와 rollback 추가 |
| 36..38·54 화면 | snapshot 구조 유지, pose·field tile 추가 | interpolation은 물리 입력이 아님 |
| 39..44·53 저장·crate | 새 physical schema와 physics crate | R5 checkpoint 무음 승격 금지 |
| 45 성능·메모리 | R5 수치를 확장 물리에 재사용 금지 | 79절 workload별 재측정 |
| 48·60·67 시험 | T01..97은 Grid-5 golden 유지 | Matter-1의 바뀐 부분은 P01..P36으로 재검증 |
| 62..66 reachability·culture | 증거 원칙 유지, 가능성 표 갱신 | H 경로·회전·압력·전기 한계가 달라짐 |

Matter-1에서 Grid-5의17개 parcel BFS, max-flow load 판정, 순간 quarter-turn, 한 tile 한 actor 점유, 목적지 이동비, 한 tile 공유 U는 더 이상 역학의 권위 있는 규칙이 아니다. 역학 하중과 실패는 실제 접촉·결합 해법으로 판단한다. R5의 최대16 moving parcel을 machine 크기 제한으로 남겨서 새로운 천장을 만들지 않는다. 대신 host 메모리·solver 유효 영역의 정직한 한계를 공개한다.

### ABI-M1의 입력 계약

순수 VM은 기존대로16 register,32 instruction,4 external query,1 effect/tick다. 새 I/O 주소는 16bit descriptor의 사용하지 않던 bits를 임의 재사용하지 않고 별도 version을 manifest에 둔다. 접촉 대상은 Sense에서 얻은 국소 contact slot0..15와 generation/revision으로 캡처하며 다음 tick에 재관측해야 한다. slot은 세계 UID가 아니고 원격 검색에 쓰지 못한다.

| primitive 계열 | Matter-1 입력·효과 | 금지되는 의미 |
| --- | --- | --- |
| READ | 상대 영역의 재료·온도·압력·전위차·접촉력·속도 bin | 원격 기계의 종류·효율·목표·기술 등급 |
| TRANSFER | 노출 site 또는 열린 fluid cell의 실제 물질 | 완성 형상 배치·막힌 용기 내부 직접 입출력 |
| DRIVE | contact slot, 방향, 힘 수준, 일 예산 | 지정 좌표 순간 이동·무한 토크·자동 장치 가동 |
| LINK/DETACH | 접촉 anchor·제약 자유도·접합 물질 | GearRatio·Motor·논리 link |
| WORK | 압축·마모·열의 국소 일, 동일1 effect 예산 | 제련·단조·발전·회로 설치 레시피 |
| WRITE/COPY | 기존 물리 매체의 유료 cell 편집·복사 | artifact의 무료 실행·임의 전하 설정 |
| REPRO | 기존 유료 배아 상태 기계 | 전기·기계 성능을 보상으로 얻는 번식 |

READ의 연속 값은 sign와 magnitude bin을 u16으로 내보내고, scale·saturation·미정 온도 등의 sentinel을 SensorSpec-M1에 고정해야 한다. 상세 register별 wire encoding은 **물리 reference kernel의 단위 확정 뒤에 고정할 잔여 ABI 작업**이다. 현재 표를52절과 섞어 이미 완성된 binary ABI라고 부르지 않는다. 이 항목은80절의 동결 차단 항목이다.

**경계:** Engineering Contract는 profile·ABI·회귀 범위다. W/P 교체를 명시한다. E/U는 같은 증거 원칙을 계속 적용한다.

## 77 Rust 자료형·소유권·시뮬레이션 단계

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

### crate와 상태 소유권

새 crate **civ_physics**를 civ_core 아래의 순수 Rust 계산 계층으로 추가한다. Bevy 의존성은 없다. civ_physics는 numeric, geometry, contacts, constraints, mechanics, thermal, fluids, electrostatic, reactions, materials, coupling, ledger, islands로 나눈다. civ_core가 agent·VM·복제·미디어와 WorldState를 소유하고, 물리 tick 중 PhysicsView의 독점 borrow를 physics coordinator에 제공한다. solver가 agent program을 읽고 기능을 추측하지 않는다.

| 핵심 자료형 | authoritative 필드 | derived·수명 |
| --- | --- | --- |
| MatterWorld | SpeciesStore, BodyStore, BondStore, ThermalStore, ChargeStore, FluxRemainders | Core 단독 소유 |
| BodyState | key, shape_ref, pose, momentum, thermal_ref, charge_sites, damage_ref | mass·COM·inertia는 shape에서 계산 |
| ShapeRecord | material sites, compressed geometry revision | convex boundary·AABB는 cache |
| BondRecord | endpoints, anchors, constraint mask, material amount, rest state, damage | λ warm start는 저장하거나 매 step0 규칙 |
| FluidCell | CellKey, species Amount, momentum2, total energy, phase state | p·T·sound speed는 EOS에서 계산 |
| ThermalCell | owner, internal energy | heat capacity·T는 물질·상에서 계산 |
| ChargeSite | owner site, signed charge, surface state | φ·C/R network는 derived |
| EnergyEscrow | actor, integer q, fractional work, activation heat, unused budget | tick 끝 정산 후 비움 |
| TransferLedger | pair key, domain, source, destination, amount, external flag | commit audit 후 사실 aggregate |
| PhysicsCheckpoint | state pages, substep plan, remainders, numeric revision | 완결 tick만 영구 저장 |

핵심 enum은 MaterialPhase, ConstraintMask, PhysicsError, TransferDomain, RefinementReason, NumericStatus다. MachineKind, PowerPlantKind, LogicFunction은 물리 crate에 없다. 부품·기술 분류는 observer의 후보 record에만 둔다.

### phase와 실패 원자성

1. stage0: 입력·메모리·checkpoint·허용 numeric revision을 검사한다. 변경될 페이지·worst-case contact scratch를 예약한다.
2. stage1: 기존 유지 비용·VM Think·감각·intent를 실행하되 전체 tick의 tentative state에 기록한다.
3. stage2: 실제 물질·접합·작업 요청을 검증하고 geometry revision과 energy escrow를 확정한다. UID 예약은 정렬 순서다.
4. stage3: 바뀐 형상, 접촉, fluid cut-cell, 전기 network를 재구성한다. 경계의 중복·누락을 검사한다.
5. stage4: 독립 physical island별 substep 계획을 만든다. 모든 결합 경계가 같은 island에 들어가야 한다.
6. stage5: substep에서 경계 flux·힘·전기·열·반응 proposal을 만들고 공통 energy ledger로 결합하여 반복 풀이한다.
7. stage6: 보존·양수 재고·비관통·에너지 오차를 검사한다. 실패 island를 이전 물리 상태부터 더 작은 step으로 재계산한다.
8. stage7: 물리 효과·receipt·소산·미사용 escrow를 commit하고 대사·몸체 손상·사망·출생을 처리한다.
9. stage8: 참조·bucket·hash·필수 사실·읽기 전용 snapshot을 commit한다. 완결 상태만 외부에 노출한다.

물리 재시도는 VM을 추가 실행하거나 mutation RNG를 다시 뽑지 않는다. 동일한 intent와 환경 forcing을 같은 기간에 재적분한다. subdivision이 늘었다고 총 태양 유입이나 시도 비용이 늘지 않는다. 마지막 시도도 실패하면 **전체 tick을 이전 완결 상태로 복구**하고 진단을 저장한다. 이미 유지 비용·변이·매체 쓰기를 확정한 뒤 물리만 되돌리는 것은 허용하지 않는다.

Grid-5의 preflight 뒤 in-place commit 최적화를 그대로 재사용할 수 없다. Matter-1은 page journal 또는 chunk copy-on-write로 VM 상태까지 포함한 rollback을 지원한다. host OOM과 NumericDomainError는 특정 개체의 죽음으로 처리하지 않는다. 자동 checkpoint는 마지막 정상 상태를 보존한다.

각 substep의 결합 반복은 시작 상태를 고정하고 trial geometry·pressure·potential을 갱신하는 방식이다. 매 iteration을 실제 시간의 추가 경과로 처리하지 않는다. reference 순서는 열·반응 half-step, 동일 trial에 대한 유체/전기/접촉의 결합 solve, 열·반응 half-step이다. 경계의 최종 transfer는 한 번만 commit한다. 결합 solve는 최대12회 고정 순서 block sweep 뒤 residual을 검사하고, 수렴하지 않으면 step을 나눈다. 단순히 각 solver를 한 번 호출하는 것이 안정된 다중 물리 결합을 증명하지 않는다.

### 병렬화와 거대한 연결 구조

독립 island는 병렬 실행하고 내부 순서는 stable physical key로 고정한다. 유체·전기·열을 통해 연결된 구조를 다른 island로 잘라 각자 무료 경계 조건으로 풀면 안 된다. 거대한 연결망이 생기면 하나의 island가 직렬 병목이 될 수 있다. domain decomposition은 고정 iteration과 경계 flux 정산을 가진 별도 최적화이며 reference와 오차·결정론 gate를 통과해야 한다.

worker가 먼저 발견한 contact에 UID를 주지 않는다. 안정적으로 정렬된 pair key·site key를 사용한다. 각 worker의 reduction은 고정 이진 tree 순서로 합친다. worker 수가 바뀌어도 분할·합산의 논리 순서는 같다. solver workload가 커졌다는 이유로 camera 밖 구조의 substep을 줄이지 않는다.

**경계:** Engineering Contract는 자료형·borrow·phase·rollback·병렬화다. P의 물리 효과는 이 경계를 통과한다. E는 계산 결과이며 U는 완결 snapshot만 소비한다.

## 78 저장·관찰·검증 witness

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

### 새로운 물리 상태의 저장

Matter checkpoint는 별도 physics/schema ID를 사용한다. Grid-5 CIVSIM5의 byte layout을 고쳐 같은 major로 저장하지 않는다. 신규 컨테이너의 section은 기존 agent·VM·lineage·media에 Body,Shape,Bond,Fluid,Thermal,Charge,FluxRemainder,NumericManifest를 추가한다. raw Rust struct dump는 금지한다.

현재 상태뿐 아니라 미래를 바꾸는 damage history, fractional flux, 반응 progress, 쿼리 latch, refinement state, solver warm-start 상태도 저장 대상이다. warm start를 저장하지 않으려면 모든 물리 substep 시작에0으로 초기화하는 규칙을 선택해야 한다. 기본 reference는 warm start를 영속 사용하지 않는다. 재시작 뒤 contact 정렬과 해법 iteration이 같아야 한다.

R5 기록은 해당 physics backend로 replay한다. 명시적 변환 branch는 물질·열·charge 초기값과 geometry 생성 절차·외부 개입을 모두 기록하고 원래 세계의 자연사 연속이라고 표시하지 않는다. renderer의 frame 간 보간은 분석 입력이나 checkpoint에 넣지 않는다.

### 관찰기가 볼 새 증거

분석 표본에 힘-변위 일, 토크-각변위 일, 열 입력·방출, 압력-volume loop, charge-current-voltage, 접촉 topology 변화, 저장 상태 수명, 입출력 지연을 추가한다. 전수 보존 ledger와 상세 trace 표본은 분리한다. 매 contact를 장기간 원자료로 남기지 않고 계량된 합계와 witness 주변의 한정 trace를 저장한다.

자동화 후보는 최소 두 조건을 요구한다. 첫째, 최초 작업자가 잠시 떠나도 외부 물리 입력으로 여러 cycle을 수행한다. 둘째, 그 입력을 끊거나 핵심 연결을 제거하는 counterfactual에서 기능이 사라지거나 달라진다. 계속 옆에서 agent가 WORK를 호출하는 구조를 자율 기계로 분류하지 않는다. 반대로 agent가 연료를 공급한다는 이유만으로 모든 자동화를 부정하지 않고 공급과 내부 cycle의 원인을 구별한다.

### 추가 expected cases

P01..P36은 구현해야 할 물리 시험이다. 실제 실행 결과가 아니다. Fixture의 손작성 구조와 프로그램은 자연 발견 실험에 주입하지 않는다. 단위식 사례가 통과해도 복합 장치·진화 발견을 자동 승인하지 않는다.

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| P01 | mass2, speed3, 운동만 존재 | K=9; 중복 회계0 |
| P02 | inertia4, angular speed2 | Krot=8 |
| P03 | 접촉점 r=(2,0), F=(0,3) | τ=6; 중심 F는 같은 torque를 내지 않음 |
| P04 | 정적 이상 레버 arm1/4, 입력 힘8 | 출력 힘2, 이상 일 동일; 실제 손실은 출력 감소 |
| P05 | friction·gravity 없는 두 body 내부 충돌 | 총 선·각운동량 보존; e=0 손실은 열 |
| P06 | wall 두께1site, 한 step 이동이 wall보다 큼 | sweep에서 충돌; 종점 통과0 |
| P07 | spring k8, deformation0.5 | Eelastic=1; rest reset 비용 없이 생성0 |
| P08 | strained bond 절단 | 재료·전하·에너지 보존, 저장 에너지 무음 삭제0 |
| P09 | tool 끝 접촉 면적 절반, 동일 force | 모델 유효 범위 내 응력2배; 최소 site 면적 이하 가정0 |
| P10 | actor가 막힌 물체에 계속 force | 무한 work0, activation heat·유지 비용 존재 |
| P11 | 이상 기체 n2,R1,T300,V4 | p150; n0에서는 p0·T미정 |
| P12 | 이상화 γ2의 닫힌 단열 기체 U100,V1→V2 | U50, 경계에 전달 가능한 일50; 역압축은 일50 필요 |
| P13 | face를 통해 species 최소량7 이동 | 송신-7,수신+7, 원소 오차0 |
| P14 | coarse face와8개 fine face | coarse flux=fine 합; 재격자화 물질 생성0 |
| P15 | 압축 liquid V0=1,V=0.5,K100 | p100, Ecomp=100(ln2-0.5)≈19.314718 |
| P16 | 닫힌 cavity를 body가 압축 | pressure work와 body 일 반대 부호; 내용물 삭제0 |
| P17 | 두 열원600/300, 연료 없는 주기 장치 | 효율≤0.5; 비주기 저장열 소비와 구별 |
| P18 | 균일 T·무외부입력에서 전기·기계 cycle | 저장 자유에너지 고갈 뒤 지속 순출력0 |
| P19 | W→V가100extent, 기존 latent440 사용 fixture | 최소 잠열44000 정산; phase model의 다른 항도 포함 |
| P20 | 두 electrolysis half reaction 합산 | net2W→2H+O, 원소·전하 보존 |
| P21 | P20, R5 formation h 사용 | chemical energy+6000, 무상 생성0 |
| P22 | 저항 R2,current3을1기간 유지 | Joule heat18, 전원/저장 에너지 출금18 |
| P23 | Q1,C1→0.5, 고정 전하 | E0.5→1, 최소 기계 입력0.5 |
| P24 | P23 역순을 포함한 닫힌 무손실 cycle | net work0; 손실 있으면 외부 입력 필요 |
| P25 | charge7인 조각을 분리 | 자식 charge 합7; 재결합·삭제 때 복제/소실0 |
| P26 | Cij kernel cutoff를 왕복 | 힘·저장 에너지의 인공 jump0 |
| P27 | 유료 초기 충전·분리 fixture | Fixture 배지; 자연 발전 발견으로 계수0 |
| P28 | 물리 latch의 입력 네 조합·초기화 반복 | 실제 입출력표·오류율·소비량 기록; LogicGate enum0 |
| P29 | 같은 intent에서8→16 substep 재시도 | VM instruction·mutation 횟수·총 태양 유입 동일 |
| P30 | 마지막 retry도 실패 | 전체 tick rollback, 부분 출생·비용·기록 commit0 |
| P31 | worker1/4/12·save resume | 승인 target의 CoreHash와 numeric state 일치 |
| P32 | body 밖 camera 이동·MAX 모드 | 동일 물리 hash; physics 해상도 변경0 |
| P33 | 장치의 입력 중단과 핵심 접점 제거 | 기능 변화의 인과 trace; 자동화 이름만으로 통과0 |
| P34 | seed별 무개입 발견 비교 | fixture library를 agent genome에 주입0 |
| P35 | 에너지 residual 예산 초과 | NumericDomainError; 환경 U로 오차 감추기0 |
| P36 | 수백 tile의 연결망과 하나의 giant island | 완전한 물리 또는 정직한 한계 보고; 묵시적 기계 크기 cap0 |

추가 통합 witness는 G1 레버·톱니 접촉, G2 압력과 열의 주기, G3 정전 분리와 회수, G4 물리 상태의 쓰기·읽기·반복, G5 열 입력으로 지속되는 접점 제어, G6 비친자 제작 절차 전달이다. G1..G6은 고수준 API가 아니라 연구자가 조립하는 검증 장면의 이름이다. 출력만 확인하지 말고 전체 제작·유지·수리·공급 비용을 측정한다.

**경계:** Engineering Contract는 저장·expected tests·증거 수집이다. W/P를 검증한다. E는 실제 run에서만 판정하며 U는 미실행·가능·유용·발견·축적을 구분한다.

## 79 메모리·성능·확장 실패의 대응

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

R5의 10.24Mtile·100kagent·1Mparcel에서10TPS 목표는 Grid-5 workload에 대한 미측정 목표였다. 더 비싼 물리 계층을 추가하고 같은 숫자를 보장할 수 없다. Matter-1은 타일·개체 수와 함께 동적 body 수, 접촉 수, 유체 fine cell 수, 전기 node 수, 최대 island 크기, substep 분포를 반드시 보고한다.

| 항목 | 계획 payload | 규모 예와 한계 |
| --- | --- | --- |
| 정적 지도·지형·기초 재고 | 기존44byte/tile에서 재설계 | 12종 Amount를 모든 tile에 dense로 두면 재고만96byte/tile |
| body dynamics | 약128byte/body | 1M이면 약122MiB, shape·bond·cache 별도 |
| 64site shape | 최소64byte/조각 | 1M이면 약61MiB, 상태·압축 형식에 따라 증가 |
| bond state | 약96byte/bond | 2M이면 약183MiB, adjacency·solver scratch 별도 |
| fine fluid cell | 약160byte/cell | 100k tile을64분할하면 약977MiB |
| contact scratch | 약128byte/contact | 1M이면 약122MiB/작업 buffer |
| charge state | 약32byte/site | 1M이면 약31MiB, network matrix 별도 |
| rollback journal | 실제 dirty page 합 | 전체 물리 상태가 움직이면 전체 상태에 가까워짐 |

모든20Mtile을64분할하고 cell당160byte를 쓰면 유체 payload만 **204.8GB, 약190.7GiB**다. 따라서 전체 미세 유체를 밀집 할당하는 구조는32GB 목표에 맞지 않는다. 압축·희소 refinement는 필수지만 최악 상태의 크기를 없애지는 못한다. 저해상도 빈 공간을 정확한 미세 해석과 동등하다고 허위 표시하지 않는다.

### 성능 측정용 세 workload

| workload | 구성 | 초기 승인 목표·해석 |
| --- | --- | --- |
| M-small | 512²tile,10kagent,10k dynamic body,100k fine cell,50k charge site | 최소10TPS, visual은 독립30FPS 목표; 아직 실측0 |
| M-large-sparse | 10.24Mtile,100kagent,1M static shape,10k dynamic body,1M fine cell,100k charge site | 12 physical core·32GB에서 최소1TPS 연구 gate;10TPS는 stretch |
| M-dense-stress | 동일 지도에100k dynamic body·5M fine cell·giant island | 처리량·peak RAM·최저 step 보고, 최소TPS 사전 보장 없음 |

최종 성능 gate는 reference physics의 안정성이 입증된 뒤 측정된 device baseline으로 동결한다. 현재 후보 목표는 완제품 성능의 실측값이 아니다. 1x=10tick/s를 유지하므로1TPS는0.1x이며 UI는 이를 정확히 표시한다. 사용자가 500x를 눌러도 계산이 부족하면 실제 배속이 늘었다고 보여주지 않는다.

핵심 최적화 순서는 정적 기하 재사용, 빈 전기 network 생략, exact equilibrium sleeping, SoA·arena·batch, 독립 island 병렬화, 검증된 coarse/fine 해법이다. 온도·부식·전하 누설이 계속 변하는 물체는 운동량0이라는 이유만으로 전체 물리를 sleep시키지 않는다. camera LOD는 renderer만 바꾼다.

### 새 실패 모드

| 실패 | 예방·검출 | trade-off |
| --- | --- | --- |
| 물리가 늘었으나 유용한 조합 없음 | 단순 precursor부터 비용 이득을 재고 복합 witness로 연결 | 물성 탐색 실험 필요; 발견 보너스 금지 |
| 무상 에너지 순환 | pair ledger·두 열원 cycle·정전 왕복·재격자화 시험 | 정밀 검사 비용과 지원 수치 영역 제한 |
| 기계가 numerical exploit만 사용 | step/해상도 변경·다른 reference 해법에서 기능 재확인 | 완전히 동일한 수치 결과를 기대하지 않음 |
| 틈 누설·톱니 관통 | sweep·cut-cell·격자 수렴·최소 간극 명시 | 작은 형상일수록 높은 비용 |
| 전체 세계가 하나의 island | 연결망 규모를 기록하고 고정 경계 iteration 연구 | 병렬 가속이 제한될 수 있음 |
| 최적화가 물리 법칙을 몰래 바꿈 | camera·speed·worker별 hash, offscreen witness | 일부 게임식 sleep·LOD 사용 불가 |
| 구성 요소가 너무 많아 탐색 불가능 | 간단한 중간 형태의 생존 이득·재사용·지역 보존 확인 | 자연 발견이 없으면 물성·환경 가설을 새 run에서 재검토 |
| 기능 이름이 Core에 재침투 | crate dependency·enum·자료형 review, no prefab gate | 검증 장면과 실제 제작 기능을 엄격 구별 |
| 보기만 복잡하고 문화는 없음 | G6 전달·재제작·소실 시험을 독립 유지 | 역학 성공만으로 문명 성공을 선언 못함 |

**경계:** Engineering Contract는 자원 예산·측정·오류 표시다. W/P는 성능 설정에 따라 변하지 않는다. E의 발생 여부와 계산 처리량은 별도다. U는 지원 범위와 실제 TPS를 보여준다.

## 80 Matter-1 기반의 동결 조건과 구현 의존성

**적용 범위: Matter-1 물리 기반이다. R7 확장은81..94절, R8은95..107절, 최신 R9 교체는108..120절이 우선한다. 현재 미동결 항목은120절을 따른다.**

이번 개정으로 정한 것은 **어떤 종류의 세계를 만들지, 어떤 고수준 지름길을 금지할지, 어떤 물리 경로를 이어야 하는지, 기존 구현 경계를 어디서 바꿀지**다. R5에 기계·전기를 몇 줄 붙이면 완제품 설계가 끝난다는 결론은 채택하지 않는다. 물리 확장은 새로운 기술 표현 공간을 열지만 검증 부담도 실질적으로 늘린다.

### 남은 동결 차단 항목

| 항목 | 현재 상태 | 닫는 데 필요한 산출물 |
| --- | --- | --- |
| 고수준 금지·아키텍처·자료 소유권 | 설계 결정 완료 | dependency/금지 API review |
| 역학·유체·열·전하 에너지 경로 | 수식·불변 조건·시험 결정 | 실제 reference trace와 ledger |
| MaterialSpec-1 수치·유효 영역 | 후보 구조 결정, 실제 데이터 미동결 | 모든 물성·EOS·phase·rate의 값/단위/범위/hash |
| mixed fluid EOS·cut-cell | 해법 방향 결정, 안정성 미입증 | positivity·압력비·해상도 수렴 보고 |
| 접촉·탄성·다중 물리 적분 | 기본 후보 결정, gate 미통과 | P01..P18·P29..P35 실제 실행 |
| 전기 기하 kernel·표면 데이터 | 에너지 함수 결정, 수치 미동결 | P20..P28·정전 구동 witness |
| ABI-M1·Founder-5 | 개념 계약 결정, binary 미작성 | register 단위 명세·실제 조상 bytes/hash·생존 예산 |
| 저장 schema·portable math | 필요한 상태 식별, byte schema 미동결 | wire layout·golden vectors·resume hash |
| 확장 물리 성능 | workload·후보 gate 결정, 실측 없음 | profile별 CPU/RAM/TPS 결과 |
| 자연 발명·문화 축적 | 연구 목표, 증거 없음 | 다중 seed의 미개입 기록과 재현 가능한 분석 |

이 표의 미동결 항목을 숨기고 “보고 코드로 옮기기만 하면 결과가 보장되는 최종 설계”라고 부르지 않는다. 동시에 이 항목을 고수준 부품을 도입할 명분으로 사용하지 않는다. reference 물리의 작은 실험은 시뮬레이터 전체를 먼저 만드는 작업과 구분된다. 아직 사용자 요청대로 실제 시뮬레이터 소스는 작성하지 않았다.

Matter-1의 초기 세계도 새 WorldGen-M1 manifest를 가져야 한다. Grid-5의 기체 재고 숫자를 새 체적·상태방정식에 그대로 넣어 과압 세계를 만든 뒤 우연한 붕괴를 문명 경쟁으로 설명하지 않는다. 원래의 지리적 차이·유한 자원·공개 조상 원칙을 유지하되, 초기 pressure/temperature/phase·생체 열 접촉·낮밤 에너지 수지를 새 단위에서 검사한다. 외부 환경의 재고·에너지 총합은 시작 ledger에 명시한다.

### 구현의 의존 순서

1. MaterialSpec·상태 저장·원소/전하/에너지 oracle와 portable math를 먼저 만든다. 물성 데이터의 차원·경계·반응식을 기계적으로 검사한다.
2. 작은 물질 세계에서 형상·힘·회전·결합·마찰·파괴를 검증한다. 이 단계에서 agent 없이 G1의 기계적 가능성부터 확인한다.
3. 열·부피·기체·액체·움직이는 경계를 결합한다. 압축·팽창과 열원/냉각원의 전체 cycle을 검증한다.
4. 전하·도통·절연·정전력·표면 반응을 연결하고 재료가 움직일 때 전기 에너지의 역작용을 검증한다.
5. 같은 primitive를 사용하는 VM과 Founder-5를 연결한다. 손작성 기능 witness와 자연 발견 조상을 분리한다.
6. 유료 기록·제작 절차 전달·재제작·손상·수리·소실을 검증한다. 기계가 가능해졌다는 이유로 문화 시스템을 생략하지 않는다.
7. 저장·replay·worker 동등성·Bevy 화면·대규모 workload를 통과시킨다. 그 다음 미개입 장기 실험을 진행한다.

원래의 생태·문화·관찰기 설계를 무효로 버리는 것이 아니다. 기존 VM·물리적 기록·계보·증거 단계·UI·headless 경계를 재사용하고, 이전에 없던 물리 자유도와 그 비용을 올바르게 연결한다. 새 수치 모델은 같은 철학을 지키면서 별도로 승인해야 한다.

가장 중요한 성공 증거는 물리 요소의 개수가 아니라 **더 단순한 유용한 조합에서 더 복잡한 조합으로 이어지는 실제 경로**다. 한 계통이 처음부터 정교한 열기관을 우연히 조립해야만 이득을 얻는 세계는 표현력이 넓어도 진화적으로 막힐 수 있다. 단순 용기·압력 이용·반복 접촉·전하 저장 같은 중간 구조가 어떤 조건에서 비용을 회수하는지 검사한다. 그 구조가 필요하다는 판정과 실제 진화가 발견했다는 판정을 끝까지 구분한다.

**경계:** Engineering Contract는 동결·개발·증거 gate다. W/P는 임의 기술 보상을 제공하지 않는다. E는 실행에서 나타나는 조합과 전달이다. U는 지원 상태와 연구 증거를 정직하게 표시한다.

## 81 R7 결정: 물질·생명·전자기·빛을 같은 세계로 연결

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

**최종 목표를 v2-MatterLife로 확장한다.** 새 실행 규칙의 임시 ID는 civ-matterlife-1-draft다. 사용자의 범위 변경에 따라 형태 진화 제외 결정을 철회한다. 행동뿐 아니라 발생, 조직 배치, 막 수송, 촉매 구성, 생식 투자도 유전·변이 대상으로 삼는다. 자기·유도와 기하 광학은 후속 희망 목록에서 필수 구현 범위로 옮긴다. 화학은 고정 12종 목록을 넘어 구조를 가진 화학종을 만들 수 있도록 바꾼다.

이번 개정은 **실행 코드 추가가 아닌 설계 추가·교체**다. 실제 reference solver, 조상 bytecode, 물성 데이터와 장기 진화 결과는 아직 없다. 물리적으로 표현할 수 있는 기능, 손으로 구성하면 작동하는 기능, 진화가 발견한 기능을 R0..R4 증거 단계로 계속 구분한다.

### 적용 우선순위

01..05절은 공통 철학·제품 경계다. 06..67절은 Grid-5 기준과 감사 이력으로 보존한다. 68..80절의 형상·역학·유체·열·전기·회계 계약을 기반으로 삼되, 아래 표의 사항은81..94절로 교체한다. 두 ruleset을 한 tick 안에서 섞지 않는다. 최종 동결 목록은94절이다.

| 기존 계약 | R7 교체·추가 | 유지하는 부분 |
| --- | --- | --- |
| 고정 몸·대사·공통 actuator | 실제 조직량·형상·발생 규칙·국소 수송 | 몸도 물질·에너지·공간을 점유 |
| 5원소·12종 물질 고정 | 8원소와 동적으로 등록되는 분자·고분자 구조 | 원소·전하·질량 보존 |
| 저항·정전용량 중심 전기 | 기하에서 유도한 인덕턴스·자화 상태와 역작용 | 실제 전하·전도·절연·열 |
| 태양 흡수의 축약 계산 | 스펙트럼·차폐·반사·굴절·흡수·방출 | 외부 유입량과 방출량 회계 |
| agent만 살아 있는 객체 | 같은 compartment 규칙을 쓰는 여러 생활 양식 | 생존·번식을 통한 선택 |
| G 기반 고정 코드·몸 비용 | C/H/O/N/P 등을 가진 실제 매체·조직 재고 | 기록·복사·계산의 유료성 |
| 개체 단위 에너지·명령 | cell의 에너지·controller와 실제 내부 전달 | 국소 관측·지연·실패 receipt |

물리적 역할이 다른 것만 새 상태로 추가한다. 불필요한 옵션을 늘려 조합 수를 부풀리지 않는다. 예를 들어 새 염료는 단순 색 ID가 아니라 파장별 흡수·반응·열의 차이를 가져야 한다. 새로운 체형은 이동 speed 보너스가 아니라 질량·면적·접촉·수송 거리의 변화로 영향을 줘야 한다.

### 범위와 제한

포함: 발생을 통한 형태 변화, 다세포 조립, 조직 손상·재생·분해, 광영양·화학영양의 경로, 물질을 통한 생태 순환, 준정적 자기·유도, 기하 광학, 가변 구조 화학종, 촉매·막·고분자와 물성의 연결이다.

제외: 원자 단위 양자화학, 실제 단백질 접힘 예측, 완전한 3D 생체역학, DNA에서 현실 동물을 재현하는 모델, 무생물에서 생명의 최초 발생 보장, 전파·회절·간섭·편광의 완전 해석이다. 이러한 제한은 문명 등급으로 해금되는 규칙이 아니다. 다른 ruleset이 필요한 표현 한계다.

**핵심 원칙은 “더 많은 조합이 가능한 매체”와 “그 조합에 도달할 생존 가능한 경로”를 함께 만드는 것이다.** 저수준 법칙이 많아도 탐색 비용만 커지거나 독립된 장난감으로 남으면 목표에 도움이 되지 않는다. 작은 기능이 비용을 회수하고, 기존 구조가 다음 구조의 재료·제어·지식으로 재사용되는지를 검사한다.

**경계:** W는 발생·재료·장·반응·빛·보존 규칙이다. P는 국소 물질·힘·기록 조작과 세포 내부의 발현 제어다. E는 체형·생활 양식·장치·생태 관계·문화다. U는 동식물·기관·기술·문명의 사후 분류다.

## 82 일반 화학: 유한 원소에서 가변 구조와 반응망으로

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 선택지 비교

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 물질·반응 목록만 확장 | 물성 검증과 대량 처리 쉬움 | 목록 밖 화학종·새 촉매를 만들 수 없음; 확장할 때마다 개발자가 경로를 추가 | 기준 fixture에 한정 |
| 원자·전자·분자동역학 | 상세한 구조 상호작용 가능 | 문명 세계 규모와 맞지 않음; 시간척도 차이와 매개변수 비용 매우 큼 | 제외 |
| 구조 화학종·국소 graph rewrite·coarse 물성 | 새로운 구조·촉매·고분자와 재조합 가능 | 반응 후보 폭증, 구조 동일성 판정, 열역학 데이터의 일관성이 어려움 | **선택** |

선택안은 실제 화학의 보존·활성화·평형·촉매·상분리를 참고하는 **인공 화학**이다. 원소 이름을 현실과 맞춘다고 현실 화학 계산기가 되지는 않는다. 새 구조의 효과는 같은 공개 함수에서 계산하고, 발견 순간 난수로 강도·촉매 효율·에너지를 선물하지 않는다. [Dittrich 등, Artificial Chemistries — A Review](https://www.cs.mun.ca/~banzhaf/papers/alchemistry_review_MIT.pdf)

### 원소·화학종·양의 표현

ElementKey의 고정 순서는 C,H,O,N,P,S,Fe,Si다. 초기 모형 질량은12,1,16,14,31,32,56,28이다. 이는 근사 모형 단위이며 실제 원자량 표라는 표기를 하지 않는다. 이전 R5의 물질 S는 silica 별칭이었다. R7에서 원소 황 S와 충돌하지 않도록 silica는 반드시 SpeciesKey 또는 SiO2로 쓴다. 원소 수 벡터는 u32[8], 전체 원소 장부는 checked u128[8]이다.

화학종의 최소 정의는 연결 graph, 각 atom의 원소·형식 전하·허용 전자 상태, bond order, 입체 상태를 대신하는 공개 이산 tag다. 용액 상태와 온도는 종 ID가 아니라 StockOwner 상태다. 상이 달라져도 원소 구성은 바뀌지 않는다. R6 W/V/I를 물질 세 종으로 다루던 표현은 같은 H2O 종의 phase와 phase inventory로 옮긴다. 전자도 charge carrier 장부에 들어가되 원소 벡터에는 포함하지 않는다.

동일 구조는 canonical bytes가 같아야 한다. 빠른 색 정련·해시 뒤 애매한 대칭 구조는 canonical labeling으로 확인한다. 해시만으로 동등성을 확정하지 않는다. 여러 worker가 만든 후보는 canonical bytes 순서로 등록하고 runtime SpeciesKey를 부여한다. 캐시 누락·삽입 순서가 반응 성질과 RNG를 바꾸면 안 된다.

기준 profile에서 작은 분자의 명시 graph는 heavy atom64개까지, 고분자는 표준 연결 규칙을 갖는 반복·분기 monomer graph 최대4,096 unit까지 허용한다. 이 상한은 공개된 표현 제한이며 현실의 화학 법칙으로 주장하지 않는다. 더 큰 구조를 지원하려면 새 manifest로 실험을 시작한다. RAM 부족을 이유로 해당 tick의 새 종만 삭제하는 것은 금지다. 상한 밖 생성은 해당 profile에 정의되지 않은 반응으로 처음부터 제외하고, 도달 시도는 분석에 기록한다.

### 반응 생성과 검증

RuleSchema는 특정 상품의 제작법이 아니라 허용되는 국소 결합·전하 변환이다. 기준 계열은 결합 형성·절단, 결합 교환, 양성자 이전, 전자 이전, 축합·가수분해, 배위 결합, 촉매에 의한 장벽 변화다. RuleSchema 각각은 atom mapping, 전하 mapping, 가역 역규칙, 반응 중심 radius≤2, 필요한 contact phase, transition-state 함수를 가진다. 아무 원자 두 개를 붙이는 만능 명령은 없다.

각 후보는 Σν_s element_s=0, Σν_s charge_s=0을 먼저 통과해야 한다. 외부 전극과 전자 교환 시 전극의 Q도 같은 transaction에 들어간다. 반응물 재고·접촉·실제 이용 가능한 일·열·빛이 부족하면 해당 extent가 줄어든다. 결합 절단에서 새 원소를 만들거나 반응물보다 많은 생성물 질량을 만들 수 없다.

화학 저장 에너지는 Echem=Σn_s h_s다. 여기서 h_s는 기존 문서 기호를 이어받은 기준 chemical internal energy이며 SI 표준 생성 엔탈피를 그대로 대입하는 값이 아니다. 엔탈피 자료를 사용할 때는 해당 상태의 pV 항과 sensible/phase energy를 분리해 내부 에너지로 변환한다. h_s는 원자 기준 에너지, 국소 bond·환경 항, phase 항에서 산출한다. 자유 에너지는 같은 데이터의 열·엔트로피·혼합 항을 포함한 G(T,p,n,Q)에서 구한다. h와 G를 같은 값처럼 쓰지 않는다. 에너지 장부는 Δh와 실제 열·일을 정산하고, 반응의 방향·평형·일의 가능성은 화학 퍼텐셜 μ_s=∂G/∂n_s를 사용한다.

양방향 transition-state 에너지를 G‡로 두고 ΔG‡f=G‡−Greactant, ΔG‡r=G‡−Gproduct로 연결한다. 표준 상태에서 kf/kr=exp(−ΔG°/RT)를 만족하도록 같은 prefactor 규약을 적용한다. 농도·활동도는 실제 rate에 별도로 들어간다. 하나의 장벽을 낮추면 정·역방향에 같은 transition-state 변화가 적용되어야 한다. 촉매가 평형을 마음대로 바꾸거나 닫힌 반응 cycle에서 에너지를 만드는 것은 금지다.

비이상 용액을 정확히 예측한다는 목표는 두지 않는다. 첫 reference는 희석 용액의 이상 활동도와 공개된 고농도 유효 범위를 사용한다. 그 범위를 벗어나면 다른 식으로 몰래 바꾸지 않고 수치/물성 범위 실패를 보고한다. phase equilibrium, 용해도, 전도성도 서로 다른 임의 표로 충돌시키지 말고 동일 free-energy 데이터와 수송 coefficient를 사용한다.

### 촉매·고분자·새 재료

촉매는 실제 분자 또는 노출된 표면이다. 결합점의 국소 전하·형상·donor/acceptor 패턴이 반응 중심과 맞으면 transition-state 항이 달라진다. genotype에는 “이 물질을 10배 잘 만드는 능력”을 직접 넣지 않는다. genotype이 합성하는 분자 구조와 그 노출·양·배치가 촉매성을 만든다. 활성점 점유·흡착·탈착·poisoning도 재고를 가진 상태다.

순수 촉매 cycle은 촉매를 반환한다. 실제 손상·산화·잘못된 결합이 일어나면 손상된 종이 남고 재합성 비용이 든다. 새 촉매가 안정적인 에너지 공급을 얻을 수 있을지는 실행으로 확인한다. 자가촉매 반응망을 감지하더라도 그것을 자동 생명체나 무료 복제자로 승격하지 않는다.

고분자는 graph의 반복, 가교, 방향성, 용매 상호작용을 통해 강도·점성·투과·광학 흡수·전도 등의 coarse 물성에 영향을 준다. atom→물성 함수는 유계·연속적인 범위를 갖고, 다른 원소 예산 없이 “무한히 강한 재료”가 나오지 않아야 한다. 함수의 정확한 coefficient와 유효 범위는 MaterialData-ML1의 동결 대상이다. 현실의 합금·반도체·단백질 특성까지 이 함수로 자동 예측된다고 약속하지 않는다.

### 계산량 억제와 결정론

각 StockOwner는 공통 소분자용 작은 dense 배열과 희귀 종용 정렬 sparse span을 가진다. 전 타일에 모든 SpeciesKey 슬롯을 만들지 않는다. 규칙 색인은 phase·반응 중심 signature·접촉면을 기준으로 후보를 좁힌다. reaction cache는 구조 버전·온도 구간·노출 상태별로 무효화한다. 화학종의 이름이나 생물 분류로 후보를 거르지 않는다.

기준 수치 방법은 deterministic extent accumulation과 adaptive chemical substep이다. 반응 key 순서는 canonical rule/atom mapping 순서이며, 경쟁 반응의 순서 편향은 순방향·역방향 sweep을 대칭 적용하고 substep 수렴으로 검사한다. 양수 재고·에너지 경계를 넘는 extent는 재계산한다. 원소는 정수 Amount와 reaction remainder로 정확히 보존한다. timestep 축소 후에도 회계·오차 조건을 만족하지 못하면 tick을 rollback한다. stochastic molecular mode는 다른 profile로 분리한다.

새 종 수가 많다는 이유로 “중요하지 않은 종”을 삭제하지 않는다. 재고0·표면 점유0·매체 참조0·반응 진행0인 종의 runtime cache만 회수할 수 있다. 역사와 genotype에서 사용하는 구조 정의는 content-addressed archive에 남긴다.

**경계:** W는 원소·허용 결합·반응·열역학·수송·coarse 물성 함수다. P는 물질 접촉·분리·일·온도·국소 발현 조작이다. E는 새 분자·촉매·재료·대사 경로다. U는 화학 family·제조 절차·기술 계보다.

## 83 생물의 최소 단위: 물질로 된 compartment와 조직

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 표현 선택

| 방식 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| size/speed/attack gene | 매우 작고 빠름 | 물리적 원인 없이 능력치를 주며 체형 조합을 제한 | 제외 |
| 모든 분자·세포를 상세 해석 | 생리적 표현 폭 큼 | 수십만 개체에서 비현실적인 비용 | 제외 |
| 유한 compartment·조직 패치·국소 발생 | 몸의 배치와 수송, 재료와 제어를 함께 진화시킴 | 세포보다 큰 해상도의 인공 생물학이며 수치 결합 필요 | **선택** |

여기서 Cell은 실제 세포 한 개와 일대일 대응하지 않는 **중간 규모의 살아 있는 compartment**다. 한 Cell은 하나 이상의 material site와 닫힌 내부 재고, 막 표면, 복사 가능한 template, 국소 발현 상태를 가진다. 원래 agent 몸체를 여러 sprite로 나누는 방식이 아니다. 외부와 물질을 주고받는 면적, 내부 압력, 재고의 부피, 연결의 강도를 실제로 계산한다.

### Cell과 몸의 소유권

CellKey는 독립 compartment의 안정된 참조다. BodyKey는 물리적으로 움직이는 덩어리의 참조다. 한 Body에 여러 Cell이 있고, 한 생명 계통의 Cell들이 여러 Body를 결합할 수도 있다. Cell이 어떤 생물처럼 보이는지는 관찰기가 판단한다. OrganismView는 분석 cache일 뿐 재고·에너지·지식 접근 권한을 주지 않는다.

Cell은 StockOwner, membrane site span, scaffold bond span, TemplateKey, developmental state, CellWorkStore, optional ControllerKey를 가진다. 실제 물질은 stock 또는 material site 중 정확히 한 곳에만 있다. “조직 질량”을 계산할 때 내부 재고와 몸 질량을 다시 더하지 않는다. 굳은 외피·죽은 섬유·분비물은 더 이상 생체 controller가 없어도 같은 material law를 따른다.

다세포 연결에는 화학·전기·유체 통로가 각각 필요하다. 같은 OrganismView에 속한다는 이유로 한쪽의 음식이나 에너지가 즉시 전신에 분배되지 않는다. transport는 면적·거리·투과도·농도차·압력차와 실제 전하를 따른다. 큰 몸은 저장과 표면 배치에 이점이 있을 수 있지만 수송·유지·운동 비용도 생긴다.

### 막 수송·삼투·산염기 상태

막의 수동 종별 flux는 같은 free-energy 함수에서 얻은 안팎의 electrochemical potential 차를 구동력으로 삼는다. 희석 reference에서는 μ_s=μ°_s+RT ln(a_s)+z_s F_model φ이며, F_model은 전하량과 모형 energy를 연결하는 단위 상수다. 두 쪽의 차이를 Δμ로 정의하면 J=L_s Δμ, L_s≥0인 수동 통로는 높은 퍼텐셜에서 낮은 쪽으로 흐른다. 실제 flux는 노출 면적·막 두께·원료·수용 부피와 정수 remainder로 제한한다.

전하를 띤 종이 움직일 때 chemical stock과 두 쪽 Q를 동시에 바꾼다. 농도차·전압차를 거슬러 수송하려면 실제 carrier turnover 또는 다른 감소하는 자유 에너지와 결합해야 한다. active transport의 가역 최소 일은 옮긴 amount×거슬러 올라간 Δμ이며 비가역 손실을 별도로 더한다. “같은 생물”이라는 이유로 이 비용을 면제하지 않는다.

물도 같은 solvent chemical potential에서 흐른다. 이상 희석 한계의 삼투압 차 RTΔc는 검증 fixture로 사용하며, solvent μ로 이미 계산한 효과를 별도 삼투 force로 다시 더하지 않는다. 물 유입·배출은 내부 압력·부피·막 응력에 연결된다. 팽윤은 공간과 막 물질이 있어야 하고 지나친 압력은 실제 결합/막 손상으로 이어진다.

pH는 수소 이온 활동도에서 계산한 관측량이다. 산·염기를 중화하는 것은 실제 proton transfer와 열·전하 보존 반응이다. 임의 pH tile stat를 바꿔 몸에 고정 damage를 주지 않는다. 초기 생체/환경 profile의 완충 능력도 실제 buffer species의 재고로 표현한다. 뿌리 같은 구조의 흡수는 접촉한 토양 pore와의 flux이고, 지하 전용 원격 자원 주소는 없다.

### 생화학 기반 장치와 명시적 추상화

최초 생명의 발생은 이번 목표가 아니다. 조상 Cell에는 막 형성, template 복사, 유료 실행, 합성·분해의 최소 기구를 공개 초기 상태로 제공한다. 이는 개발자가 제공하는 생명 기반이다. 조상에게 손·다리·눈·잎·뿌리·뇌·위 같은 완성 기관을 제공하지 않는다. 기관 비슷한 기능은 동일 조직의 배치와 상태 변화로 만들어져야 한다.

실행·복사는 여전히 VM과 계수 기반의 coarse machinery로 모델링한다. 실제 리보솜·DNA 중합·신경세포를 분자 수준으로 재현했다는 주장을 하지 않는다. 이 추상 장치도 실제 막·매체·촉매 stock이 존재하는 동안만 작동하고, 분해되면 정지한다. 생명이 아닌 구조에 bytecode를 놓았다고 무료 CPU가 생기지 않는다.

구조를 만드는 물질에는 C/H/O만 있는 탄수화물계, N을 포함하는 촉매·고분자계, P를 포함하는 막·기록 매체 등이 있다. 어떤 비율이 필요한지는 해당 화학 구조의 원소 벡터가 결정한다. 매체의 물리 backing이 줄면 용량도 줄고, 손상된 word는 손상 규칙을 따른다. 생체 매체와 비생체 외부 기록은 별도 재료로 만들 수 있지만 같은 word 접근·출처 검증 계약을 지킨다.

### 조직에서 힘·감각·유지가 나오는 방식

조직의 actuator는 재료 결합의 평형 길이·strain을 유료로 변경하는 국소 상태 변화다. 최대 일률은 실제 활성 재료량과 turnover, 힘은 단면과 허용 stress, 가동 범위는 결합 geometry로 제한한다. Controller의 DRIVE는 선택한 자기 조직의 목표 활성량만 요청한다. 대상 물체에 먼 거리에서 힘을 부여하지 않는다. 작용·반작용과 CellWorkStore 감소, 비가역 손실의 열을 함께 기록한다.

형상에 따라 접촉을 밀고 당기거나 마찰이 달라지면서 이동할 수 있다. speed gene이나 limb enum은 없다. 생체 결합을 흥분시키는 입력과 수동 재료의 열팽창·정전력·자기력은 서로 다른 원인이지만 동일 역학 해법으로 합쳐진다.

감각도 면적·재료·국소 접촉에서 나온다. 압력·변형 수용, 흡착에 의한 화학 반응, 빛 흡수, 전위 변화가 공개된 transduction 함수를 통해 Controller 입력으로 들어간다. 기본 프로필에는 제한된 해상도의 유료 센서 readout을 제공한다. 새 체형에 눈 같은 무늬를 그렸다는 이유로 전역 시야를 주지 않는다. 몸 안 다른 Cell의 감각도 실제 전달 경로와 지연이 필요하다.

유지 비용은 생존 세금 하나로 끝내지 않는다. 막 누출·고분자 손상·촉매 변성·template 오류와 복구 합성이 물질을 소모한다. 계산·복사 비용은 추가로 계량한다. 첫 구현에서 이를 단순한 turnover law로 근사할 수 있지만, 같은 물질의 온도·pH·산화·물리 손상과 연결하고 손실 생성물을 남겨야 한다. 0으로 감소한 부품이 다른 종류의 영양분으로 임의 변환되어서는 안 된다.

**경계:** W는 compartment·막·합성·유료 실행·조직 물성·수송이다. P는 자신의 국소 발현·조직 활성과 외부의 기존 물질 조작이다. E는 체형·기관 유사 기능·움직임·먹이 처리·방어다. U는 잎·근육·껍질·감각 기관 같은 기능 가설이다.

## 84 발생 genome·제어·복제·형태 돌연변이

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### Genome의 세 부분

GenomeBundle은 BehaviorProgram, DevelopmentProgram, SynthesisTemplates를 갖는다. 세 부분 모두 가변 길이·모듈 재사용·복사 출처를 지원한다. BehaviorProgram은 기존 레지스터 VM을 계승한다. DevelopmentProgram은 국소 상태를 읽어 발현 목표를 정하는 작은 조건 규칙과 module call로 구성한다. SynthesisTemplate은 허용된 화학 graph의 조립 순서를 기술한다. 이것은 없는 원소를 만들어내는 구조 청사진이 아니다.

개체 출생 때 완성 body를 전역 좌표에 한 번에 배치하지 않는다. 원료 흡수→합성→막 확장→내부 부피 확보→template 복사→분할 경계 형성이라는 실제 과정이 필요하다. Controller 없이도 DevelopmentProgram과 물리·화학 반응만으로 성장하는 계통이 가능하다.

### 발생 방식 비교

| 방식 | 장점 | 단점·성능·복잡도 | 개방성과 선택 |
| --- | --- | --- | --- |
| 전체 몸 blueprint를 즉시 생성 | 재현·해석 쉬움, 초기 빠름 | 성장·공간·중간 생존 비용을 건너뜀 | witness 초기 배치에만 사용 |
| 모든 Cell의 큰 VM | 임의 발생 제어 가능 | 작은 생태 개체에도 큰 제어 상태·명령 비용 | 기본 제외 |
| 국소 발생 규칙 + 선택적 controller | 대다수 조직은 작은 상태, 필요한 곳만 프로그램 | 지연·조직 연결·규칙 해석기 구현 필요 | **선택** |

DevRule 입력은 자기 화학 stock·작업 에너지·온도·압력·국소 빛 흡수·변형·연결면의 전달 신호다. 출력은 합성 target의 rate 상한, 막 통로의 활성 비율, 접합 재료의 배치, scaffold의 목표 strain, template 복사 투자, 국소 분할 축의 bias다. 자기 바깥의 material site를 명령 한 번으로 조직으로 변환하지 않는다.

Cell당 controller는 최대1개다. 발생 규칙은8tick 동안 유지되는 기본 expression target을 만들고, 그 Cell의 controller는 명시한 항목을1tick 동안 override할 수 있다. 여러 규칙이 같은 target을 쓰면 rule/module의 canonical 실행 순서에서 마지막 유효 기록을 사용한다. 어느 경로의 target도 원료·촉매·work 상한을 넘는 실제 반응을 만들지 않는다.

기준 scheduling은 매 tick 생화학·누출·손상을 처리하고, CellUID mod8에 맞춰 8tick마다 발생 규칙을 갱신한다. 기존 발현 목표는 중간 tick에도 유지되며 그 반응의 실제 원료·에너지를 매 substep 청구한다. Controller는 활성 Cell에만 있고 최대32개 VM instruction/tick을 실행한다. 이 clock은 생태 class가 아니라 공통 실행 계약이다. 동일 상태의 프로그램을 memoize하더라도 생물학적 비용은 그대로 청구한다.

### 성장·분할·기하 제약

site를 채우는 합성은 빈 부피 또는 압력·변형 일을 감당할 공간이 있을 때만 진행한다. 최소 막 면적·닫힌 경계·내부 stock·template의 유효한 복사가 갖춰지면 partition candidate를 만들 수 있다. 분할은 통과 가능한 주변 부피, 새 경계의 실제 물질, 분리 work와 작업량을 요구한다. 충돌 시 다른 몸을 지우거나 위로 겹치지 않는다.

분할 경계는 고정된 세계 좌표의 오른쪽 같은 편향 대신 자기 polarity frame과 local geometry를 사용한다. 기본 경계 탐색 순서는 canonical 후보 순서이며 genotype이 방향 bias를 줄 수 있다. 최소 단위의 compartment 분할 law는 hard-coded biological rule로 명시한다. 모든 세포막의 분열을 원자 역학에서 유도한다는 목표는 두지 않는다.

분할 후 원소·전하·내부 에너지·운동량·기록 매체는 두 자식에 정확히 배분한다. 복사되지 않은 template을 공짜로 두 개 만들지 않는다. host에서 immutable genome bytes를 공유하는 것은 메모리 최적화일 뿐 각 Cell의 실제 template 제작 비용은 면제하지 않는다. 기존 cell의 단순 성장·체세포 분할과 독립 계통의 생식 분기는 다른 사건으로 기록한다.

### 생식과 유전

새 생식 단위는 부모가 투자한 실제 compartment와 복사 template에서 시작한다. offspring investment는 복사·원료·작업 배분으로 표현한다. 출생 단추로 몸을 완성하거나 global spawn 위치를 선택하지 않는다. 충분히 작은 단세포는 둘로 나뉘고, 큰 몸은 떨어져 나간 살아 있는 조각이 지속될 수 있다. 보존되는 몸 일부를 “사망 처리”했다가 같은 물질을 새 개체로 다시 만드는 방식은 금지다.

Template 복사 중 mutation을 counter RNG로 적용한다. 기본 계열은 instruction/constant 치환, 발생 규칙 삽입·삭제, 발현 시간 변화, polarity 변화, module 복제·삭제·이동·재조합, 합성 graph의 국소 결합 변경이다. 원소·허용 valence를 깨는 template은 invalid 산물 또는 실행 실패를 만들 수 있지만 원료 비용과 복사 시간은 이미 수행한 만큼 남는다. 잘못된 자식을 조용히 재추첨하지 않는다.

mutation rate와 operator 비중은 유전 가능한 제한 범위의 parameter다. mutation 자체가 목적을 알고 유익한 구조를 고르지 않는다. 체세포 mutation은 해당 Cell의 template에만 반영된다. 생식 template에 실제로 복사되지 않으면 후손 전체에 전파되지 않는다. 학습한 문화 module도 물리적 기록·복사 없이 유전체에 들어가지 않는다.

### 자가 수리·해체·다세포 갈등

재생은 손실된 물질·공간·template·작업이 공급되고 기존 발생 규칙이 작동한 결과다. RegrowLimb, HealAll 같은 명령은 없다. 일부 Cell이 분할 투자를 늘려 다른 Cell의 자원을 빼앗을 수 있고, 접합·신호·자원 수송이 이를 억제할 수도 있다. 암·면역·종양은 초기 전용 system으로 넣지 않는다. 복제 갈등과 억제 관계를 관찰할 수 있는 정도의 표현 공간을 먼저 제공한다.

역사상 형태 진화 연구에서는 몸과 제어기의 구조를 함께 유전시키는 접근이 사용되었다. R7은 이 원리를 참고하지만 외부 이동 점수로 우수 개체를 뽑는 선택 방식은 채택하지 않는다. [Karl Sims, Evolving Virtual Creatures, 1994](https://www.karlsims.com/papers/siggraph94.pdf)

**경계:** W는 template 복사·오류·발생 scheduling·compartment 분할·비용이다. P는 국소 발현·복사 투자·기존 VM 조작이다. E는 몸의 대칭·분지·운동 방식·생식 전략·재생이다. U는 종·형태 family·기관·생애사 가설이다.

## 85 대사·동식물·먹이·분해의 닫힌 물질 순환

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 생태계를 미리 만든 역할로 나누지 않는다

Core에는 Plant, Animal, Herbivore, Predator, Decomposer, FoodValue, RespawnResource가 없다. 살아 있는 모든 compartment는 같은 재고·막·촉매·발생·손상 규칙을 사용한다. 광자를 흡수해 탄소를 고정하는 경로가 우세하면 관찰기가 광영양성으로, 외부 유기물을 주로 산화하면 종속영양성으로 분류한다. 두 경로를 함께 쓰는 계통도 허용한다.

식물 같은 계통은 빛에 노출된 표면과 안정된 지지 구조, 물·영양염 수송에서 이득을 얻을 수 있다. 동물 같은 계통은 접촉 힘과 감각·이동·물질 처리에 투자할 수 있다. 분해자 같은 계통은 외부 고분자를 절단하는 촉매와 흡수 면적으로 살아갈 수 있다. 이러한 결과를 보장하거나 각 계통에 독점적인 명령을 주지 않는다.

### 작업 에너지와 대사

CellWorkStore는 화학 반응으로 충전되는 생체 작업 저장고의 coarse 상태다. 최대 용량은 실제 carrier 재고×재료별 용량이다. 열이나 태양 노출 자체가 이 저장고를 직접 채우지 않는다. 허용된 결합 반응과 촉매가 필요하다. R6의 개체 공통 Ecap=65,535와 무료 기본 대사 통로는 R7에 적용하지 않는다.

저장된 work energy는 Echem의 기준 상태에 포함하지 않고 별도 Uwork로 정확히 한 번 센다. carrier가 파괴되면 실재하는 남은 chemical product와 Uwork의 열·일 배분을 남긴다. carrier 양이 줄어 용량을 초과할 때 excess를 소리 없이 clamp하지 않는다. 재충전에서 생체 일을 꺼낼 수 있는 양은 실제 반응 자유 에너지 감소와 결합 효율 이하이며, 나머지 에너지는 열 등으로 간다. 그 효율은 촉매·막·상태 함수에서 나오고 식물/동물 flag에 붙지 않는다.

VM 계산, template 복사, active transport, 조직 수축, 합성의 추가 work는 자신의 CellWorkStore에서 청구한다. 수동 확산과 압력 흐름은 상응하는 chemical/free energy를 낮추며 진행된다. 농도차를 거스르는 펌프는 carrier turnover 또는 실제 전위차가 필요하다. 원격 Cell의 저장고를 바로 차감하는 단일 몸 에너지 통은 없다.

### 기본 생지화학 경로

| 물질·에너지 흐름 | 물리·화학적 필요 조건 | 막히면 나타날 수 있는 결과 |
| --- | --- | --- |
| 빛→탄소 고정→유기물 | 흡수·반응 가능한 촉매·CO2·물·폐기물 배출 | 그늘 경쟁·광 손상·생산 감소 |
| 유기물+산화제→작업 에너지+열 | 막 수송·촉매·전자 수용체 | 산소 부족·다른 대사 경로 선택 |
| 질소·인→조직·template | 흡수 가능한 화학종과 합성 원료 | 성장 정지·재활용·지역적 niche |
| 조직→사체·분비물→작은 분자 | 절단·가수분해·산화·촉매와 물 | 유기물 퇴적·영양염 잠김 |
| mineral→용액→생체→mineral | 용해·침전·배출과 실제 수송 | 지역 고갈·침적·계통별 자원 차이 |
| 물→증발→응축→흐름 | 복사·열·상변화·압력과 유한 저장고 | 건조·침수·염 농축 |

모형에서 검증할 탄소 경로의 원소식은 CO2+H2O→CH2O+O2와 그 역방향이다. 이것은 모든 대사를 이 반응 하나로 구현하라는 지시가 아니라 최소 회계 fixture다. CH2O라는 분자식이 같은 구조를 의미하지 않으므로 실제 SpeciesGraph를 별도로 지정한다. 광합성의 합성 방향은 흡수 광자와 반응 경로를 필요로 한다. 호흡은 적절한 전자 수용체와 막·촉매를 필요로 한다.

질소고정도 N2를 무료 영양분으로 바꾸는 타이머가 아니다. fixture N2+3H2→2NH3는 원소 균형만 나타낸다. 실제 경로의 장벽·촉매·환원력·에너지 비용이 있어야 한다. 인은 원소로 영구 보존하며 암석·용액·조직·매체·침전물 사이에서 이동한다. 영양분 소비량을 단순 growth multiplier로 처리하지 않는다.

생태계의 물질 순환과 태양 유입·열 방출의 구분은 실제 생지화학에서 가져오는 핵심 원칙이다. 정확한 지구의 주기·생산량을 그대로 재현하려는 것은 아니다. [OpenStax Biology 2e — Biogeochemical Cycles](https://openstax.org/books/biology-2e/pages/46-3-biogeochemical-cycles)

### 먹기·죽음·부패·자원 이용

포식은 접촉 손상·막 파열·노출된 물질의 이동·분해·흡수의 결과다. take 명령이 살아 있는 몸의 비노출 내부 재고를 훔치지 못한다. 부리·이빨처럼 보이는 구조가 있다면 실제 모양·stress·마찰·절단 작용으로 효율 차이가 나야 한다. food enum을 읽어 energy를 더하는 shortcut은 없다.

죽음은 실행 기구·막·template의 생존 조건이 깨져 해당 Cell의 유료 제어가 멈추는 사건이다. 남은 화학종·저장된 일·열·전하·몸·기록은 world에 남는다. 죽으면 자동으로 영양분 N개를 생성하지 않는다. 분해는 살아 있는 촉매의 작용과 비생물 반응 양쪽에서 일어나며, 건조·산소·온도·용매·원료에 따라 느려지거나 다른 산물을 만들 수 있다.

베어진 줄기 같은 구조는 실제 섬유·수분·고분자 재료다. 말리기·태우기·절단·접합·운반에 같은 재료 규칙을 적용한다. 식량, 목재, 섬유, 기름, 독, 약이라는 이름은 UI 해석이다. 그 물질의 분자·표면 상태가 실제로 조직을 손상시키거나 특정 반응을 억제할 때만 독성 효과가 있다. 종별 전용 damage multiplier는 없다.

서식지 변경도 연쇄적인 물질 변화다. 생체가 빛을 가리고, 토양 물을 흡수하고, 유기물을 남기고, 유로를 바꾸면서 다른 계통의 niche가 달라진다. 생물을 보호·이동·번식시키는 반복 행동이 출현하면 관찰기가 재배·사육 후보라고 이름 붙일 수 있다. PlantCrop, TameAnimal, FarmBonus는 추가하지 않는다.

### 초기 조건과 멸종

완성 세계 profile에는 공개된 세 종류의 **손작성 조상 template fixture**를 준비한다. 광 흡수와 탄소 고정이 가능한 작은 고착성 조상, 외부 유기물 흡수와 약한 수축을 가진 조상, 외부 절단 촉매를 방출하는 조상이다. 이들은 plant/animal/decomposer enum이 아니라 동일 genome 포맷의 seed다. 전체 bytes·초기 물질·유전 차이·삽입 위치·개체 수를 manifest에 기록한다. 이런 생활 양식을 시작부터 준 사실을 자연 발명 성과로 세지 않는다.

별도 연구 profile은 한 조상 또는 더 제한된 대사에서 시작할 수 있다. 두 profile의 성과를 섞지 않는다. 신규 niche를 보이게 하려고 운영 중 종을 몰래 재주입하지 않는다. 전멸하면 그 실행은 전멸로 끝난다. 사용자 개입 복원은 명시적 branch다.

토양과 수계는 generic StockOwner다. nutrient fertility는 실제 용존/광물 N·P, 물, pH, 유기물, 접근성에서 계산한 관찰량이다. WorldGen이 매 tick 지정한 fertility만큼 자원을 생성하지 않는다. 대기·토양·물·암석·생체·기록·사체를 합친 원소 장부가 닫혀야 한다. 열린 경계라면 들어오고 나간 물질을 명시적 외부 ledger로 기록한다.

**경계:** W는 흡수·반응·성장·손상·분해·물질 순환이다. P는 국소 힘·물질·발현·복사·기록이다. E는 먹이망·공생·기생·재배·사육·자원 관리다. U는 식물·동물·분해자·생태 자원·영양 단계다.

## 86 자기·유도: 기하에서 생기는 장과 역기전력

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 모델 선택과 한계

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| Coil/Motor/Generator prefab | 매우 빠름, 예제 제작 쉬움 | 권선·토크·변환 기능을 개발자가 제공 | 제외 |
| 전 세계 Maxwell FDTD | 전파·간섭을 포함한 넓은 표현 | 파장·시간 step과 물질 운동의 척도 차이, 매우 큰 메모리 | 기준 제품에서 제외 |
| 기하 기반 준정적 전도·유도·자화 | 물질 배치에서 회로·자력·발전 경로를 만들 수 있음 | 긴 거리 결합·topology 변화·수치 보존을 별도 해결 | **선택** |

도체 material site들의 실제 접촉과 길이·단면에서 node/branch graph를 만든다. branch는 솔버 변수이며 agent에게 제공하는 인덕터 부품이 아니다. 전하 축적과 용량은75절, 저항은 재료·길이·단면·온도에서 유도한다. 자기 결합은 유한 두께를 가진 도체 경로의 기하 kernel에서 계산한다. 주어진 회로 diagram에 L값을 사용자가 손으로 넣는 방식은 witness 외에는 금지다.

기계 공간은 기존 2D 위치·회전과 두께1의 slab를 유지한다. 자기장 방향·자화는3성분을 가질 수 있지만 물체가 층을 건너거나 3D 코일로 자동 변환되지는 않는다. 평면 도체 loop와 자기 분포 변화는 표현할 수 있고, 실제 솔레노이드·적층 권선·배선 교차는 이 기하의 제한을 받는다. 부피가 없는 전선 교차나 원격 포트로 그 한계를 숨기지 않는다.

준정적 근사는 전자기 전파 시간을 생략한다. 배선의 신호 지연은 R/L/C와 연결망에서 생기지만 무선 전파·안테나·빛의 파동성과 동일한 모델은 아니다. 광학은87절의 별도 에너지 수송 근사다. 두 대역의 결합은 실제 흡수·방출·반응에서만 일어나고 전자기 에너지를 이중 계상하지 않는다.

### 보존형 상태 방정식

상태는 node charge Q, branch flux linkage λ, 자성 site의 자화 m, 물체 geometry x다. 전류 I는 독립 재고로 중복 저장하지 않고 I=∂U/∂λ에서 계산한다. reference 선형 영역의 에너지 함수는 U=Uelectric(Q,x)+½(λ−Φm(x,m))^T L(x)^(-1)(λ−Φm(x,m))+Um(m,x)다. L은 동일 단위계의 대칭 양의 정부호 행렬이어야 한다.

Um은 자화의 local 안정성·anisotropy·demagnetization 및 자기 상호작용 에너지를 포함한다. Φm과 Um, L은 서로 무관한 힘 보정값으로 정의하지 않는다. 같은 기하 kernel과 constitutive data에서 유도한다. 비선형 saturation에서는 해당 free-energy 함수의 미분으로 I·힘을 구하며 선형 식에 임의 saturation clamp를 덧붙이지 않는다.

incidence matrix B를 도체 branch 방향과 일치시켜 dQ/dt=−BI, dλ/dt=B^T φ−RI+ηchem으로 둔다. φ=∂U/∂Q, I=∂U/∂λ다. 그러면 전기·자기 내부 교환의 교차 항이 상쇄되고 저항 손실은 I^T RI≥0이다. ηchem이 일을 공급하면 해당 전극 반응의 chemical/free energy가 감소해야 한다. 지면 전위0이라는 reference는 전원이 아니다.

기계적 힘은 동일 U의 Q·λ·m과 열역학적 entropy 상태를 고정한 geometry 미분 F=−∂U/∂x, torque=−∂U/∂angle로 계산한다. 움직이는 자석·도체의 유도 전압과 역방향 힘은 이 결합에서 같이 나와야 한다. 등온 물성표를 사용하는 구현은 대응하는 free-energy potential에서 같은 힘을 구하고 열 교환까지 계산해야 한다. 전류를 만들면서 기계적 저항을 빼먹는 해법은 통과시키지 않는다. 자기력의 에너지·co-energy 관점은 기존 다중 물리 도구에도 쓰이지만, 그 도구의 완성 actuator를 agent 부품으로 가져오지는 않는다. [Modelica Magnetic UsersGuide](https://doc.modelica.org/Modelica%204.1.0/Resources/helpDymola/Modelica_Magnetic_FluxTubes_UsersGuide.html)

### 자성 재료와 자기 기억

자화는 모든 물질에 기본 제공하지 않는다. 원소·배열·온도·domain 상태의 constitutive 함수가 자화 가능한 범위를 정한다. 기준 모델은 유계 자화 magnitude, 쉬운 축, 유한 장벽과 비가역 relaxation을 가진 coarse domain이다. dm/dt=−Γ∂U/∂m 형태의 소산 항은 Γ가 양의 준정부호일 때 줄어든 에너지를 열로 전달한다. 안정점 사이 장벽은 이력과 기억을 만들 수 있다.

가열·냉각·외부장·기계적 회전으로 자화 상태가 바뀐다. Curie 유사 전이는 온도 의존 constitutive 데이터로 표현하되 실제 합금의 온도를 예측한다고 표시하지 않는다. 영구자석을 만드는 과정, 잔류 에너지, 초기 world의 자화 모두 시작 장부에 들어간다. SetMagnet, MagnetizeInstant, GeneratePower는 없다.

기본 reference는 고정된 noise-free constitutive dynamics로 시작한다. 열 요동을 추가하려면 fluctuation·energy exchange와 RNG stream을 함께 규정한 다른 profile이 필요하다. 자화 방향을 난수로 바꾸고 에너지 차이를 무시하지 않는다.

### kernel·토폴로지·해법 계약

기하 kernel은 유한 단면에서 self term이 발산하지 않도록 정규화한 Coulomb형 적분을 사용한다. 같은 basis로 self/mutual 항을 계산하고 reciprocal pair를 한 번만 소유한다. 단순 거리 cutoff로 mutual 항을 지웠을 때 L의 양의 정부호나 힘의 연속성이 깨지면 그 최적화는 허용하지 않는다. 정확한 kernel 적분·자화 에너지 데이터·길이 단위는 동결 전 gate다.

reference는 작은 domain에서 dense 대칭 행렬과 결정론 linear solve를 사용한다. 확장 구현은 동일 에너지 함수를 근사하는 계층 행렬 또는 matrix-free 해법을 후보로 둔다. 근사 에너지·힘 미분·대칭성·양의 정부호·오차를 함께 검증해야 한다. thread별 임의 pair truncation이나 역기전력을 생략한 빠른 경로는 없다.

도체 절단 시 저장된 ½LI²를 삭제하지 않는다. 벌어진 gap의 전하 축적·capacitance·누설·절연 파괴·주변장의 과도 상태로 넘어가야 한다. 새 graph에 Q·λ를 투영할 때 최소 에너지 projection만 수행하고 나머지를 소리 없이 없애는 것도 금지다. reference는 변형되는 도체·gap의 상태를 유지하고, field 잔여량이 물리적 경로로 해소된 후 graph를 축약한다. gap law와 topology transfer의 수치 상세는 X21 gate를 통과하기 전 미동결이다.

자기 domain을 카메라 범위로 잘라서는 안 된다. 긴 도체나 가까운 자석이 거대한 coupling domain을 만들 수 있다. 해당 비용은90·91절 workload에 포함하고, 부족하면 world를 정지·저장한다. 물리 결과가 달라지는 근사 변경은 새 manifest에서만 한다.

**경계:** W는 전하·자속·자화·저항·힘·열의 에너지 결합이다. P는 실제 물질 배치·접촉·열·힘이다. E는 유도 결합·전자기 구동·발전·자기 기억이다. U는 코일·변압기·모터·발전기 후보와 측정된 성능이다.

## 87 광학: 실제 에너지를 운반하는 빛

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 표현 선택

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| tile별 조명 scalar | 빠르고 생태 생산량 조절 쉬움 | 반사·굴절·집광·광학 신호의 경로가 없음 | 최종 물리에서 제외 |
| 파동 FDTD | 회절·간섭·분산의 폭넓은 표현 | 공간·시간 해상도 부담과 전 세계 실행 비용 | 향후 별도 연구 |
| 스펙트럼 packet과 기하 광학 | 렌즈·거울·차폐·흡수·광반응의 조합 가능 | 각도·표면 해상도 제한, packet 관리와 열 결합 필요 | **선택** |

Meep 같은 FDTD 해법은 장의 시공간 격자와 재료의 분산·손실 상태를 해석한다. 그 정도의 파동 모델을 생태 세계 전체에 비용 없이 넣을 수 있다고 가정하지 않는다. R7은 위상의 없는 기하 광학을 택한다. [Meep Introduction](https://meep.readthedocs.io/en/latest/Introduction/) · [Meep Materials](https://meep.readthedocs.io/en/latest/Materials/)

### 2D 세계와 광선의 공간

물체의 기계 운동은2D지만 실제 점유 형상은 두께1의 slab로 광학에 노출한다. 광선은 이 slab 안에서3성분 방향을 가지며 위·아래·옆 표면과 만날 수 있다. 별도의3D 몸 배치 권한은 생기지 않는다. 위에서 들어오는 햇빛은 해당 column의 첫 실제 표면에서 반사·흡수·투과한다. 통과한 빛은 바닥·액체·다른 표면을 만날 때까지 남는다.

태양은 상부 경계의 방향·스펙트럼·시간별 유입량으로 정의한다. 한 tile의 태양 credit과 photon packet을 동시에 지급하지 않는다. 밤에는 그 경계의 유입이0이고, 이미 존재하는 복사·열·저장된 chemical energy는 그대로 진행된다. column visibility cache는 geometry·흡수·굴절 상태가 변할 때 무효화한다. 카메라·프레임 수는 cache와 에너지 유입에 관여하지 않는다.

기준 전파 속도 c_model은32 tile/tick인 인공 시간척도다. 매질 내 속도는 c_model/n으로 근사한다. 현실 빛의 속도를 재현한다는 표기를 하지 않는다. 상부 slab를 통과하는 직접광을 quasi-steady하게 적분하는 fast path는 매 tick 상태가 같은 기준 solver와 에너지·차폐 오차 gate를 통과해야 한다. 통신용 가변 광원 packet은 이 fast path로 즉시 원격에 전달하지 않는다.

### 스펙트럼과 표면

기준은6개 band다. 대표 파장과 단위 변환 h·c, 열 방출 적분을 OpticalData-ML1에 명시한다. 후보 band 경계는 모형 μm 단위로0,0.3,0.5,0.6,0.8,3,∞이며 대표 파장은0.2,0.45,0.55,0.68,1.5,10이다. 이 거친 묶음은 실제 분광학·색각을 재현하기 위한 값이 아니다. 실제 photon energy의 모델 q 변환 상수는 manifest에 별도로 동결한다.

표면은 band별 굴절률 n≥1, 흡수계수 α≥0, 거칠기, 광화학 반응 coupling을 가진다. 조성·phase·온도·구조에서 값이 나오며 색깔 이름으로 특수 기능을 주지 않는다. Beer–Lambert형 전파에서는 통과 길이 d에 대해 Eout=Ein exp(−αd), 차이는 실제 매질에 흡수된다. 그 차이를 동시에 열과 반응 에너지로 두 번 더하지 않는다.

매질 경계에서는 Snell 법칙과 비편광 Fresnel 평균을 사용한다. 전반사 조건이면 투과분은0이다. 거친 표면은 공개된 정규화 angular kernel로 재분배한다. 각 hit에서 Ereflected+Etransmitted+Eabsorbed=Ein을 만족해야 한다. 여러 거울을 통과하거나 굴절률이 높아졌다는 이유로 packet 에너지가 커지지 않는다.

기준 profile은 복사압·Doppler·편광·간섭·회절을 생략한다. 따라서 광선의 운동량으로 추진하는 장치를 지원한다고 주장하지 않는다. 이동 표면에서의 기하적 반사와 에너지 보존은 처리하지만 상대론적인 광기계 결합을 제공하지 않는다. 이는 명시적 모델 제한이며 마찰이나 반응의 에너지 누수를 허용하는 예외가 아니다.

### 광자 에너지·열·화학의 소유권

OpticalPacket은 EnergyAmount, band, position, direction bin, arrival subtime, medium/contact revision을 가진다. 한 packet은 많은 photon을 대표하는 에너지 묶음이다. 광화학 반응은 band가 활성화를 허용하고 해당 촉매·반응물이 있을 때만 extent를 얻는다. 흡수 직후 진행되지 않은 들뜬 상태는 실제 species excitation state로 남거나 정해진 relaxation으로 열·방출광이 된다. 광반응에는 대응하는 역방향·방출 경로와 복사 평형 검사가 필요하다. 대사라는 이름만으로 이를 면제하면 열복사에서 일방적으로 일을 뽑는 허점이 생기므로 금지한다.

열 복사는 해당 온도·band의 흡수율과 일치하는 방출 law를 사용한다. 방출할 때 재료 Uthermal에서 실제로 빼고 새 packet에 넣는다. 대기/외부로 나간 에너지는 boundary ledger에 기록한다. 거울·창·복사 차폐가 한 온도의 닫힌 환경에서 지속적인 일을 만들면 실패다. 정적 색상 render를 바꾸어 thermal emission이 바뀌지 않는다.

생체의 빛 감각도 흡수에 따른 국소 변화에서 나온다. 빛을 전기·화학 신호로 바꾸는 매체에는 변환 경로·반응·손실이 필요하다. EMIT 광 통신을 선택하면 자기 작업 저장고 또는 반응 에너지로 실제 packet을 만든다. 수신자가 원래 payload를 마법처럼 복원하지 않고 국소 band별 흡수량·시간 변화를 읽는다.

### packet 수와 해상도

기준 각도 집합은 azimuth96×수직 방향8개다. 수직 입사와 수평 전파를 포함하도록 direction table의 실제 벡터를 manifest에 저장한다. 반사·굴절 방향을 가장 가까운 bin 하나로 무조건 반올림하지 말고 인접 방향에 비음수 weight로 분배한다. energy의 정수 나머지는 canonical branch 순서로 배분하고 잔차 accumulator를 보존한다.

같은 leaf·entry face·subtime·band·direction bin의 packet만 합칠 수 있다. 이 키를 같게 만드는 공간·시간 양자화 자체가 공개된 광학 모델이다. 서로 다른 도착 시간을 합쳐 신호를 앞당기거나, 저에너지 packet을 삭제해 비용을 낮추지 않는다. 오래 살아 있는 packet에도 물리적 흡수·외부 유출 외의 임의 TTL은 없다.

packet pool 한도는 host 운영 한도다. 한도에 닿으면 checkpoint 후 정지하거나 실행 전 선택한 더 낮은 해상도의 manifest로 새 branch를 만든다. 현재 tick에서 덜 보이는 광선을 제거하지 않는다. 각도 수를2배, geometry 해상도를2배로 높인 작은 reference와 비교해 집광 위치·흡수량·열 평형·통신 지연 오차를 평가한다. 기준 허용치는 witness별로 지정하며 아직 실측 전이다.

**경계:** W는 복사 유입·전파·표면·흡수·방출·광반응이다. P는 물질 형상·배치·에너지 사용과 국소 광 readout이다. E는 그늘·집광·광열 가공·광학 신호·광 반응 제어다. U는 렌즈·거울·광 수신기·집광 장치 후보와 빛 경로다.

## 88 교차 결합과 하나의 보존 장부

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

새 system을 독립 미니게임으로 붙이지 않는다. 광학이 만든 열은73절의 열 상태를 바꾸고, 화학 반응이 만든 기체는 같은 압력을 만들며, 조직 성장과 파괴는 같은 질량·운동량·점유 규칙을 사용한다. 자기 손실은 재료를 가열하고, 열·화학 변화는 전도·자화·광학·막 투과도에 다시 영향을 준다.

### R7 총 에너지

Etot=Echem+Uthermal+Uwork+Klinear+Kangular+Ug+Uelastic+Ufluid_compression+Kfluid+Uelectric+Umagnetic+Usurface+Eoptical다. 같은 물질에 속한 phase 잠열·excitation·막 전위는 각각 선언된 항 한 곳에서만 센다. Uwork는85절의 carrier 기준 상태 위에 저장된 부분이다. Umagnetic은86절 식의 유도·자화 항 전체이며 별도로½LI²를 더하지 않는다.

기존71절의 agent/embryo energy는 CellWorkStore로 대체한다. photo credit는 packet 또는 excitation chemical state로 대체한다. 살아 있는 몸·배아·사체라는 해석이 바뀌어도 physical owner의 Etot는 끊어지지 않는다. 생물 birth/death 때문에 에너지가 생성되거나 사라지면 실패다.

원소 장부는 전체8종에 대해 정확하고, 순 전하는 실제 charge ledger에서 정확하다. chemical amount는1/65,536 모형 unit 단위 u64로 유지하되 checked wide intermediate를 쓴다. EnergyAmount는1/65,536 q 단위 정수와 수치항 f64를 구별한다. 작은 교환을 매번0으로 절삭하지 않도록 edge/reaction/optical remainder를 상태에 둔다.

free energy의 온도 의존 항으로 힘·반응을 구할 때 internal energy와 entropy도 같은 constitutive potential에서 유도한다. 예를 들어 자성의 온도 의존 coefficient를 바꾸고 Um 차이를 없애거나, 상변화에서 잠열 없이 투과도만 바꾸는 것은 금지다. 물성 모듈은 평가값뿐 아니라 필요한 미분과 유효 영역을 반환해야 한다.

### 서로 연결되는 최소 경로

| 연결 | 실제 중간 상태 | 금지되는 shortcut |
| --- | --- | --- |
| 빛→생체 재료→건설 | 흡수·반응·원료·조직·절단·운반 | 햇빛 tick마다 wood 아이템 지급 |
| 빛→열→압력→회전 | 실제 U·기체·경계 work·접촉 | solar engine 부품의 고정 출력 |
| 회전→자속 변화→전류→열 | geometry·λ·Q·저항·역토크 | 회전속도×보너스로 power 생성 |
| 화학→전위→조직 수축 | 반응·carrier/전하·유료 strain | food 점수를 speed로 변환 |
| 조직 분비→새 재료→광학 | 실제 고분자·표면·n·α | 눈/렌즈 gene 하나로 원격 시야 제공 |
| 생물 잔해→촉매 반응→영양염 | 고분자 절단·산화·확산 | 사망 보상으로 nutrient 생성 |
| 광/전기 신호→국소 제어 | 실제 흡수/전위·transduction·지연 | 수신자가 payload·발신자 ID를 자동 해독 |

### 에너지 공급과 열의 방향

같은 원소 집합에서도 태양 유입과 외부 열 방출이 있으면 비평형 상태를 유지할 수 있다. 외부 에너지 유입을 제거한 닫힌 세계가 같은 생산량을 영원히 유지해서는 안 된다. 반대로 물질 총량이 유한하다는 이유로 순환 가능한 영양분을 매 소비마다 영구 삭제해서도 안 된다.

에너지 보존만으로 충분하지 않다. 열역학적 구동력과 수송 방향, detailed balance, 비가역 손실의 비음성도 검사한다. 한 열원에서 아무 변화 없이 계속 일을 뽑는 촉매/광학/자기 cycle은 회계가 맞아 보여도 실패다. 각 coupling은 고립 계, 정적 평형, 닫힌 cycle, 외부 구동 cycle을 각각 검사한다.

### 수치 오류와 물리적 반응의 분리

수치 residual을 조직의 먹이·열·mutation 자원으로 지급하지 않는다. 유한 오차는 NumericResidual ledger에 별도 표시하고 승인 한도를 넘으면 substep 축소 또는 rollback한다. 원소·전하 오류는 허용 오차 없이 실패다. 열·일·장 에너지 오차 한도는71절의 후보와 추가 stiff case로 재승인한다.

호스트 계산이 부족한 경우 reaction rate를 조용히 낮추거나 생물 일부만 maintenance를 면제하지 않는다. 생태적 비용은 모델 규칙, CPU wall time은 구현 비용이다. 두 종류의 비용을 바꾸어 실행이 빠른 유전자가 더 많은 tick을 받게 만들지 않는다.

**경계:** W는 보존·열역학·결합·오차 처리다. P는 이 경로를 이용하는 국소 조작이다. E는 여러 매체를 엮는 생존·생산·제어 방식이다. U는 에너지 흐름·효율·작동 원인의 증거다.

## 89 Primitive·VM·문화와 새 몸의 접점

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 남기는 primitive와 바꾸는 의미

새 세계에서도 Walk, Attack, GrowLeg, Photosynthesize, EatPlant, BuildGenerator, MakeLens, Farm, Domesticate, Invent는 host call로 존재하지 않는다. 광영양·대사는 실제 반응이고, 기관 발생은 국소 합성·기하 변화다. 특정 이름의 유용한 결과를 직접 호출할 수 없다.

| service | R7의 저수준 계약 | 접근·비용 경계 |
| --- | --- | --- |
| READ | 자기 또는 노출된 국소 표면의 유료 감각 readout | 감각 재료·면적·차폐·분해능·거리 제한 |
| DRIVE | 자기 활성 조직의 국소 strain/force 요청 | 실제 연결·단면·work·역작용 |
| TRANSFER | 접촉한 재고/노출 물질의 실제 이동 | 막·부피·접근성·운동량·전하·열 보존 |
| LINK / UNLINK | 실제 재료로 결합·절단 | 접촉·앵커·유한 강도·접합 물질 |
| WORK | 국소 압축·마모·가열 등의 에너지 전달 | 접촉·실제 작업량·수용 능력 |
| WRITE / COPY | 접근 가능한 매체의 word 변경·복사 | 물리 backing·시간·에너지·출처 |
| EXPRESS | 자기 Cell의 합성/수송/조직 활성 목표 설정 | 있는 template·촉매·원료 범위, 즉시 효과 없음 |
| REPRO_CONTROL | 자기 template 복사·분할에 대한 투자 한도 | 생식 단위 실제 형성, spawn 불가 |
| EMIT | 빛 packet 방출 또는 실제 분자의 외부 이동 | 광원/분비 매체·실재 원료·에너지 |

EMIT는 R6의 추상적인 signal field에서 교체한다. 광 mode의 channel은 허용 band, value는 방출 시간·강도에 영향을 주는 raw 명령이다. chemical mode는 보유한 분자의 local handle과 실제 amount다. 빛·용질은 각 전파 법칙을 따르고 원래 숫자 payload가 world field에 그대로 저장되지 않는다. received resource_location 같은 의미는 VM이 만든다.

READ에는 자기 밖 대상의 GenomeID·LineageID·식물 분류·상대 잔여 에너지·문명 ID를 제공하지 않는다. 공개 매체를 실제로 검사·복사하면 code word를 얻을 수 있다. 화학 감각은 센서의 측정값·fingerprint를 주며 전역 SpeciesKey 목록과 숨은 분자 구조를 무료 열람하게 하지 않는다. 합성 template의 graph는 자기 물리적 기록에서 읽을 수 있다.

### 국소 주소와 ABI-ML1

VM instruction encoding은 기존 register machine 형식을 계승하되 ProgramHeader의 world_abi가 ABI-ML1을 명시해야 한다. legacy ABI의 같은 숫자를 새 의미로 해석하지 않는다. opcode 실행·정수 overflow·fault latch·query4/effect1 제한은 재사용하고 외부 service argument layout은 아래와 같이 별도로 동결한다.

service 입력은 r0=service id, r1=target slot, r2=field/subcommand, r3=local material/template selector, r4..r5=value payload, r6..r7=amount payload다. 32bit payload는 low16/high16 순서다. r14는 status, r15는 작은 결과 길이 또는 완료량이며 큰 결과는 사전에 지정한 자기 scratch span에 기록한다. 출력 scratch는 자신의 실제 기억 backing을 요구한다. 출력 span이 부족하면 부분 구조체를 쓰지 않고 BufferTooSmall을 반환한다.

target slot은0=self Cell, 1..15=현재 감각 frame에서 얻은 국소 handle이다. handle에는 runtime type, generation, geometry revision, sensing tick이 들어가며 그 내용은 VM의 world UID가 아니다. 다른 Cell의 내부 handle을 정수로 추측해 접근할 수 없다. request와 commit 사이 표면이 변하면 StaleTarget이며 action activation 비용만 청구하고 미사용 work escrow를 반환한다.

새 READ field는 local temperature, pressure, strain, surface potential, band absorption, accessible stock amount, template word, contact geometry, self work/capacity다. 정확한 단위·부호·saturation·센서 noise·fingerprint mapping은 SensorData-ML1로 묶는다. 이 표는 host service 구조 결정이며 모든 wire id/scale이 완료되었다는 표시는 아니다. legacy sensor와 혼용을 막는 golden vector가 필요하다.

새 status는 MissingTransducer, NoBacking, InsufficientMatter, InsufficientWork, NoVolume, StaleTarget, InvalidTemplate, BufferTooSmall, PhysicsRetryAbort다. host memory 부족은 agent status로 돌려주지 않고 RunHalt로 처리한다. 그래야 host의 임의 한도가 생물의 선택압이 되지 않는다.

### Controller와 문화의 경계

Controller는 실제 Cell에 붙는다. 다세포 몸 전체에 무료 공용 r0..r15나 공유 inventory를 만들지 않는다. 여러 controller 사이 전달은 전기·화학·광학 경로 또는 실제 기록·매체 복사를 통해 이루어진다. 같은 계통의 Cell끼리 통신 비용을 자동 면제하지 않는다.

형태가 바뀌면 사용 가능한 감각·작업 위치도 바뀐다. 불가능한 오래된 module call은 일반 실패를 반환한다. module 이식은 code를 받은 사실과 새 몸에서 유용하게 실행한 사실을 따로 기록한다. 문화 학습은 기본적으로 BehaviorProgram/발현 목표를 바꾸며, germline 변화는 실제 생식 template에 복사된 때만 기록한다.

생체 합성 절차도 지식이 될 수 있다. 기록된 합성 template을 다른 계통이 가져가 실제 원료·촉매·몸 조건을 갖추었을 때 유사 재료를 만들 수 있다. 기록을 읽는 것만으로 물성이 unlock되지 않는다. 계통마다 도구·광학·대사 차이가 있어 같은 지식을 즉시 실행하지 못할 수 있다.

**경계:** W는 국소 권한·감각 매체·비용·호환 ABI다. P는 표의 host service와 VM 내부 연산이다. E는 통신 언어·몸의 협응·학습·기술 전달이다. U는 절차·신호·형태와 문화의 연결을 보여준다.

## 90 Rust 데이터 소유권·phase·저장 형식

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### crate/module 구성

civ_types는 Key·Amount·EnergyAmount·geometry와 schema DTO를, civ_numeric은 결정론 수학·checked 산술·canonical reduction을 소유한다. civ_vm은 VM 해석만 한다. 새 civ_chem은 species canonicalization, rewrite, thermodynamics, kinetics, material evaluation을 담당한다. 새 civ_bio는 compartment, development, synthesis, template copying, controller binding을 담당한다. civ_physics는 mechanics/fluids/thermal/electric에 magnetics, optics, coupling, audit를 추가한다.

의존 방향은 공통 types/numeric→chem·vm·physics, chem·vm→bio, 이 라이브러리들→core→runner다. observer와 viewer는 public snapshot/event schema를 읽는다. chemistry가 core를 역참조하거나 physics가 생물의 직업·문명 분석을 호출하지 않는다. MaterialSample은 순수 평가값과 유효 범위·미분을 제공해 chem↔physics의 순환 소유권을 막는다.

civ_core::MatterLifeWorld만 authoritative store를 소유한다. subsystem은 phase별 제한된 view와 staging writer를 받는다. 서로 다른 crate의 전역 singleton에 살아 있는 material 또는 에너지를 따로 저장하지 않는다. observer는 world에 write handle을 갖지 않는다. Bevy ECS는 render proxy와 UI state만 소유한다.

### 핵심 struct 계약

| type | 핵심 필드·의미 | 단독 owner |
| --- | --- | --- |
| MatterLifeWorld | manifest, tick, species, stocks, bodies, cells, fields, ledgers | core |
| SpeciesDef | canonical graph, elements[8], charge, model hash, thermo terms | SpeciesRegistry |
| StockSpan | common amount block, sparse pair range, phase, volume/thermal owner | StockStore |
| ReactionState | owner/face key, rule mapping, extent remainder, occupied catalyst | ReactionStore |
| CellState | stock key, membrane span, scaffold span, template key, dev state, work store | CellStore |
| ControllerState | cell key, VM state, sense frame, pending intent, paid backing | ControllerStore |
| TemplateRecord | behavior/dev/synthesis hashes, mutation policy, physical backing ref | TemplateStore |
| DevelopmentState | polarity, expression targets, copy progress, split progress, due tick | CellStore |
| ConductionGraph | stable geometric node/branch keys, incidence, material revisions | PhysicsStore |
| MagneticState | charge refs, flux span, magnetization span, constitutive revision | FieldStore |
| OpticalPacket | energy, band, pose, direction bin, arrival subtime, medium revision | OpticalStore |
| BoundaryLedger | element/charge/energy import and export by boundary kind | AuditStore |
| TransactionJournal | state pages, allocations, pending definitions, remainders, RNG counters | Runner checkpoint |
| OrganismView | physical connectivity, lineage mixture, inferred roles, confidence | observer only |

메모리의 repr(Rust) layout을 save wire로 그대로 쓰지 않는다. Key는 slot:u32와 generation:u32를 기본으로 하되 long-run UID는 u64 독립 counter다. generation rollover는 재사용 전에 검출한다. 최종 field width와 padding은 size_of 측정 뒤 고정하며91절 수치는 예상 payload다.

핵심 enum은 PhaseKind, MaterialPhase, ReactionFamily, FieldDomain, SensorField, MutationKind, HandleKind, IntentKind, RunHaltReason이다. OrganKind, DietClass, CivilizationMembership, TechnologyLevel, MachineKind는 코어 enum에 없다. observer는 관찰 label을 확장 가능하게 저장할 수 있지만 물리 분기 조건으로 사용하지 않는다.

### 한 tick의 실행 순서

1. Preflight: 직전 commit의 snapshot을 기준으로 due work와 page reserve를 준비한다. boundary 입력·태양 schedule·사용자 개입을 고정하고 journal을 연다. 실제 입력 에너지는 이후 trial 단계에서 한 번만 넣는다.
2. Sense/Think/Develop: controller query와 발생 규칙을 읽기 snapshot에서 실행한다. 계산 비용·발현 목표·intent를 tentative state에 기록한다. controller UID·cell UID 순서의 결정론 비용 처리를 사용한다.
3. Resolve intents: 국소 접근·stock·work·volume을 예약한다. 물질 이동·성장·결합·분할은 즉시 world를 완성 상태로 바꾸지 않고 물리 transaction으로 만든다. 동일 원료를 여러 효과가 중복 소비하지 못한다.
4. Build trial domains: geometry/contact/transport/conduction/optical boundary index를 갱신하고 coupled dependency graph를 만든다. 새 chemical species 후보는 canonical bytes를 가진 pending 정의로 사용한다.
5. Coupled substeps: 반 단계 chemical/thermal 처리, 광 전파·흡수와 field/pressure/contact의 결합, 나머지 반 단계 chemical/thermal 처리 순으로 진행한다. 성장·active transport·strain 작업도 같은 work ledger 안에 들어간다. 반복 solve는 동일 substep 출발점에서 수렴시키며 입력을 반복 지급하지 않는다.
6. Audit/retry: 원소·전하·비관통·자유 에너지 방향·에너지 residual을 검사한다. 실패 시 해당 coupling domain의 trial state와 incident photon/flux/reaction remainder까지 복구하고 더 작은 substep으로 재시도한다. domain 경계를 넘어 이미 commit한 효과가 있으면 tick 전체를 rollback한다.
7. Viability/copy/receipt: 충분히 진행된 template copy와 compartment split을 확정하고 실행 기구 손상을 반영한다. birth/death는 실제로 남은 물질 상태에 대한 사건이다. 불필요한 escrow를 되돌리고 성공/실패 receipt를 만든다.
8. Commit: 새 species는 canonical bytes 순서, 새 Cell/Body/Controller는 원인 event key 순서로 ID를 배정한다. checksum·필수 event·snapshot을 완결 tick으로 발행한다. 실패한 tick의 신규 정의·counter 증가·photon·RNG 소비가 다음 tick에 남지 않는다.

새 화학종이 tick 중 여러 단계 반응에 사용되면 pending key로 일관되게 참조한다. commit에서 부여할 runtime index를 물성·반응 난수·우선순위 계산에 쓰지 않는다. 사전 reserve가 부족해 추가 할당할 때도 journal로 되돌릴 수 있어야 한다. 메모리 부족으로 특정 후보만 취소하지 않는다.

### threading과 결정론

독립 Cell의 VM·발생 평가, 독립 stock의 후보 생성, 독립 physics domain은 병렬 처리한다. 경계 flux·광 packet·mutual magnetic pair는 canonical owner가 한 번만 계산하고 destination accumulator를 정렬 reduction한다. work-stealing의 완료 순서는 결과 순서가 아니다.

자기 coupling과 장거리 광 경로는 기존 chunk 독립성을 깨뜨릴 수 있다. chunk는 저장·공간 색인 단위이고 항상 독립 물리 단위인 것은 아니다. reference magnetic solve는 domain 전체를 일관되게 처리한다. domain 분할 최적화는 실제 에너지 함수를 바꾸지 않거나 명시적 근사 오차 gate를 통과해야 한다.

복제 RNG key는 run seed, tick, source CellUID, copy event ordinal, mutation kind, mutation ordinal이다. retry 횟수와 worker 번호는 key에 들어가지 않는다. 물리 reference는 deterministic continuous/extent 방식이며 임의 stochastic 반응을 섞지 않는다. stochastic 확장을 쓸 때는 분할 시간과 사건 stream에 대한 별도 저장 계약이 필요하다.

### save/replay

ML1 파일 magic은8byte CIVML001이며 Grid-5/Matter-1 reader와 분리한다. header는 schema_major/minor, endian marker, manifest hash, material/chemistry/optical model hash, numeric backend hash, seed, committed tick, directory offset을 가진다. section directory는 kind/version/count/uncompressed length/compression/digest/offset이다. 각 vector에는 length와 검증 가능한 최대값을 둔다.

저장에는 species canonical definitions, 모든 stock/phase, shape/bond, Cell·controller·template backing, 발생·복사·분할 진행, Q·λ·m, optical packet의 도착 시간, reaction/transport/rounding remainder, boundary ledger, UID와 RNG event counter, 수치 refinement plan을 포함한다. future에 영향을 주지 않는 factorization·render cache는 재생성할 수 있다. 새로 계산한 cache가 결과를 바꾸면 그것은 cache가 아니라 저장 대상 상태다.

끝난 tick에서만 저장하고, 불완전 journal은 committed checkpoint와 재실행 가능한 입력으로 복구한다. 예전 save를 열면 해당 legacy backend로 replay한다. R7으로 변환하면 초기 조건을 다시 만드는 intervention branch이며 같은 자연사의 연속으로 표시하지 않는다. 새로운 물성 파일을 같은 hash로 덮어쓰지 않는다.

**경계:** Engineering Contract는 crate·store·borrow·phase·thread·wire·rollback이다. W/P는 이 실행 계약에서만 상태를 바꾼다. E는 host scheduling과 무관해야 한다. U는 모델 버전·오차·성능·개입 이력을 노출한다.

## 91 성능·메모리: 개체 수만으로는 규모를 말할 수 없다

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

R7의 비용 지표는 tile, 살아 있는 Cell, Controller, 동적 Body, material site, species-stock pair, 반응 후보, 광 packet, 자기 branch, 최대 coupled domain, substep 수다. 동물처럼 보이는 몸 하나에 Cell100개가 있다면 개체1개라고만 보고하지 않는다. “10만 개체”의 평균 Cell 수와 controller 수를 함께 제시한다.

### 추가 상태의 예상 payload

| 상태 | 예산 가정 | 예시 규모의 payload |
| --- | --- | --- |
| Cell core | 192B/Cell | 2M Cell 약366MiB |
| Controller state | 256B/controller | 100k 약24MiB |
| sparse chemical stock pair | 16B/(owner,species) | 24M pair 약366MiB |
| optical packet | 80B/packet | 1M 약76MiB |
| magnetic material site | 64B/site | 250k 약15MiB |
| flux branch state | 24B/branch | 100k 약2.3MiB |
| reaction progress | 32B/active candidate | 1M 약30.5MiB |

위 예시의 합은 약881MiB이며 **추가 상태의 payload만**이다. world 기하·유체·열·매체·genome·species graph·adjacency·행렬·allocator 여유·rollback·분석 기록·render snapshot은 포함하지 않는다. 기존 agent state와 새 Controller를 동시에 더하는 이중 산정도 피한다. 실제 Rust layout과 peak allocation은 구현 후 측정한다.

Cell10M, 평균 stock pair12개라면 Cell core+pair만3.576GiB다. 평균 pair100개로 늘면 pair만14.90GiB다. 희귀 종이 많아질수록 화학 표현력과 메모리 비용이 같이 커진다. 저장할 원소 총합만 남기고 화학종을 지워서 메모리를 줄일 수는 없다.

자기 행렬을100k branch에 dense f64로 만들면 행렬 하나가80GB, 약74.51GiB다. reference dense solver는 branch1,024 이하의 작은 fixture에만 사용한다. 최종 대형 profile은 행렬을 명시적으로 전부 만들지 않는 구조가 필요하다. 아직 검증되지 않은 matrix-free 해법을 이미 구현된 성능 근거로 사용하지 않는다.

### 실행 profile과 목표

| profile | 명시된 workload | 목표·현재 상태 |
| --- | --- | --- |
| ML-reference | 작은 물리/화학/생체 fixture, 자기 branch≤1,024 | 수치 검증 우선, TPS 목표 없음 |
| ML-small | 512²tile, Controller10k, Cell100k, fine fluid100k, optical50k, magnetic branch2k | 12 physical core·32GB에서 최소1TPS,10TPS stretch; 미측정 |
| ML-large-sparse | 10.24Mtile, Controller100k, Cell2M, static shape1M, fine fluid1M, optical1M, magnetic branch100k | peak32GiB·1TPS를 연구 목표로 검증; 달성 전 지원 보장 금지 |
| ML-stress | Cell10M·고다양성 화학·강한 반사·거대 결합망 | peak RAM·실패 원인·최저TPS 보고; 최소TPS 사전 약속 없음 |

성능을 보장할 release profile은 실제 workload 측정 뒤 동결한다. 이번 확장 때문에79절의 Matter-1 목표를 그대로 최종 제품의 성능 수치로 사용하지 않는다. 1x=10TPS의 시간 정의는 유지하므로1TPS는0.1x다. 500x와 MAX는 희망 배속과 실제 달성치를 함께 표시한다.

### 최적화 우선순위

1. immutable species/genome/shape 공유, 정렬 span·arena·small block allocation, 구조적 반복의 캐시를 적용한다. 같은 template의 비용을 생물학적으로 면제하지 않는다.
2. 정적 광 column·접촉·conduction topology를 재사용하고 물성·형상 revision으로 정확히 무효화한다. sensor가 없어도 빛·열·화학은 그대로 작동한다.
3. Cell의 발생은 공통8tick schedule, 화학·막·유지·수송은 필요한 물리 substep으로 처리한다. plant label을 보고 낮은 빈도를 적용하지 않는다.
4. 원료 없는 반응, 전위차가 정확히0인 안정 network, 물리적으로 변하지 않는 geometry의 작업만 생략한다. 외부 flux가 들어오면 정해진 인과 시점에 깨어난다.
5. field·chemical domain의 검증된 coarse 해법과 독립 domain 병렬화를 적용한다. observer sampling·render LOD·UI 빈도는 별도로 낮춘다.

genotype가 같고 재고·geometry·주변 flux까지 동등한 Cell의 같은 계산을 memoize할 수는 있다. 공간적으로 떨어진 생물들을 한 population 숫자로 합쳐 가축처럼 보이는 개별 몸을 나중에 생성하지 않는다. 멀리 있는 동안 형태 진화를 생략하는 최적화도 기본 제외다.

**경계:** Engineering Contract는 sparse storage·cache·workload·측정이다. W/P의 비용과 반응률은 host 부하로 바뀌지 않는다. E는 같은 규칙에서 생긴다. U는 Cell/Controller/개체 추정 수를 구분하고 실제 배속을 보여준다.

## 92 생태·형태·재료·기술 관찰과 화면

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 판정은 world rule이 아니다

관찰기는 biomass/원소/에너지 흐름, compartment connectivity, geometry motif, genotype와 module ancestry, 실제 제작·사용 trace를 읽는다. 구조가 식물처럼 보이거나 몸이 복잡하다는 이유로 번식 보상을 주지 않는다. 생물 분류·기술 감지·문명 clustering의 결과는 world 입력으로 되돌리지 않는다.

생태 생활 양식은 한 창의 primary fixation, 유기물 흡수, inorganic redox 수익, 이동 work, 외부 고분자 절단, 실제 소비자의 관계로 표현한다. 단일 label보다 비율 벡터와 신뢰도를 먼저 저장한다. “식물”과 “동물”을 섞는 계통, 다른 계통에 의존하는 광영양체, 움직이는 분해자도 표현할 수 있다.

OrganismView는 물리적으로 연결된 living compartment graph와 지속적인 자원·제어 교환에서 추정한다. 붙어 있다는 이유만으로 하나의 생식 개체로 확정하지 않는다. 서로 다른 계통의 융합·군체·기생·일시 접촉은 ancestry mixture와 관계로 남긴다. cell count와 inferred organism count를 함께 표시한다.

### 기술과 생물학적 혁신의 분리

새 형태·촉매·대사 경로는 biological innovation 후보로 기록한다. 같은 재료나 합성 template을 외부 기록으로 전달하고 다른 제작자가 재사용하면 cultural technology 증거가 추가된다. 어떤 생물에서 섬유가 처음 생긴 것과 그 섬유로 제작 절차가 전파된 것을 같은 발명 사건으로 합치지 않는다.

새로운 chemical SpeciesKey가 생겼다는 사실만으로 혁신 알림을 띄우지 않는다. 반복 생산·지속성·기능 변화·다른 구조에서의 재사용·유전 또는 수평 전달 중 어떤 증거가 있는지 기록한다. 서로 동형인 분자를 다른 canonical order로 두 번 발견한 것처럼 표시하지 않는다.

novelty embedding에는 기존 행동 trace와 함께 물질 흐름 motif, 에너지 변환 비율, body connectivity, 발생 sequence, 신호 지연 구조, 재료 사용·반응망 구조를 넣는다. 사람이 정한 공격력·속도 순위가 diversity 축이 되지 않는다. embedding 모델·normalization·샘플링 버전을 고정하고 원자료를 재분석 가능하게 보존한다. archive는 관찰용이며 novelty bonus를 생존 에너지로 주지 않는다.

계통 graph는 template parent, 실제 mutation, module acquisition, somatic branch, 생식 branch, 문화 복사 출처를 구분한다. 기술 ancestry는 graph/module similarity만으로 인과를 확정하지 않는다. 실제 복사·사용 event가 있을 때 확정 전파 edge를, 유사성만 있으면 후보 edge를 표시한다.

### Bevy 화면과 UI

Far zoom은 바이오매스·광 흡수·물·영양염·열·관계망·문명 후보 영역을 chunk texture로 표시한다. Mid zoom은 실제 body의 형상·농도·수송·그늘·집단을 보여준다. Close zoom은 compartment 경계·운반 물질·움직이는 조직·결합·국소 신호·광선 표본을 볼 수 있다. 모든 Cell을 Bevy Entity로 만들 필요는 없다. chunk mesh와 instance buffer를 사용한다.

기본 색은 물질·실제 조직·lineage 중 사용자가 고른 한 기준이다. 식물이라는 detector label 때문에 몸에 잎 sprite를 자동 생성하지 않는다. 원래 geometry에서 읽을 수 없는 기관을 장식으로 추가하지 않는다. 광선 표시가 느려지면 표시 표본만 줄이고 physics packet은 유지한다. magnetic field glyph도 같은 원칙이다.

선택 패널은 body/Cell/Controller를 구분한다. biomass·원소 조성·work 저장량·local metabolism·감각 범위·발생 투자·genome/module·영양 관계·새 조직 출처를 보여준다. 사용자가 “왜 굶었나”를 조사할 때 섭취 실패, 산소/물/인 부족, 운반 병목, 막 손상, 계산 비용을 실제 ledger로 확인할 수 있어야 한다.

기술 패널에는 가능성 R0, 실행 R1, 비용 이득 R2, 자연 발견 R3, 전달·누적 R4를 유지한다. 자기 발전 후보면 입력 기계 work·출력 전기 work·손실·수치 residual, 집광 후보면 입사·출사·흡수량과 온도 변화를 보여준다. “새로운 식물 종”과 “새 문명 기술” 알림은 다른 근거를 갖는다.

### 장기 실험의 기록 한도

모든 photon 경로와 모든 분자 반응을 영구 보관하지 않는다. 필수 보존 장부·계통·template/culture 출처·위험 event는 전수 집계하고 상세 trace는 시간 창별 표본·선택 구간으로 저장한다. gap과 샘플링 비율을 표시한다. 인과 증거를 저장하지 못한 시기에는 확정 기술 전파라고 표기하지 않는다.

기술 소실은 artifact 파괴뿐 아니라 촉매 계통 멸종, 특정 biomass 원료 고갈, 재료 처리 지식의 소실, 광·자기 매체 손상으로도 발생할 수 있다. 기록은 남아도 실행 조건이 없을 수 있다. 지식의 존재와 실현 가능한 생산 능력을 별도 곡선으로 보여준다.

**경계:** Engineering Contract는 관찰 데이터·화면·해석 버전이다. P에는 관찰 label 접근이 없다. E는 형태·먹이망·생산·문화의 실제 변화다. U는 생물 분류·자원 이름·기술 family·문명과 설명 가설이다.

## 93 R7 expected case와 통합 witness

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

다음 X01..X64는 **구현 후 검증할 기대값·판정**이다. 이번 문서 작업에서 실행한 시뮬레이션 결과가 아니다. 숫자가 있는 사례도 제공한 이상화 fixture의 해석값이며 실제 물성의 측정치가 아니다. R5 T01..T97과 R6 P01..P36은 해당 profile의 회귀 기준으로 보존한다.

### 화학·생물 사례

| ID | fixture | 기대값·판정 |
| --- | --- | --- |
| X01 | CO2+H2O→CH2O+O2 | C1/H2/O3 양쪽 일치, 모형 질량62 보존 |
| X02 | N2+3H2→2NH3 | N2/H6, 질량34 보존; 무료 진행을 뜻하지 않음 |
| X03 | 2 C2H5NO2→C4H8N2O3+H2O | C4/H10/N2/O4, 질량150 보존 |
| X04 | 한 매체 unit에 N1/P1, 재고 N7/P3 | 다른 원료 충분 시 최대3unit, N4/P0 잔류 |
| X05 | ΔG°=RT ln4인 가역 반응 | 표준 상태 kf/kr=1/4 |
| X06 | 같은 촉매가 정·역 장벽을2RT만큼 낮춤 | 두 rate 상수 모두 exp(2)배, 평형비 불변 |
| X07 | h(A)=2,h(B)=5,h(C)=9의 닫힌 cycle | 순 Δh=3+4−7=0 |
| X08 | 반응물이 없고 촉매만 있음 | 생성물·에너지 생성0 |
| X09 | 원자 순서를 바꾼 같은 molecular graph | 같은 canonical bytes/종; hash 충돌은 full 비교 |
| X10 | 두 worker가 같은 새 종을 동시 생성 | canonical 정의1개, 재고는 정확히 합산 |
| X11 | 비노출 물질·먼 반응물만 존재 | 접촉/수송 전에는 반응하지 않음 |
| X12 | species pool host 한도 도달 | tick rollback·RunHalt, 종/원료 삭제 없음 |
| X13 | 모든 경계 물질을 포함한 질량101의 Cell 분할 | 두 자식·잔여물 합101; template도 실제 복사 |
| X14 | 분할 방향이 밀폐된 고체로 막힘 | 비관통·부피 유지, 무료 자식 생성 없음 |
| X15 | 활성 조직 단면2, 허용 stress3 | 해당 이상 fixture의 힘 상한6 |
| X16 | 같은 strain 요청, work 저장0 | 유료 active work0, 수동 탄성 반응만 가능 |
| X17 | work carrier 용량10×3, 현재 work25; carrier1 파괴 | 잔류 용량20, 초과5는 정의된 에너지 경로로 회수/열화 |
| X18 | 같은 생물 cluster의 떨어진 두 Cell | 자원·code의 즉시 공유 없음 |
| X19 | 살아 있는 조각이 실제로 분리됨 | 질량·전하·운동량 유지, 조건 충족 시 local 제어 지속 |
| X20 | template 매체를 부분 파괴 | backing 감소·오류·실행 실패 반영, bytes 무료 보존 금지 |
| X21 | 전류가 있는 도체를 절단 | Q·field 에너지·gap 과도 상태 정산, ½LI² 삭제 금지 |
| X22 | 부모가 문화 module만 학습하고 생식 template에는 안 씀 | 자식에게 자동 germline 유전되지 않음 |

### 자기·광학 사례

| ID | fixture | 기대값·판정 |
| --- | --- | --- |
| X23 | L=2,I=3 | λ=6, 자기 저장 에너지9 |
| X24 | λ=4 고정, L2→4 | I2→1, 저장 에너지4→2; 차이2는 실제 work/열 경로 |
| X25 | L=[[2,0.5],[0.5,1]],I=(1,2) | λ=(3,2.5), E=4, determinant1.75>0 |
| X26 | L=2,R=1,I(0)=3, 무입력 | t1에 E=9 exp(−1), 누적 저항열9[1−exp(−1)]에 수렴 |
| X27 | 정지 자화·정지 도체, 이미 안정된 Q/λ | 지속적인 전력 생성 없음 |
| X28 | 자석을 움직여 닫힌 도체에 전류 유도 | 출력·손실과 입력 work의 정산, 역작용 누락 금지 |
| X29 | 자화 relaxation으로 Um이7 감소 | 그7이 실제 열/일로 이동, 무상 삭제 없음 |
| X30 | 재료 회전·다시 원위치인 닫힌 무손실 cycle | 최종 상태 같으면 순 에너지 생성0 |
| X31 | 같은 기하의 reference와 matrix-free 평가 | 에너지·힘·대칭성·SPD·오차 gate 충족 |
| X32 | solver iteration/retry 횟수 증가 | 전극 반응 입력·boundary work 중복 지급 없음 |
| X33 | 광 E=8, αd=ln2, 비반응 흡수체 | 투과4·흡수열4 |
| X34 | n1=1,n2=1.5, 수직 입사 E10, 무흡수 | Fresnel 반사0.4·투과9.6 |
| X35 | n1=1.5→n2=1, 입사60도 | 전반사, 투과0 |
| X36 | energy 최소량7, 세 방향 weight1/3 | canonical 첫 분기3, 나머지2·2, 총7 |
| X37 | 거울을 여러 번 지난 packet | 반사만으로 에너지 증가 없음 |
| X38 | 광 E10000, fixture 탄소 고정 Δh5000 | 반응1회 시 chemical 증가5000, 잔여5000은 열 등 지정 경로 |
| X39 | 동일 photons에 photo credit도 켜는 잘못된 경로 | 이중 지급 검출·실패 |
| X40 | 발광이 packet energy13을 방출 | source의 실제 work/chemical/thermal 감소 합13 이상, 손실 정산 |
| X41 | 닫힌 동일 온도·무구동 광학 cavity | 지속 work 없음, 복사 평형 오차 gate |
| X42 | 이동 중 photon packet 상태를 저장·복원 | 도착 시간·band·흡수량·hash 동일 |
| X43 | 광 pool 한도 초과 또는 camera 밖으로 나감 | photon 삭제 금지; host 한도는 안전 정지, camera는 무관 |
| X44 | 각도/geometry 해상도2배 reference | 집광 위치·흡수·통신 지연의 지정 허용 오차 검사 |

### 생태·실행·관찰 사례

| ID | fixture | 기대값·판정 |
| --- | --- | --- |
| X45 | C/N/P가 닫힌 세계의 성장·죽음·분해 | 전체8원소 벡터가 시작과 정확히 같음 |
| X46 | 분해자 없는 사체 | 허용된 비생물 반응만 진행; 타이머 영양분 보상 없음 |
| X47 | 햇빛·모든 외부 자유 에너지 공급 차단 | 저장 에너지를 넘는 영구 생산 불가 |
| X48 | P가 모든 접근 가능한 stock에서 고갈 | P를 요구하는 새 매체/조직 생산 중단 |
| X49 | O2 없는 조건의 산소 호흡 경로 | 해당 경로 중단; 다른 반응의 존재와 구별 |
| X50 | 그림자 형성 후 producer의 흡수 측정 | 실제 차폐된 광자만큼 생산 조건 변화, 광원 추가 없음 |
| X51 | 밀폐된 살아 있는 몸에서 외부 TRANSFER | 내부 재고 직접 탈취 실패; 노출/막 수송 필요 |
| X52 | 초기 조상·계통 전멸 | 자동 재주입 없음, 멸종 기록과 종료 상태 보존 |
| X53 | 발생·반응·광·자기 진행 뒤 trial 실패 | VM·cell·species·RNG·field·ledger 전체 rollback |
| X54 | worker1/2/8, 동일 seed/input/승인 backend | 완결 tick hash 일치 |
| X55 | Pause/MAX·카메라 이동·UI sampling 변경 | world hash·에너지·birth event 불변 |
| X56 | Save→load→동일 입력 | species ID mapping·진행 잔여·packet·자화 포함 hash 일치 |
| X57 | 사용 중 SpeciesDef의 cache 회수 | canonical 정의/성질/미래 반응 불변 |
| X58 | invalid template가 복사 중 만들어짐 | 실제 소비 비용 유지, 유리한 자식 재추첨 없음 |
| X59 | 내 몸 밖의 Cell handle 또는 stale target | 접근 실패, 외부 work/stock 변경 없음 |
| X60 | 같은 표현을 phototroph/animal label로 교체 | Core 실행·선택압 변화0 |
| X61 | novel species 발생하지만 재사용·기능 증거 없음 | chemical novelty만 표시, 확정 기술 알림 금지 |
| X62 | 조상에게 제공한 대사·체형을 처음 관찰 | seeded 표시, 자연 발명 R3로 세지 않음 |
| X63 | 광 source/자기 fixture를 제거한 인과 branch | 해당 기능의 의존성과 비용 변화를 보고, 자연 run과 구분 |
| X64 | 기록은 남지만 촉매 계통/원료가 사라짐 | 지식 존재와 실현 가능한 기술 보유를 별도로 표시 |

### 복합 witness와 단계별 증거

W-ML1은 광 흡수→유기물→조직 성장→사망→분해→영양염 재흡수의 닫힌 물질 cycle이다. W-ML2는 같은 조상 규칙의 변화가 형상·수송·접촉 이동에 실제 차이를 만드는지 확인한다. W-ML3은 원료에서 시작한 촉매·고분자 경로와 재료의 물성 차이를 검사한다. W-ML4는 물질로 구성한 도체 loop와 자성 구조의 운동→전기 변환, 입력 work와 역작용을 검사한다. W-ML5는 조형한 투명/반사 재료의 집광→열/광반응 효과를 검사한다. W-ML6은 이 절차 중 하나가 기록·비친자 전달·재제작·소실로 이어지는지 확인한다.

각 witness는 초기 재료·외부 에너지·제작 프로그램·solver/manifest hash를 공개한다. 처음부터 완성된 구조를 놓는 테스트는 구조가 작동할 수 있다는 R1 증거이고, 원료로 제작한 테스트와 구분한다. 제작·유지·학습 비용까지 회수해야 R2다. 자연 run에서 발견해야 R3, 전달·재사용·누적이 확인되어야 R4다.

**경계:** Engineering Contract는 수치 사례·보존·인과·회귀 검증이다. W/P를 검증하기 위한 손작성 장면을 허용하지만 자연 run과 분리한다. E의 성공은 실제 미개입 증거가 필요하다. U는 단계와 불확실성을 표시한다.

## 94 통합 로드맵·실패 모드·현재 완료 상태

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

**R8 적용 안내: 이 절은 MatterLife-1 기반이다.95절의 교체표가 우선하며, 화학·파동·환경·계산 표현·최신 성능/완료 상태는96..107절을 따른다.**

### 추가 실패 모드와 trade-off

| 실패 | 예방·판정 | 비용·trade-off |
| --- | --- | --- |
| 생체 기관이 사실상 완성 부품 | 기관 enum 제거, tissue·원료·실제 기하로 기능 확인 | 발생 검증·학습 난도가 증가 |
| 물질은 많지만 거의 모든 반응이 무의미 | 간단한 유용 경로·가역성·중간 이득 witness부터 검증 | 물성·환경 연구 필요, 결과 보상은 금지 |
| 화학 cache가 무한히 커짐 | sparse stock·canonical 공유·0참조 회수·explicit halt | 생성 종 수의 표현 상한과 host 한도 공개 |
| 분자 이름만 다르고 성질이 동일 | 재료 구조·반응 중심·실제 property 구별을 측정 | 물성 함수 복잡도·교정 부담 |
| 모든 생명이 광영양 단순 복제자로 수렴 | 유한 영양염·실제 차폐·수송/표면 비용·분해 경로 | 다른 niche가 존재해도 발견은 보장 못함 |
| 큰 몸·복잡한 genome이 무조건 손해 | 표면/체적·저장·접촉·수송의 실제 이득과 비용 함께 검사 | 복잡성 보너스로 강제 성장시키지 않음 |
| 먹기만 해도 완전 효율로 work 충전 | 접근·분해·산화제·촉매·자유 에너지·열 제한 | coarse 생화학의 범위·수치 데이터 필요 |
| 생태계가 영양분 잠김으로 종료 | 다양한 용해·분해·재활용 경로의 가능성 확인 | 멸종/퇴적 자체는 허용, 자동 보정 없음 |
| 빛/자기로 무한동력 발견 | 고정 Q/λ 힘, 역작용, 평형·cycle·retry 시험 | solver·결합 검증 비용 큼 |
| 새 몸에서 문화가 전혀 전달되지 않음 | 매체 접근·자기 몸 조건·실행 실패와 적응을 구분 | 완전 보편적 module 호환은 보장하지 않음 |
| 모든 생물에 VM을 달아 속도 붕괴 | small development state·optional controller·실제 Cell 수 보고 | 세포 수준의 무한 제어 자유도는 제한 |
| 재미를 위해 대사·기술을 몰래 주입 | seed/fixture/intervention manifest와 R0..R4 분리 | 초기 탐색이 오래 걸릴 수 있음 |

### 개발 의존 순서와 버전

v0-ML은 기존 core/VM/SoA/observer 경계를 가져와 species/stock/energy audit, 단세포 compartment, 유료 template 복사, 기본 광·화학 순환, 작은 시각 세계를 완성한다. 형태 발생은 작은 site 수에서 시작하고 고정된 speed/attack 몸으로 되돌리지 않는다. 이 단계의 광·자기 미완성 profile은 완제품으로 표시하지 않는다.

v1-ML은 다세포 발생·국소 수송·조직 힘·손상·재생, 다양한 합성 template, 살아 있는 생태 자원, 실제 optical transport와 전기·자기 reference를 결합한다. W-ML1..W-ML5에서 제작·작동·비용을 검사한다. 낮은 primitive에서 나온 중간 경로가 없으면 해당 재료/경계 가설을 새 run으로 수정한다.

v2-ML은 모든 필수 세계 법칙을 시작부터 켠 고정 manifest에서 문화 전달·기술 계보·생태 변화·문명 감지, 저장/replay·Bevy UI·대규모 workload를 연결한다. W-ML6과 장기 다중 seed 실행을 수행한다. 기술 수준에 따라 자기·광학·생명 법칙이 추가되는 progression은 없다.

v3는 실제 파동 optics/전자기 전파, 더 많은 공간 층, 진화하는 mutation operator, 신경 모듈, 복잡한 발생·화학 표현을 별도 ruleset으로 연구한다. v3 항목을 지금 구현된 기능처럼 표시하지 않는다. R7에서도 큰 고분자·미세 기관·3D 회로·모든 현실 기술을 표현할 수는 없다.

### 바로 다음 구현의 입력 산출물

| 산출물 | 현재 문서가 정한 것 | 아직 필요한 완료 증거 |
| --- | --- | --- |
| MaterialData-ML1 | 원소·graph·phase·coarse property와 derivative 계약 | 실제 coefficient·단위·범위·검증 가능한 hash |
| ChemistryKernel-ML1 | rewrite 계열·상세평형·stock·반응 scheduling | 허용 valence/전자 상태 표·전이 상태 함수·수치 수렴 |
| BioData-ML1 | compartment·조직·발생·복사·carrier 회계 | 최소 template/막/촉매 자료·성장/생존 witness |
| MagneticData-ML1 | 에너지 상태·역작용·기하 graph·자화 | 유한 단면 kernel·자화 곡선·절단 과도 상태·solver 실행 |
| OpticalData-ML1 | slab·6band·packet·흡수·방출·각도 양자화 | 단위 변환·물성값·경계/열 평형·오차 기준의 실측 |
| ABI-ML1 | controller 접근·service 의미·register pack | service 숫자·field scale·binary golden vectors |
| WorldGen-ML1 | 유한 초기 재고·공개 seed·전멸 처리 | 실제 조상 bytes·부피/압력/온도/원소/에너지 초기 장부 |
| CIVML001 wire | header·필수 상태·부분 저장 금지 | 정확한 field offset/encoding·migration·corruption/resume 시험 |
| 수치 검증 | X01..X64와 W-ML1..W-ML6 계약 | 실제 executable reference와 통과 trace |
| 규모·생태·문명 | workload·관찰·증거·실패 기준 | CPU/RAM/TPS 실측 및 미개입 장기 관찰 |

이 문서 작업에서 완료한 것은 **요청한 다섯 확장 축의 아키텍처·상태·소유권·수식·진화 경계·실행 단계·검증 계약을 통합한 설계 개정**이다. 실행 가능한 시뮬레이터나 연구 결과가 완료된 것은 아니다. 위 표의 실제 데이터와 수치 reference가 없는 상태를 “더 이상 수정할 일이 없는 최종 구현 청사진”으로 표현하지 않는다.

다음 구현의 시작점은 작은 폐쇄 세계의 원소·전하·에너지 oracle와 MaterialData다. 이후 화학/빛→생체 성장·분해, 기존 역학/유체→새 몸, 전하→자기·유도, 매체→문화의 의존 순서로 합친다. 검증을 마친 뒤에도 생존 가능한 복잡성·새 행동·누적 기술의 자연 발견은 별도 연구 결과로 평가한다. 물리 자유도를 넓힌 목적은 완성된 문명 기능을 숨겨 넣는 것이 아니라, 생물·재료·장치·지식이 서로의 새로운 가능성을 만드는 조건을 여는 데 있다.

**경계:** Engineering Contract는 개발 순서·동결 상태·실측 승인이다. W/P는 명시된 저수준 규칙에 한정한다. E는 몸·생태·기술·사회의 실제 역사다. U는 무엇을 지원하고 무엇을 관찰했는지 증거와 함께 설명한다.


## 95 R8 범위: 화학·파동장·환경·계산 표현의 결합

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

R8은 사용자가 고른 네 축만 확장한다. R7의 생물 형태·발생·대사·문화는 유지하고, 자유로운 3D 몸 배치, 신경 제어기, 진화하는 돌연변이 연산자는 이번 범위에 추가하지 않는다. 지형의 높이·퇴적 깊이는 환경의 상태이며 agent가 자유롭게 쓰는 새로운 공간 축이 아니다. 파동장은2D 공간에서3성분을 계산하므로 실제3D 안테나나 입체 광학을 그대로 재현하지 않는다.

새 기준 ruleset은 civ-continuum-1-draft, 목표 제품은 v2-Continuum이다. R7을 실행한 뒤 기술 수준에 따라 네 법칙을 켜는 방식이 아니다. 완성 profile에서는 시작부터 모든 법칙이 존재한다. 아래 설계 결정과 reference 수치는 구현 입력이며, 실제 solver·물성 데이터·자연 진화 증거가 완성되었다는 뜻은 아니다.

### 기존 계약의 적용 순서

| 영역 | 유지할 기반 | R8의 추가·교체 계약 |
| --- | --- | --- |
| 원소·화학종 | 82절의8원소, 가변 graph, 원소·전하 보존 | 96..97절의 물리적 조립·미세구조·상분리·촉매망 |
| 생명 | 83..85절의 실제 조직·발생·물질 순환 | 환경·재료·기록과의 접촉 경로 확대, 기관 prefab 추가 없음 |
| 전기·자기 | 물질의 유한 전도·자화·에너지 비용 | 98..99절의 동적 장이 전하·자기 에너지를 단독 소유;86절 준정적 에너지와 중복 합산 금지 |
| 빛 | 87절의 실제 광 에너지·기하 수송 | 위상 있는 평면 파동과 비간섭 복사의 소유권을99절에서 분리 |
| 지형·자원 | 국소 물질 이동·유한 재고 | 100..101절의 침식·퇴적·수문·대기 조성·기후 피드백 |
| 실행·문화 | 89절 VM/ABI·유료 기억·국소 전달 | 102절의 데이터 해석·자기 언어·해석기·물리 기록 |
| 소유권·저장 | 90절의 단독 Core·phase·rollback | 103..104절의 새 store·phase·CIVCT001 schema |
| 성능·검증 | 91..94절의 미실측 표기·R0..R4 증거 | 105..107절의 새 workload·64개 기대 사례·동결 조건 |

현재 문서의 우선순위는95..107절의 명시적 교체,81..94절의 유지 부분,68..80절의 유지 기반 순이다.06..67절은 계속 Grid-5 회귀 명세다. 이전 버전의 구체적인 숫자를 새 모델의 빠진 계수로 자동 대입하지 않는다.

### 네 축이 여는 경로와 한계

화학은 같은 조성에서도 배열·접촉·가교·기공이 다른 재료를 허용한다. 파동장은 구조의 형상과 재료가 신호의 전파·간섭·공진을 바꾸게 한다. 환경은 살아 있는 자원과 지식의 유용성을 장기간 바꾸는 물질 흐름을 만든다. 표현 진화는 같은 기본 명령으로도 후손이 다른 언어·자료구조·절차 묶음을 이어받을 가능성을 만든다.

네 축을 추가한다고 개방형 진화가 보장되지는 않는다. 이 설계의 목적은 숨은 완성 기능의 수를 늘리는 것이 아니라, 기존의 한 발명이 다른 영역에서 새 용도를 가질 경로를 늘리는 것이다. 유한 격자·원소·표현 길이·수치 범위는 여전히 천장이다. 그 천장을 넘었다는 주장 대신 실행 가능한 조합과 실제 발견을 구분한다.

**경계:** W는 물질·장·환경·유료 계산의 국소 법칙이다. P는 기존 물질·힘·기록·발현 조작이다. E는 새 재료·통신·생태 변화·언어·기술의 결합이다. U는 네 축의 작동 증거와 표현 한계를 표시한다.

## 96 화학 확장: 조성·분자·미세구조를 분리한다

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 선택지 비교와 결정

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 결정 |
| --- | --- | --- | --- |
| 고분자 길이 상한만 증가 | 기존 구현 재사용, 더 긴 서열 | 배열·기공·계면을 설명하지 못함; 탐색 공간만 커질 수 있음 | 단독 해법으로 사용하지 않음 |
| 전 원자 reactive MD | 입체 구조와 반응의 폭넓은 결합 | 문명 시간척도·규모에 부적합, 파라미터 검증도 매우 어려움 | 제외 |
| 구조 화학+물질을 가진 중간 크기 조립 요소 | 결합망·배향·계면에서 새 성질 가능 | 내부 상태·소유권·구성식의 추가 비용 | **선택** |

R7의 작은 분자64 heavy atom, 고분자4,096 monomer 제한은 이번 개정에서 유지한다. 대신 여러 화학종과 고분자가 실제로 연결·배열되는 assembly의 크기는 연속 arena와 공개된 실행 한도로 관리한다.4,096 monomer보다 큰 조립체가 가능해져도 하나의 무한 길이 분자를 지원한다고 표시하지 않는다. 공유결합의 분자 상한을 우회하려고 assembly bond를 공유결합처럼 해석하지 않는다.

### 세 가지 상태와 물질의 단독 소유

SpeciesDef는 화학적 정체성, StockLot는 실제 양과 phase, Microstructure는 그 물질의 공간적 구성이다. MicroNode는 새로운 물질 재고가 아니라 StockLot의 독점적 material slice를 차지하는 물리 요소다. 용액의100unit 중20unit을 조립하면 자유 재고80과 조립 재고20이 남는다. 두 곳에서20을 동시에 소비할 수 없다.

Assembly membership는 연결된 물리 요소를 찾기 위한 계산 cache다. 엔진·조직·막·컨테이너 같은 의미를 가지지 않는다. 연결망이 끊어지면 connected component가 바뀌고, 각 component는 실제 질량·운동량·열·표면 전하를 가진 채 남는다. 완성된 물체 이름을 발견했다고 한 rigid body로 무료 변환하지 않는다.

MicroNode의 최소 상태는 material_slice, position2, orientation, occupied_area, momentum, thermal_owner, exposed_patch_span, constitutive_revision이다. MicroBond는 양 끝 patch, rest geometry, elastic state, chemical binding state, damage state를 가진다. 노드는 기존 material site와 동일 공간 단위를 사용한다. 물체 안에 숨은 별도 우주를 만들어 비관통·수송 거리를 우회하지 않는다.

### 접촉 patch와 일반 상호작용

Patch의 노출 방향·면적·국소 전하·donor/acceptor 특성·친용매성은 실제 분자 표면과 구조에 의해 정해진다. agent가 임의 이름을 써서 상보성을 획득하지 않는다. pair interaction은 반발, 방향 의존 인력, 용매 매개 항, 결합 탄성을 조합한다. 모든 힘과 토크는 동일한 에너지 함수의 위치·각도 미분에서 나온다.

reference 형식은 U_pair=U_rep(r)+s(r) a(θ_i) a(θ_j) U_attr(r)+U_solvent이다. s는 유한 범위의 매끄러운 cutoff, a는0..1의 방향 가중치다. coefficient는 원소·표면 구조·용매 상태의 공개 함수다. 경계에서 힘이 갑자기 튀는 단순 truncation을 피한다. 이것은 인공 중간 크기 상호작용이며 실제 단백질 접힘을 예측하는 potential이 아니다.

화학 결합의 기준 에너지는 SpeciesDef의 Echem에, 조립 결합의 에너지는 MicroBond에, 탄성 변형은 Uelastic에 한 번씩 들어간다. 기존 표면 에너지 항과 patch attraction이 같은 접촉을 두 번 보상하면 안 된다. 각 pair term의 owner와 활성 구간을 EnergyTermRegistry에서 선언한다.

HOOMD-blue의 방향성 patch potential은 이런 표현이 실제 입자 시뮬레이션에 사용되는 예다. 여기서 빌리는 것은 국소 방향성 상호작용이라는 모델 방식이며, 그 소프트웨어의 계수나 결과를 본 프로젝트의 물성 검증으로 대신하지 않는다. [HOOMD-blue PatchyExpandedGaussian](https://hoomd-blue.readthedocs.io/en/stable/hoomd/md/pair/aniso/patchyexpandedgaussian.html)

### 상분리·가교·복합재

상분리는 지역 조성의 mixing free energy와 계면 비용을 가진다. reference의2성분 혼합은 f=RT[c ln c+(1−c)ln(1−c)]+χc(1−c), 계면항은 κ/2·|∇c|²로 둔다. c=0/1 경계는 활동도와 flux의 극한을 사용하고 log0을 직접 계산하지 않는다. 질량을 옮기는 finite-volume face flux만 조성을 바꾼다. 단순 색칠이나 phase label 교체로 분리시키지 않는다.

다성분 확장은 동일한 자유 에너지의 화학 퍼텐셜과 양의 준정부호 mobility를 쓴다. 동일 물질에 explicit patch 조립과 phase-field 계면 모델을 함께 적용할 때 각 항의 역할을 분리한다. explicit geometry의 접촉을 다시 계면장으로 세지 않는다. 초기 CT1 reference는 한 material region에 하나의 공간 표현만 권위 있게 사용한다. 변환은107절 검증 전 지원하지 않는다.

가교·충전재·기공·배향은 상태이고 강도·투과·전도·광학 응답은 그 상태의 결과다. 단순 평균 조성으로 모든 복합재 특성을 정하지 않는다. 노출된 geometry와 연결망의 실제 수송·응력 해석을 reference로 삼는다. homogenization은 같은 경계 응답을 오차 범위 안에서 보존할 때만 사용한다. 이전의 파괴·배향·분극 기억을 지우는 평균화는 금지한다.

**경계:** W는 patch 상호작용·상분리·재고 소유권·구성식이다. P는 실제 합성·배치·열·힘·분리다. E는 자기조립·복합재·새 촉매 표면·생체 재료다. U는 재료 family와 조성·미세구조·측정 성질을 구별한다.

## 97 화학 동역학·촉매망·자기조립의 실행 계약

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 조립과 화학 반응을 같은 접촉에서 처리한다

반응 후보는 실제 contact patch와 접근 가능한 StockLot에서만 생성한다. 접촉 없는 두 반응물을 전역 registry에서 찾아 결합하지 않는다. 촉매의 결합 자리는 유한 면적과 점유 상태를 가지며, substrate를 잡은 동안 다른 substrate가 같은 자리를 동시에 쓰지 못한다. 흡착·탈착·반응·손상은 모두 재고와 에너지 transaction이다.

결합·해리의 transition-state는 정·역방향을 함께 정의한다. 바닥 상태 사이 에너지 차이가 바뀌면 평형도 바뀔 수 있지만, 순수 촉매가 transition-state만 바꾸면 평형은 바뀌지 않는다. 표면을 “촉매”로 분류한 사실로 반응률 배수를 주지 않는다. 흡착 때문에 자유 농도가 변하는 현상과 촉매가 반응 평형을 바꾸는 오류를 구별한다.

분자 graph rewrite는 local connectivity를 바꾸고, assembly 해석은 물리 요소의 배치와 결합 점유를 바꾼다. 하나의 사건이 두 상태를 바꾸면 하나의 transaction으로 묶는다. 후보 생성 중 조립 재고를 자유 재고처럼 다시 열거하지 않는다. 생성된 종은 R7의 canonical pending key를 따르고 commit 순서는 worker 순서와 무관하다.

### 열 운동과 최소 reference

CT1의 첫 reference는 deterministic extent chemistry와 감쇠된 조립 역학을 유지한다. 감쇠로 줄어든 운동·변형 에너지는 실제 열로 간다. 상분리의 화학 퍼텐셜 구배와 실제 유체 수송이 조립 요소를 움직인다. 스스로 장벽을 넘을 수 있는 모든 열 운동을 정밀 재현하지 않으며, 장벽 통과는 명시된 화학 반응률에서 처리한다.

Brownian displacement를 단순 난수 이동으로 추가하지 않는다. 열 요동을 추가하려면 유한 열 bath와 fluctuation-dissipation, substep 분할에도 같은 사건을 정의하는 RNG, 운동량·에너지 교환이 필요하다. 이는 CT1의 별도 미동결 확장이다. 초기 모형이 열적 자기조립을 과소 탐색할 수 있다는 제한을 실험 결과에 표시한다.

reference 적분은 한 microstep에서 transport 반 단계, contact/reaction candidate 생성, coupled extent·binding·mechanics solve, transport 반 단계, 원소·전하·에너지 audit 순서다. competing reaction extent는 R7처럼 정렬된 대칭 sweep과 step 축소로 수렴을 확인한다. 접촉의 생멸, valence saturation, substrate 고갈 때 step을 나누며 음수 농도를0으로 잘라 보존을 맞추지 않는다.

### 촉매망과 살아 있는 제작 체계

반응망은 registry에 미리 배포된 “생산법”이 아니다. 실제로 관측한 flux graph를 observer가 만든다. 촉매 A가 B의 생성을 돕고 B가 A의 생성을 돕더라도, 모든 원료·빛·열·일과 분해 경로가 같은 회계 안에 있어야 한다. cycle이라는 모양만으로 생명·복제·발명으로 세지 않는다.

살아 있는 조직이 재료를 분비하고 그 재료가 조직을 보호하거나 신호를 조절하는 경로는 허용한다. BiologicalMachine enum, 생체 공장 능력, 생물에게만 주는 repair bonus는 없다. 자기복구가 관측되려면 파손된 접촉 주변에서 실제 재료 공급·반응·열 처리가 이루어져야 한다. 죽은 뒤에도 수동 조립과 촉매 반응은 조건이 남으면 계속된다.

### 계산 상한과 새로운 중간 이득

neighbor list는 geometry cell과 cutoff로 만들고 canonical pair를 한 번만 평가한다. 접촉 후 반응 후보는 반응 중심 signature로 색인한다. 최대 조립 요소 수·총 bond 수·활성 반응 수는 manifest에 공개한다. 메모리 부족 때문에 새 재료만 불리하게 실패시키지 않고 RunHalt로 멈춘다.

복잡성의 보상은 추가하지 않는다. 더 단순한 막의 누출 감소, 더 안정된 접촉, 촉매 재사용, 기존 물질의 재활용 같은 중간 기능이 실제 조건에서 비용을 회수하는지 손작성 witness로 먼저 확인한다. 이는 R1/R2의 가능성 검증이며 진화가 그 경로를 찾은 R3 증거가 아니다. 환경·계수 변경 실험은 별도 manifest로 남긴다.

**경계:** W는 국소 반응·점유·장벽·감쇠·수송과 실제 비용이다. P는 기존 국소 물질 조작·합성·발현이다. E는 촉매망·자기조립·자기복구·살아 있는 생산 체계다. U는 실제 flux와 제작 계보를 보여준다.

## 98 파동장: 동적 Maxwell 격자와 명확한 해상도

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 선택지 비교

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 결정 |
| --- | --- | --- | --- |
| 신호 packet에 주파수·거리 감쇠 부여 | 저비용, 긴 거리 송수신 쉬움 | 위상·회절·구조 공진을 실제로 만들지 못함 | 파동장 대체로 사용하지 않음 |
| 국소 FDTD 섬+원격 scattering 연결 | 작은 장치에 계산 집중 가능 | 이동하는 장치·다중 산란·near field·위상·경계 소유권이 매우 어려움 | 후속 최적화 연구 |
| 고정2D Maxwell 격자+재료 응답 | 국소 법칙에서 전파·간섭·공진·유도 가능 | 전역 격자와 많은 substep, 큰 계산비 | **정확성 reference로 선택** |

CT1의 authoritative coherent field는2D 위치에 E,D,B,H의3성분을 가진다. ∂/∂z=0인 병진 대칭 모델이다. 두께1을 에너지 환산에 쓰지만 z로 빠져나가는 실제3D 방사까지 재현하는 slab 해법은 아니다. 원거리 감쇠·편광 모드·안테나 패턴도2D 해석의 결과다. 하늘로 움직이는 물체, 입체 배선, 실제3D 전파를 이번 범위로 주장하지 않는다.

### 상태 방정식과 격자 위치

reference 방정식은 ∂B/∂t=−curl E, ∂D/∂t=curl H−Jfree다. D=ε∞E+ΣP_a, B=μ0(H+M)이며 분극 P와 자화 M은 유료 물질의 구성 상태다. 모든 전도·생체·화학·이동 전하는 동일한 Jfree를 통해 결합한다. 전자기파 송신을 위해 원격에 임의 E를 더하는 primitive는 없다.

Yee형 staggered grid에서 E/D와 B/H는 공간·시간 위치가 다르다. discrete curl/divergence의 incidence를 같은 grid topology에서 생성하여 div(curl)=0이 성립해야 한다. 연속식을 각 array에 임의 배치하는 구현은 금지한다. free charge와 current deposition은 Δρ+Δt·div Jfree=0을 같은 stencil로 만족한다.

전하 초기화는 Gauss 제약을 만족하는 장을 구축하고 필요한 에너지를 초기 장부에 기록한다. 이후 전하를 움직일 때 그 경로 전체에 current를 deposit한다. 큰 장 오차를 숨기기 위해 매 tick 전기장을 비물리적으로 reset하지 않는다. 초기 제약 solve와 오류 진단용 projection은 실제 시간 전파와 구분한다.

### 시간·공간 해상도

CT1 reference 후보는 Δx=Δy=0.25tile, c_wave=32tile/tick, Courant S=0.5다. 진공 격자의 Δt=1/256tick이므로 최소256 EM step/tick이 필요하다. 매질의 가장 빠른 모드·분극 resonance·손실 적분 제약은 더 작은 step을 요구할 수 있다. S만 검사하고 dispersive material의 안정성까지 보장했다고 하지 않는다.

Yee FDTD는 공간과 시간 해상도의 제약을 함께 받는다. 분산 재료에는 추가 분극 상태와 별도 안정성 검사가 필요하다는 점을 reference 선택의 근거로 사용한다. [Meep Introduction](https://meep.readthedocs.io/en/latest/Introduction/) · [Meep Materials](https://meep.readthedocs.io/en/latest/Materials/)

물리적 해석의 목표 범위는 해당 매질 안 최소 파장16cell 이상과 resolve된 재료 경계다. 이는 정확도를 보장하는 정리가 아니라 초기 검증 기준이다. 격자 모드는 그 아래에도 존재할 수 있으나 이를 현실적인 광학 발명으로 분류하지 않는다. 해상도를 두 배로 늘린 reference에서 주파수·위상·전송률·힘의 수렴을 확인한다. 파장이 짧다는 이유로 공짜 packet으로 바꾸지 않는다.

### 파동이 가능한 기능

전도 경로의 시간 변화가 장을 만들고 다른 재료의 전하·분극이 그 장에 반응한다. 산란·회절·간섭·정재파·공진·차폐가 이 관계에서 나타날 수 있다. 송신기·수신기·안테나·필터·무선 채널은 prefab이 아니다. code payload는 장에 붙이지 않고 agent가 실제 국소 신호를 해석해야 한다.

여기서 파동광학은 모형에서 해상된 파장의 광학적 현상을 뜻한다. 수μm 태양광을 거대한 tile 지도 전체에서 직접 분해한다는 뜻이 아니다.87절의6band 비간섭 복사는 별도로 유지하며, 그 빛의 모든 경로에 간섭을 지원한다고 표현하지 않는다. 두 표현의 연결과 한계는99절에 고정한다.

### 경계와 희소 실행

외부 경계의 PML/흡수층은 세계 밖 수치 경계다. 흡수한 에너지는 BoundaryLedger의 radiation_out으로 간다. agent가 그 층을 접촉해 무료 열원·특수 재료로 이용할 수 없다. 반사 경계·주기 경계·입사파는 별도 manifest에 명시한다. PML 폭·반사 허용 오차는 reference 시험으로 동결한다.

청크를 생략할 수 있는 조건은 장·재료 기억·전류·입사 stencil·경계 source가 모두 정확히0인 경우다. 인접 전파가 도달하기 전에 stencil halo를 깨운다. 작은 진폭을0으로 잘라 wavefront를 없애거나 카메라 밖 파동을 버리는 최적화는 금지한다. 정전기장이 넓게 퍼져 있으면 이런 정확한 생략이 거의 불가능할 수 있다.

**경계:** W는 격자 Maxwell·유한 전파·재료 응답·경계다. P는 실제 전하·물질·힘·국소 발현 조작이다. E는 파동 송수신·공진·회절·간섭 기반 장치다. U는 주파수·위상·장 흐름과2D/해상도 한계를 표시한다.

## 99 장·빛·물질 결합: 에너지의 중복 소유를 막는다

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### R7 준정적 회계의 교체

CT1에서는98절의 장이 정전기·유도·전파를 함께 소유한다.86절의 Uelectric(Q,x)와 branch ½LI²는 따로 합산하지 않는다. ConductionGraph는 기하 색인·전류 적분·관측 용도로 남길 수 있지만, 별도의 λ 저장 에너지가 실제 장 에너지와 중복되면 안 된다. 준정적 해법을 함께 돌릴 때는 별도 ruleset의 비교 reference로만 사용한다.

기준 선형 진공 항은 Ufield=∫[ε∞|E|²/2+|B|²/(2μ0)]dV다. 분산·자성·비선형 물질은 추가 구성 자유 에너지와 상호작용 항을 포함해야 하며 위 식 하나로 다 계산했다고 하지 않는다. 가장 단순한 수동 분극은 P¨+γP˙+ω0²P=aE, a>0, γ≥0이다. 그 oscillator 에너지는(|P˙|²+ω0²|P|²)/(2a), 손실은γ|P˙|²/a로 열에 전달된다.

자화 M과 B의 결합 항, 잔류 자화의 저장 에너지, 자기 손실은 같은 MagneticConstitutive 함수에서 나온다.86절의 Φm·L kernel과 Maxwell 자화 source를 동시에 써서 힘을 더하지 않는다. 실제 함수·자화 적분·힘 미분이107절 gate를 통과하기 전에는 자성 재료까지 완성되었다고 표시하지 않는다.

### 일·힘·이동 재료

J·E는 장과 물질 사이의 전력 교환이다. 전도 손실은 열로, 유료 구동은 실제 chemical/work store의 감소로 연결한다. 외부에서 손작성한 impressed current는 시험 fixture 또는 명시적 외부 전원이다. 자연 진화 run의 송신기는 이 경계를 사용할 수 없다.

이동하는 유전체·도체·자성체는 grid occupancy와 구성 계수를 바꾼다. 단순히 ε를 바꾸고 E를 유지하면 에너지가 생길 수 있다. reference는 재료 점유·전하·분극의 conservative remap과 같은 coupling work를 한 trial에서 해결한다. 전자기력은 일관된 이산 에너지/가상일 또는 검증된 stress·momentum balance에서 구하고, 변형·이동에 한 일이 장과 물질 양쪽에 반대 부호로 들어가야 한다.

진공 Lorentz force만 넣고 bound charge·magnetization force를 누락한 구현은 최종 해법이 아니다. 위상 경계의 힘·재료 이동 remap·전도체 접촉/절단은 이번 개정의 가장 큰 수치 연구 gate다. 이 단계가 미해결이면 frozen-material 파동 실험까지만 통과한 것으로 표시한다. 에너지 잔차를 주변 열로 임의 분배해 통과시키지 않는다.

### coherent field와 비간섭 복사의 분리

CT1은 모든 파장을 하나의 Maxwell 해석으로 푸는 모형이 아니다. coherent sector W는98절의 resolved planar field, incoherent sector R은87절의6band 방향성 에너지 packet이다. W와 R은 서로 다른 유효 대역으로 선언하고 RadiationBandMap에 단위·응답 범위를 기록한다.6band의 모형 μm 수치를 그대로 tile 파장으로 바꿔 W와 중첩하지 않는다.

각 물질의 oscillator/transition은 W 또는 R 중 하나의 owner를 가진다. R의 흡수·방출 전이를 같은 에너지로 W에도 여기하지 않는다. 태양/열 복사는 R로 들어가고, 저주파 전도·분극은 W에서 전파한다. 같은 region이 두 sector와 상호작용할 수는 있지만 전이 채널·에너지·활성 범위가 분리되어야 한다.

R packet에는 위상이 없으므로 W field로 임의 위상을 붙여 승격하지 않는다. 재료가 실제로 흡수한 에너지를 chemical/work로 저장한 뒤 W의 유료 전류를 구동하는 변환은 가능하다. 반대로 W의 흡수열이 R로 재방출될 수 있다. 모두 실제 물질 상태를 거친다. 열평형 시험에서는 두 sector를 통과하는 cycle로 일을 만들지 못해야 한다.

이 분리는 계산량을 줄이는 유효 모형의 제한이다. 태양광의 완전한 간섭, 연속 전 스펙트럼의 Maxwell 통일, 실제 레이저의 양자 과정을 지원하지 않는다. W 대역 안에서는 간섭·회절·광학적 구조를 시험할 수 있다. 훗날 sector 사이의 보존형 multiscale coupling이 검증되면 새 ruleset으로 확장한다.

### 외부 작용과 수치 오차

한 substep의 계약은 Δ(Ufield+Uconstitutive+K+Uelastic+Uthermal+Echem+Uwork)+방사유출−외부유입=Residual이다. Residual은 비소비성 진단값이다. 물리 손실과 PML 유출은 실제 장부에 각각 기록한다. 저항·흡수 손실을 한 번 열에 보낸 뒤 “수치 안정화” 명목으로 다시 없애지 않는다.

Gauss 오차, passive medium의 에너지 증가, topology change의 순간 에너지 점프, material remap의 비국소 정보 이동을 별도 gate로 둔다. reference 이하의 오차가 나왔다고 실제 무선 기술의 발견을 의미하지는 않는다. 구조 제작·유지·송수신 프로그램의 전달 증거가 추가로 필요하다.

**경계:** W는 장·재료·복사의 에너지 교환과 실제 힘이다. P는 기존 유료 물질·전하·힘 조작이다. E는 유도·무선·광변환·구조 신호 처리다. U는 대역·소유권·손실·출력과 검증 수준을 보여준다.

## 100 환경 진화: 지형·침식·퇴적·수문을 실제 재고로 연결한다

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 모델 선택

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 결정 |
| --- | --- | --- | --- |
| 주기적 지형 재생성·재난 event | 싸고 변화가 눈에 보임 | 과거 구조·물질 장부를 끊음, 환경이 연출에 종속 | 제외 |
|3D voxel 지질·완전 대기/해양 | 지하와 복잡한 유동의 넓은 표현 | 공간 자유도 확대와 계산비 폭증 | 이번 범위 제외 |
|2D 표면+재고를 가진 지질/기체 column | 물질 이력·침식·수문·생태 연결 가능 | 수직 구조가 coarse하고 동굴·단층 운동 제한 | **선택** |

각 surface cell은 bedrock surface, 퇴적층의 순서, pore stock, surface water, 실제 경계면을 가진다. column의 층은 원소·화학종·기공·열·물성의 기록이다. agent가 각 층을 자유롭게 돌아다니는 다층 공간은 아니다. 지하 동굴·지하 도시·자유로운 비행을 이번 개정에 끼워 넣지 않는다.

층의 높이는 재고량·밀도·기공률과 면적에서 계산한다. 먼저 높이를 바꾼 뒤 흙을 무한 생성하지 않는다. 깊은 bedrock도 초기 원소·질량을 가진다. 완전 소진되면 바닥 경계가 드러나며 새 암석을 보충하지 않는다. 심부 reservoir를 사용하는 run은 그 유한 재고 또는 외부 유입을 BoundaryLedger에 공개한다.

### 물과 침식의 수송

reference 수문은 양수 수심을 보장하는 finite-volume shallow-water 형식이다. 물 깊이 h, 단위 면적 운동량 h·u, 용질/부유 입자의 양을 저장한다. face flux 하나를 양쪽 cell에 반대 부호로 반영한다. hydrostatic reconstruction을 가진 well-balanced flux를 선택하고, 정수 재고로 양자화한 차이는 face remainder에 저장한다. 호수의 수면이 평평하고 정지해 있으면 지형 경사 때문에 가짜 흐름이 생기지 않아야 한다.

침식률은 국소 유체 traction, 재료의 결합·입자 크기·노출에 의존한다. “강 타일”만 깎이는 branch는 없다. 침식은 bedrock/퇴적층에서 실제 stock을 제거하여 suspended 또는 bed-load stock으로 옮긴다. 탈착·파괴·들어 올리기에 필요한 일은 유체/기계 에너지에서 빠진다. 운반 에너지가 부족한데 목표 침식률을 만족시키려고 재료를 삭제하지 않는다.

침강은 유체·입자의 밀도·점성·입자 형상으로 제한한다. 퇴적된 질량은 새 층 또는 같은 상태의 최상층에 더한다. 낙하·마찰의 에너지는 열·유동으로 보낸다. 수송된 광물의 원소와 화학적 정체성은 남고, 다른 종으로 변하려면96..97절 반응을 거친다. 중력 유동과 물질 보존을 분리된 terrain 효과로 구현하지 않는다.

Landlab SPACE는 표면 세계에서 암반 침식과 퇴적물 이동을 함께 다루는 선례다. 본 설계는 그 구조를 참고하되 지형 높이뿐 아니라 전체 화학 재고와 일·열 장부까지 연결해야 한다. [Landlab SPACE](https://landlab.readthedocs.io/en/latest/tutorials/landscape_evolution/space/SPACE_user_guide_and_examples.html)

### 지형과 기존 몸체의 접촉

agent와 artifact의 좌표 자유도는 계속2D다. 표면 높이 b(x,y)는 지지면의 제약과 Ug를 정한다. 경사를 오르내릴 때 실제 높이 변화에 필요한 일을 청구한다. 마찰·접촉은 표면 geometry를 사용한다. 높이를 바꾸면서 위에 놓인 몸체를 공짜로 들어 올리지 않는다.

퇴적이 물체 밑 지지면을 올리면 퇴적/유체가 물체에 한 일을 같은 transaction에 반영한다. 침식으로 지지가 사라지면 접촉 solve를 다시 하고 수직 정착의 위치 에너지를 손상·열·유동 등 명시된 경로로 보낸다. 수직 trajectory는 coarse 제약 해법이며 자유낙하·점프·비행을 제공하지 않는다. 이 제약 해법이 보존 gate를 통과하기 전에는 움직이는 지형과 몸체 결합을 완성으로 표시하지 않는다.

### 지층·풍화·심부 열

표면 층만 직접 접촉할 수 있다. 물·기체는 기존 permeability에 따라 pore를 지나고 노출된 반응 중심에서 용해·침전·풍화가 진행된다. 날씨라는 label로 암석이 사라지지 않는다. 층은 조성·열·기공·손상 상태가 완전히 동등한 인접 구간만 정확히 병합한다. 색이 같거나 오래됐다는 이유로 물리적으로 다른 층을 합치지 않는다.

최초 CT1은 태양 입력과 유한 초기 내부 열을 사용한다. 선택적인 basal heat boundary는 명시적 외부 에너지 공급이며 global ledger에 들어간다. 용융·pore pressure·표면 파괴·유출은 기존 상변화·압력·결합 법칙으로 연결할 수 있다. 마그마 chamber의3D 역학, 판 운동, 분화구/화산 event 생성은 이번 모델이 제공하지 않는다. 작은 열수 분출 witness와 행성 지질 재현을 구별한다.

**경계:** W는 중력·유체·파괴·퇴적·pore 수송과 유한 재고다. P는 기존 물질 이동·구조 배치·힘·열이다. E는 강 유로 변화·침식 지형·매몰·토양 변형·서식지 변화다. U는 지층 이력·물질 수지·유역과 변화 원인을 표시한다.

## 101 대기 조성·복사·장기 기후와 생태의 피드백

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 모델 선택과 범위

기후 event table은 싸지만 화학 조성과 연결이 약하다.3D 대기 대순환 모델은 강력하지만 이번2D 문명 모델의 예산과 맞지 않는다. **선택은 보존형 기체 column과 수평 수송, 실제 복사·증발·응결의 축약 모델**이다. 지구 기후 예측기가 아니라 조성·물·열·지형·생물이 서로 영향을 주는 인공 환경이다.

AtmosColumn은 species amount, sensible energy, 응결된 물, 수평 운동량, optical depth, hydrostatic 상태를 가진다. 기체 조성은82절의 species를 사용한다. CO2·O2·H2O처럼 observer가 이름 붙일 수 있어도 행성의 산소 시대·온실 시대를 코어 enum으로 만들지 않는다. 광화학 반응도 실제 radiation/반응 경로를 요구한다.

### column 열역학과 수송

압력·밀도·높이·열용량은 같은 EOS와 hydrostatic closure에서 구한다. reference closure는 column의 질량·조성·entropy·경계 압력에 대해 내부 에너지와 중력 에너지의 일관된 상태를 정한다. 압력 구배가 수평 유동에 한 일, column 팽창·수축, 표면과의 열 교환을 같은 total-energy equation에 포함한다.

먼저 단순 고정 기하 기체층의 폐쇄 수송 fixture로 원소·열을 검증한 뒤 가변 hydrostatic column으로 진행한다. 이 두 조건을 같은 기후 profile로 몰래 바꾸지 않는다. 구체 EOS·수직 분포·압력 일 flux는107절의 미동결 수치 항목이다. 물질량만 보존하는 예쁜 구름 그림을 완성된 대기 동역학으로 주장하지 않는다.

수평 flux는 donor가 가진 실제 기체·물·열을 이웃으로 옮긴다. 낮은 해상도 atmosphere grid와 fine surface의 교환은 고정된 area weight의 합이1이어야 한다. 대류·난류의 미해상 수송은 양수 소산을 가진 공개 closure를 쓴다. wind map을 임의 갱신하여 에너지와 용질을 무한히 이동시키지 않는다.

### 물 순환과 해수면

증발은 surface water stock을 atmosphere H2O로 옮기고 잠열을 빼며, 응결은 반대 변환과 열 방출을 한다. precipitation은 응결 stock이 중력·column 수송 조건을 만족해 내려오는 흐름이다. 임의 rain event가 물을 생성하지 않는다. 외부 기상 forcing을 넣는 연구 run은 물·에너지 수지를 경계 유입으로 명시한다.

호수와 바다는 특별한 무한 저장소가 아니다. 실제 액체량과 basin geometry에 따라 수면이 결정된다. 유출입·얼음·증발·퇴적이 달라지면 수면과 해안선이 바뀔 수 있다. 전 세계 하나의 sea_level 값을 독립적으로 올려 침수시키는 조작은 자연 run에 없다. 화면에서 계산된 평균 해수면은 observer 통계다.

### 복사와 생명 사이의 결합

태양 에너지는 R7의 R-sector boundary에서 한 번 유입한다. atmosphere의 광학 깊이는 실제 column 조성과 물성에서 계산한다. 대기에 흡수된 양을 표면에도 동시에 지급하지 않는다. 표면·기체의 방출은 같은 band의 흡수와 일관된 Kirchhoff형 계약을 따르고 실제 내부 에너지를 소모한다. 구름·기공·재료·피복이 바뀌면 반사·흡수·열 이동이 바뀐다.

생물은 광흡수·원소 고정·기체 방출·영양염 소비·분해로 이 상태를 바꾼다. 바뀐 조성이 대사·재료 산화·용해·복사에 영향을 준다. 식물이 많다는 이유로 비가 자동 증가하는 보정식은 없다. 복사 흡수계수를 가진 어떤 분자가 증가했을 때 어떤 조건에서 온도가 얼마나 바뀌는지는 실제 모델과 경계 조건에 의해 정해진다.

### 긴 시간척도와 환경 개입

모든 과정은 같은 tick 시간 단위를 쓴다. 지형만1,000배 빠르게 변화시키거나 기후를 문명 발전 속도에 맞춰 조절하지 않는다. 느린 상태는 누적 flux를 저장하면서 결정론적 due schedule과 오차 제한으로 갱신할 수 있다. 그동안의 에너지·반응·차폐 효과가 정확히 유지되지 않으면 subcycle해야 한다.

계절·일주기는 시작 manifest의 외부 경계 schedule로 허용한다. agent의 성과·다양성·population을 보고 난이도를 바꾸는 숨은 환경 조절기는 없다. 영양염 고갈·장기 냉각·산소화·멸종·매몰은 허용되는 역사다. 단조로운 정지 상태나 과도한 파괴가 잦으면 결과를 보존하고 다른 초기 조건의 새 실험으로 검토한다.

**경계:** W는 기체·물·열·복사·중력과 외부 경계 입력이다. P는 별도 기후 조작 없이 기존 물질·열·조직 활동이다. E는 생명과 환경의 장기 공진화·자원 조건 변화다. U는 기후·대기 조성·해수면·서식지와 변화의 수지를 표시한다.

## 102 표현 진화: 고정 VM 위에 자기 언어와 해석기를 만든다

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 비교와 선택

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 결정 |
| --- | --- | --- | --- |
| 새 opcode를 host에 등록 | 빠른 고수준 연산 가능 | 세계 법칙·비용·권한을 바꾸는 우회, 재현성 붕괴 | 금지 |
| 반복 행동을 host가 자동 macro로 승격 | 학습·실행 최적화 쉬움 | 개발자가 의미 추출과 추상화를 대신함 | 관찰 분석에만 허용 |
| 기존 VM 프로그램이 데이터를 해석 | 언어·문법·compiler·자료구조가 진화 가능 | 느리고 기억/복사 비용 큼, 탐색 성공 보장 없음 | **선택** |

표현의 진화는 host VM의 instruction set·물리 법칙·chemistry schema가 진화한다는 뜻이 아니다. 개체가 기존 연산으로 새로운 데이터 형식과 해석 절차를 만들고 전달하는 것이다. 유한 자원 안에서 계산상 새로운 조직이 생길 수 있지만, host에서 원래 계산 불가능했던 물리 효과를 발명할 수는 없다. “탐색 공간 자체가 무한 확장된다”는 주장은 하지 않는다.

### 실행 가능한 최소 경로

기존 VM의 산술·조건 분기·indexed local read/write·CALL/RETURN으로 guest interpreter를 구현할 수 있어야 한다. 검증용 예는 self memory의 token을 읽고, 그 값으로 dispatch하고, local data를 바꾸고, 허용된 기존 host service를 호출하는 절차다. 이 손작성 interpreter는 회귀 fixture다. 자연 조상의 genome에 넣어두고 언어가 자연 출현했다고 세지 않는다.

표현 기록은 raw byte/word이고, interpreter도 일반 BehaviorProgram이다. 명령 token의 길이·구분자·문법·데이터형·호출 규약은 guest 프로그램이 정한다. host는 token7을 “운반”이라고 해석하지 않는다. 자기 언어를 해석하는 interpreter를 또 다른 자기 언어로 작성할 수도 있지만, 결국 실행은 동일한 유료 base VM operation으로 귀착된다.

### 비용·시간·권한

GuestState는 VM의 실제 paid memory 안에 있다. 별도 무료 host stack·map·parser 객체를 만들지 않는다. guest instruction 하나가 base instruction100개를 실행하면100개 비용과 그에 필요한 tick을 쓴다. R7의 controller당32 base instruction/tick 후보 상한과 query4/effect1을 공유한다. guest nesting마다 새로운 quota를 주지 않는다.

guest loop·재귀·잘못된 token은 base VM 시간·기억 한도에서 중단/재개·실패한다. timeout은 host CPU wall-clock이 아니라 모델의 instruction quota다. parser가 긴 문법을 읽다가 멈추면 paid memory와 PC에 실제 진행이 남아 다음 tick에 이어진다. 쓰레기 같은 코드의 무한 실행도 전 세계 thread를 붙잡지 않는다.

guest의 수·문자열을 world handle로 직접 cast할 수 없다. I/O 권한은89절의 현재 국소 frame과 실제 transducer에서 나온다. guest 안에 저장한 stale handle은 base service에서 동일하게 실패한다. 다른 개체의 code·memory·template을 읽는 데 국소 접근·매체 비용을 생략하지 않는다.

### 자기 수정·번역·채택

해석할 데이터는 일반 WRITE로 바꿀 수 있다. 실행할 base code로 번역하려면 자기 draft record에 유료로 word를 쓰고 기존 code-adoption 경계에서 검증·채택한다. code의 semantic value를 판단하는 host compiler는 없다. host verifier는 encoding·메모리 범위·권한 형식만 검사한다. 실행 중인 frame의 code revision은 고정하고, 새 revision 채택은 다음 명시적 call/activation 경계에 적용한다.

compiler가 더 효율적인 코드를 만들면 실제 base instruction 수 감소의 이득을 얻을 수 있다. 대신 compiler·원문·중간 결과의 제작·저장·전달 비용을 낸다. host JIT는 CT1 reference에서 끈다. 나중에 적용하더라도 같은 base instruction·I/O·에너지·tick trace를 보존하는 cache 최적화만 허용한다. 계산 비용을 삭제하는 JIT는 새로운 세계 법칙이다.

### 문화·유전·진화 연산자의 경계

interpreter와 data는 실제 매체를 통해 따로 또는 함께 전달된다. data만 받은 개체가 대응 interpreter를 갖지 못하면 무의미할 수 있다. 불완전한 기록·규약 불일치·다른 신체의 I/O 차이 때문에 기술이 퍼지지 않거나 사라질 수 있다. host가 language ID를 보고 translator를 자동 제공하지 않는다.

생애 중 습득한 code는 germline에 자동 반영되지 않는다. 실제 생식 기록에 복사되었을 때만 기존 유전 경로를 따른다. 돌연변이 연산자 자체를 프로그램으로 바꾸는 기능은 이번 범위에 없다. 프로그램이 자신의 learned code를 수정하는 능력과 host 복제 변이 법칙의 진화를 구별한다.

### 관찰 증거

표현 family는 code similarity만으로 판정하지 않는다. interpreter revision, raw record, dispatch trace, producer/consumer relation, 실제 I/O, 전달 출처를 증거로 묶는다. 반복되는 token만 많으면 새 언어가 아니다. 관찰용 분기에서 interpreter나 decoder를 제거/교체했을 때 기능이 달라지는지 확인할 수 있지만, 그 intervention을 원본 run의 자연사에 섞지 않는다.

R0는 base VM이 guest 해석을 표현할 수 있음, R1은 손작성 guest fixture 실행, R2는 비용을 포함한 유용성, R3는 주입 없이 실제 출현, R4는 서로 다른 개체/계통 사이의 실제 전달·수정·누적이다. 표현의 계층이 깊다는 이유로 기술 점수를 올리지 않는다. 짧은 직접 코드가 더 유리한 경우도 허용한다.

Taylor의 개방형 진화 논의는 탐색 공간 안의 변화와 새로운 조직·영역 간 결합을 구분할 필요를 설명한다. 본 설계의 guest-language 방식은 그 문제를 다루기 위한 제한된 제안이며 transformational open-endedness 달성의 증명은 아니다. [Taylor, Evolutionary Innovations and Where to Find Them](https://www.tim-taylor.com/paper-details/taylor2019evolutionary.html)

**경계:** W는 고정 base VM·유료 기억·국소 권한·복사 비용이다. P는 기존 계산·분기·CALL·READ/WRITE·code 채택이다. E는 자기 언어·interpreter·compiler·계층적 절차·문화 규약이다. U는 표현 family·의존성·실행/전달 증거를 표시한다.

## 103 네 축의 결합과 simulation phase

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 같은 세계 안의 상호작용 경로

| 출발 상태 | 실제 연결 경로 | 가능한 관찰 결과 |
| --- | --- | --- |
| 새 고분자와 기공 구조 | 투과·기계 강도·광/장 응답 변화 | 막·수동 필터·생체 재료 후보 |
| 촉매 표면과 흐르는 물 | 흡착·반응·수송·침전 | 환경에서 반복되는 재료 제작 |
| 생물의 기체 고정·분해 | 대기 조성·복사·산화·용해 변화 | 생태 niche 이동·기술의 유용성 변화 |
| 퇴적과 물 유로 변화 | 재료 매몰·접촉 단절·영양염 이동 | 정착지·물류·기록 보존 조건 변화 |
| 반복적인 전하 구동 | 장 전파·산란·다른 재료의 응답 | 신호 중계·무선 상호작용 |
| 실제 매체와 interpreter | 국소 복사·유료 해석·실제 actuator | 새로운 표현 family와 절차 재사용 |

이 표는 출현 목표의 예이며 시뮬레이션의 단계·퀘스트·보상 함수가 아니다. material→wave→language가 반드시 그 순서로 발견될 이유도 없다. 과거 구조의 우연한 재사용이나 여러 계통의 다른 조합을 허용한다.

### 단일 tick의 계약

1. Preflight: 입력·경계 schedule·수치 backend·material revision을 고정하고 world journal을 연다. 환경 due work와 EM step 계획을 결정한다. 메모리 reserve 실패는 생물의 action 실패가 아니라 RunHalt다.
2. Sense/Think/Develop: 기존 R7 controller·발생 프로그램을 읽기 snapshot에서 실행한다. guest 해석은 이 VM 실행 안에 포함된다. 아직 commit하지 않은 원격 field·새 화학종·관찰 label에 접근하지 않는다.
3. Intent reservation: 실제 stock·work·접촉·기억·공간을 예약한다. 합성·물질 이동·code draft·힘 목표를 transaction으로 기록한다. 여러 subsystem이 동일한 원료와 에너지를 예약하지 못한다.
4. Coupled trial: 반 단계 반응/수송, mechanics·조립·환경·EM subcycle의 결합, 나머지 반 단계 반응/수송을 진행한다. 분극·자화·전도·geometry remap·수문 flux·생체 작업은 같은 시간 구간의 에너지 교환으로 해결한다.
5. Radiation/environment: R-sector propagation·흡수·방출·column 교환을 실제 시간 위치에 반영한다. 느린 환경 단계는 양방향 flux ledger를 사용하며 과정 순서 편향은 substep convergence로 검증한다. 태양·비·basal heat를 iteration마다 반복 지급하지 않는다.
6. Audit/retry: 원소·전하·Gauss·에너지·양수 stock·층 부피·비관통을 검사한다. 실패하면 해당 coupling 구간의 모든 상태와 전파된 boundary flux를 rollback한다. 연결이 전 세계로 퍼졌으면 tick 전체를 되돌린다.
7. Viability/adoption: 실제 분할·사망·소진을 반영하고 code revision 채택 경계를 확정한다. 사망이 조립체·퇴적물·장·기록을 삭제하는 명령이 되지 않는다.
8. Commit: canonical 원인 순서로 ID·event·pending species를 확정한다. 완료 tick hash와 read-only snapshot을 발행한다. observer 결과는 다음 tick 법칙의 입력이 아니다.

### 다중 시간척도와 재시도

위 목록은 결합의 위치를 정한 것이며 모든 과정을 한 번씩 순서대로 호출하면 정확해진다는 주장이 아니다. stiff reaction, 빠른 EM, 느린 퇴적을 여러 substep으로 푼다. 각 coupling interval의 시작 상태를 유지하고 interface work·flux가 수렴할 때까지 같은 구간을 재계산한다. solver 내부 반복은 실제 세계 시간을 추가 진행시키지 않는다.

reference retry는 가장 작은 유효 timestep의2배 분할로 수행한다. 실패 횟수·worker 수·카메라가 난수 key나 물리 계수를 바꾸면 안 된다. local retry가 다른 chunk로 전달한 전하·빛·퇴적물·파동을 되돌릴 수 없으면 global rollback으로 승격한다. 이는 성능비가 크므로105절 workload의 retry율을 함께 측정한다.

**경계:** Engineering Contract는 단계·시간·rollback·소유권이다. W/P는 명시된 transaction으로만 상태를 바꾼다. E는 네 축을 가로지르는 실제 상호작용이다. U는 coupling 원인과 실패·오차를 표시한다.

## 104 Rust 자료형·system·저장·관찰 인터페이스

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### crate와 data ownership

기존 civ_types, civ_numeric, civ_vm, civ_chem, civ_bio, civ_physics, civ_core, civ_runner, civ_observer, civ_viewer를 유지한다. civ_material은96..97절 microstructure·patch·homogenization 계약, civ_environment는100..101절 surface/column flux를 담당하는 새 라이브러리다. civ_physics::em은 grid·constitutive·current deposit·boundary·coupling을 가진다. civ_vm에는 별도 guest-language executor를 추가하지 않는다.

공통 types/numeric은 다른 subsystem을 의존하지 않는다. chemistry는 SpeciesDef와 thermo evaluation, material은 microstate와 MaterialResponse를 만든다. physics와 environment는 같은 MaterialResponse DTO를 읽는다. bio는 화학·VM의 일반 계산을 호출한다. core가 모든 authoritative store를 소유하고 phase별 view·delta writer를 넘긴다. DTO를 통한 data flow와 crate 의존을 구별하여 상호 의존 singleton을 만들지 않는다.

| type | 주요 필드·저장 단위 | owner |
| --- | --- | --- |
| ContinuumWorld | R7 stores, micro, em, surface, atmosphere, exchange ledgers | core |
| MicroNodeSoA | MaterialSliceKey, x/y/angle, momentum, patch span, revision | MicroStore |
| MicroBondSoA | patch endpoints, binding, strain, damage, energy owner | MicroStore |
| MaterialResponse | phase/thermo, mechanical/transport terms, passive EM terms, validity | 파생 view |
| EmGrid | topology hash, D/B arrays, charge projection, constitutive spans, subtime | EmStore |
| PolarizationState | P, Pdot, material slice, energy/loss terms | EmStore |
| CurrentDeposit | face/edge key, integrated charge, interval, source transaction | trial ledger |
| SurfaceColumn | area, ordered strata span, water/pore stock refs, thermal/geometry state | EnvironmentStore |
| Stratum | StockLot span, porosity, damage, volume, deposition provenance | EnvironmentStore |
| AtmosColumn | species span, energy, condensed water, momentum, closure state | EnvironmentStore |
| ExchangeLedger | source/target keys, atoms[8], charge, heat/work, interval, remainder | AuditStore |
| CodeRecord | existing physical backing, raw words, code revision, copy provenance | R7 TemplateStore |
| RepresentationEvidence | interpreter/data hashes, traces, transfer edges, confidence | observer only |

Atom counts와 Amount는 정수 보존, EM·유동·구성식 상태는 deterministic numeric backend를 사용한다. 표의 Rust field 순서·padding은 wire schema가 아니다. SoA pointer나 native enum discriminant를 save에 그대로 쓰지 않는다. 실제 primitive size·packing·vector alignment는 구현 size report로 검증한다.

### system 계약

주요 system은 gather_local_contacts, enumerate_reactions, reserve_material_slices, integrate_microstructure, deposit_conservative_current, step_em_constitutive, couple_field_work, solve_surface_flux, update_strata, exchange_atmosphere, step_radiation, audit_continuum, commit_code_revision이다. 각각은 read view와 자기 delta span만 받고, Core coordinator가 충돌·rollback 경계를 소유한다.

공간적으로 독립한 VM·반응 후보·micro pair 생성·EM stencil interior는 병렬 처리한다. face flux는 canonical face owner가 한 번 계산한다. EM halo 교환은 substep barrier 뒤에 이루어진다. giant field domain에서 global conservation audit·환경 closure·기하 remap이 병목일 수 있다. chunk 병렬만으로 모든 비용이 선형 축소된다고 가정하지 않는다.

### primitive와 ABI

CT1에서 추가 고수준 action opcode는0개다. 기존 READ/DRIVE/TRANSFER/WORK/LINK/WRITE/COPY/EXPRESS와 VM 계산을 사용한다. 장 센서는 실제 material transducer의 국소 응답만 돌려준다. observer의 정확한 global E/B, 지층 전체 조성, 상대 interpreter 의미를 직접 읽게 하지 않는다. 전하를 바꾸는 발현·작업은 실제 carrier 이동·국소 전류·에너지 비용으로 해석한다.

ABI-CT1은 ABI-ML1과 ProgramHeader에서 구분한다. 새 sensor payload는 표면 strain·주파수 응답·접촉 재료·환경 값의 기존 측정 형태를 확장한다. 정확한 service 숫자·scale·unit·saturation·error 우선순위는 ABI 동결 자료가 필요하다. 이번 문서가 schema 이름만으로 전체 binary layout을 완료했다고 표시하지 않는다.

### save/replay

CT1 magic은8byte CIVCT001이다. manifest에는 ChemistryModel, MicrostructureModel, RadiationBandMap, EmGridSpec, ConstitutiveData, EnvironmentClosure, BaseVmSemantics, NumericBackend의 hash를 추가한다. R7 save는 legacy backend에서 재생한다. 변환 branch는 새로운 초기 장과 에너지·기체·지형 상태를 만드는 개입이며 같은 자연사의 무변경 연속이 아니다.

필수 상태는 미세구조·binding 점유·물질 slice 소유권, D/B의 staggered 시간 상태, 모든 분극/자화/손실 기억, 물질 이동 remap 상태, 경계 흡수층의 미래에 영향을 주는 상태, 지층·pore·대기·잠열 상태, flux remainder, 코드 raw record·진행 PC·기억이다. 끝난 tick에서도 EM half-step 상태를 맞춰 저장해야 한다. 미래에 영향을 주는 파동 phase와 solver history를 cache로 분류해 버리지 않는다.

### UI와 관찰의 경계

far zoom은 지형·수계·생체량·기후·관측 집단, mid zoom은 구조·재료·신호 경로·퇴적 변화, close zoom은 접촉·조립·실제 장/재료 반응을 보여준다. 파동 overlay의 색은 field 값이며 문명 통신의 의미가 아니다. 실시간 GUI를 위해 모든 field cell을 Bevy Entity로 만들지 않는다. chunk texture와 제한된 instance snapshot을 사용한다.

새 event 후보는 MaterialMicrostructureFamily, CatalyticFluxPattern, WaveInteractionPattern, PersistentEnvironmentalShift, RepresentationFamily다. 이들은 observer schema이며 world enum이 아니다. 새 화학종·장 공진·token 통계만으로 NEW TECHNOLOGY를 확정하지 않는다. 실제 기능·재사용·전달과 R0..R4 증거를 같이 보인다.

**경계:** Engineering Contract는 crate·SoA·system·wire·snapshot이다. W/P는 기존 저수준 권한을 유지한다. E는 물리 상태와 실제 code 실행에서 발생한다. U는 해석과 raw evidence를 분리한다.

## 105 메모리·성능 예산과 현실적인 실행 profile

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 파동 비용의 하한 계산

Δx=0.25tile이면 타일당16개 EM cell이 필요하다. c=32tile/tick, S=0.5에서256step/tick이다. 아래 값은 계산된 cell update 수이며 실행 속도 측정이 아니다. 재료 oscillator·J deposition·PML·halo·retry·생물·화학 비용은 별도다.

| world | EM cell 수 | 256step의 update/tick | D/B 6×f64 payload |
| --- | --- | --- | --- |
|64×64 |65,536 |16,777,216 |3MiB |
|128×128 |262,144 |67,108,864 |12MiB |
|512×512 |4,194,304 |1,073,741,824 |192MiB |
|3,200×3,200 |163,840,000 |41,943,040,000 |7,500MiB |

48byte/cell은6성분 array만의 산정이다. E/H cache·ρ·재료 계수·polarization·magnetization·경계 state·journal·alignment를 포함하지 않는다. 예상 planning allowance를192byte/cell로 잡으면512²world의 field 영역만768MiB,3,200²world는약29.30GiB다. snapshot/rollback 복사와 기존 R7 world memory가 추가된다.192byte는 measured struct size나 상한이 아니다.

큰 세계에서 기존 R6의10TPS 목표를 유지한다고 쓰지 않는다. full-wave와100k controller/2M cell의 동시 실행 성능은 현재 미정이다.1x가10TPS라는 UI 정의도 실제10TPS를 보장하지 않는다. MAX는 처리 가능한 한 빨리 진행할 뿐 physics step을 건너뛰지 않는다.

### 다른 새 상태의 예산

MicroNode를112byte, MicroBond를64byte로 가정하면1M node+2M bond는240,000,000byte, 약228.88MiB다. patch·neighbor·stock·journal은 별도다.100k surface column×128byte는약12.21MiB,400k stratum×64byte는약24.41MiB다. atmosphere는 species sparse span과 closure 상태를 더해야 한다. 이 수치는 layout 가정에 따른 payload 예산이며 컴파일 결과가 아니다.

표현 진화의 guest heap은 기존 유료 controller memory 안에 포함한다. interpreter에 무료 host hashmap을 붙이는 편법으로 성능을 얻지 않는다. code canonicalization·공유 immutable byte cache는 가능하지만, 각 개체의 물리적 기록·복사·기억 비용은 유지한다. chemistry graph canonicalization과 새로운 종의 수가 또 다른 병목이 될 수 있다.

### 개발 profile과 release gate

CT1-Reference는64²world,100controller,2k Cell,10k MicroNode,20k MicroBond, 전체 Maxwell·화학·환경·R-sector·guest fixture를 목표 workload로 둔다.12physical-core 데스크톱32GiB, release build에서1TPS를 초기 측정 목표로 삼되 미실측이다. 수치 보존을 먼저 통과하고, benchmark에는 CPU 모델·compiler·backend·시간 step·active/total EM cell·종 수·retry율·p50/p95 TPS를 기록한다.

CT1-Research는128²world,1kcontroller,20k Cell,100k MicroNode,200k MicroBond를 다음 workload로 둔다. 정확한 기능·저장·replay 통과가 우선이며 TPS의 합격 수치는 reference 실측 후 동결한다.512² 및1,024만tile의 full-wave 규모는 비용 평가 대상이다. 아직 성능 gate를 통과한 최종 제품 규모로 표시하지 않는다.

성능이 부족할 때 기본 순서는 재료 평가 cache, SoA와 stencil SIMD, deterministic parallel halo, 정확한0영역 생략, coupling/journal 최적화다. 그 다음에야 검증 가능한 model reduction을 별도 ruleset으로 연구한다. world 크기를 줄이는 것은 같은 법칙의 초기 조건 변경이지만, c·Δx·물성·파동 생략을 바꾸는 것은 물리 모델 변경이다.

### 선택의 의미

이 설계는 사용자가 고른 표현력을 기준으로 reference를 정했다. 동시에 수십만 개체·수천만tile·full-wave·높은 TPS를 달성한다는 근거는 없다. 가장 큰 성능 위험이 파동장이라는 점을 숨기지 않는다. 먼저 작은 세계에서 새로운 현상이 실제로 가능하고 쓸모 있는지 확인한 뒤 확장한다. benchmark 통과를 위해 일부 계통에만 정밀 물리·해석 비용 면제를 주지 않는다.

**경계:** Engineering Contract는 payload·workload·실측 gate다. W/P의 결과를 CPU·카메라에 따라 바꾸지 않는다. E의 풍부함과 실제 탐색 속도는 별도 측정한다. U는 요청 배속과 달성 TPS·메모리·모델 profile을 표시한다.

## 106 R8 검증 명세:64개 기대 사례와 복합 witness

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

아래 Y01..Y64는 앞으로 구현할 solver·core가 만족해야 할 기대 사례다. 이 문서 작업에서 시뮬레이터 시험을 실행했다는 뜻이 아니다. 기존 Grid-5 T01..T97, Matter-1 P01..P36, MatterLife X01..X64는 해당 legacy profile의 regression으로 보존한다. R8에서 교체한 모델에 기존 숫자를 무조건 적용하지 않는다.

| ID | 입력·조건 | 기대 결과·판정 |
| --- | --- | --- |
|Y01| 자유 재고100에서 조립20 | 자유80+조립20; 원소 총량 불변 |
|Y02| 조립20을 용액 반응 후보가 재사용 | 이중 예약 거부; 총 소비≤100 |
|Y03| 하나의 pair potential에서 F와 torque 계산 | finite-difference 에너지 미분과 step 수렴 |
|Y04| U결합=−3인 결합 형성, 다른 저장 변화 없음 | 주변 열/일 합이+3; 결합과 표면 항 중복 없음 |
|Y05| 같은 결합을 다시 절단 | 최소3의 공급 또는 다른 실제 에너지 감소 |
|Y06| 촉매가 정·역 장벽을2RT씩 낮춤 | 두 rate는각 exp(2)배, 평형 비율 불변 |
|Y07| 활성점1개에 substrate2개 동시 접근 | occupancy≤1; 나머지는 실제 재고로 남음 |
|Y08| 촉매 poisoning 반응 | 점유/새 종과 반응 수지 존재; 임의 능력 삭제 아님 |
|Y09| 같은 조성, 다른 배향·가교 | reference 응답 차이를 실제 geometry로 설명 |
|Y10| 상분리 폐쇄계의 face flux | 원소 정확 보존, 자유 에너지 감소/열 정산 |
|Y11| 서로 다른 representation을 중첩 | double energy owner 검출·실행 거부 |
|Y12| assembly 분해·죽은 생체 재료 | 실제 물질·열·전하 유지, label로 삭제 없음 |
|Y13| payoff 없는 종 이름만 바뀜 | innovation 후보라도 기술 확정 안 함 |
|Y14| 자유 재고0·결합 점유1의 종 | GC 금지; 살아 있는 참조 보존 |
|Y15| closed catalytic cycle | 외부 투입 없이 순 work 증가 없음 |
|Y16| dx=.25,c=32,S=.5 | dt=1/256tick, 최소256 EM step/tick |
|Y17| discrete curl/div stencil | div(curl) 수치 항등/정밀도 gate 통과 |
|Y18| 전하 q를 이웃으로 이동 | 연속식과 Gauss 오차가 통합 허용 범위 |
|Y19| 진공 평면파, λ≥16cell | 해상도 증가 시 위상속도·전송 오차 수렴 |
|Y20| 같은 위치 두 장 contribution E=1,−1 | 총 E=0; 에너지로 선형 합산하지 않음 |
|Y21| equal-path coherent interference | 위상차0과π에서 서로 다른 field intensity |
|Y22| slit 크기와 파장 변화 | 회절 pattern이 reference와 수렴 |
|Y23| 유한 재료 cavity | resonance가 geometry/constitutive에 의존 |
|Y24| passive polarization a=2,ω0=3,P=2,Pdot=0 | oscillator energy=9 model energy |
|Y25| γ>0, 외부 source0 | oscillator loss가 열로 이동, 총 에너지 잔차 gate |
|Y26| γ<0 또는 passive coefficient 비정상 | validated active medium 없으면 material 거부 |
|Y27| finite source가 방사한 에너지10 | source 감소=field+흡수+경계유출, 오차 범위 |
|Y28| 도체 절단 중 field 존재 | 에너지·전하·과도 상태 유지, λ중복 없음 |
|Y29| 유전체를 다른 E 영역으로 이동 | remap·기계 work·field 에너지 차이가 일치 |
|Y30| 이동 자성체와 도체 | 유도·역작용과 구성 에너지의 cycle gate |
|Y31| PML에서 에너지2흡수 | radiation_out+2; 세계 내부 열+2 금지 |
|Y32| R packet 에너지7흡수 | heat/chemical 합7; W에 같은7자동 지급 금지 |
|Y33| incoherent R ray2개 교차 | 위상 상쇄 없음; 각각의 에너지 유지 |
|Y34| W 여기 전이를 R에도 등록 | 중복 transition owner 검출 |
|Y35| 정확히0인 EM chunk에 이웃 stencil 도달 | 갱신 전에 wake, 조밀 reference와 같은 결과 |
|Y36| 아주 작은 nonzero field | 임의 threshold로 삭제하지 않음 |
|Y37| 소진 가능한 bedrock10에서 침식3 | bedrock7+suspended3; 원소·일 장부 일치 |
|Y38| 밀도2,면적4의 재료8퇴적,기공0 | 높이 증가1; 질량8이 실제 이동 |
|Y39| 질량2를 높이3올림,g=1 | Ug+6; 실제 공급 work≥6 |
|Y40| 정지 호수,수면 일정,기복 있는 바닥 | lake-at-rest 유지, 가짜 유동 허용 오차 이하 |
|Y41| water1,잠열4/unit,증발.25 | liquid−.25,vapor+.25,열−1 |
|Y42| 위 vapor.25 재응결 | 물질 반대 이동,열+1; cycle 추가 work 없음 |
|Y43| donor 물1인데 flux2요구 | positivity 보존 재계산; 물을 임의 생성 안 함 |
|Y44| 유체의 가용 일보다 침식 일이 큼 | 침식 감소/step 재계산, 부족분 free work 없음 |
|Y45| 조성이 다른 인접 퇴적층 | 이력/상태 구별; 동일 색만으로 병합 안 함 |
|Y46| 대기 흡수3,유입복사10 | 통과·반사·저장/재방출 합7+3의 일관된 수지 |
|Y47| atmosphere coarse face→fine surfaces | 면적 가중치 합1; 원소·열 총량 불변 |
|Y48| 바다 재고0,외부 물 경계0 | 임의 sea_level/rain으로 물 생성 없음 |
|Y49| 생체 기체 전환으로 대기 조성 변화 | 해당 원소·반응 에너지 보존, 실제 물성 재평가 |
|Y50| slow env step과 fine step 비교 | 같은 시간 단위, 수렴 gate; 지형 시간 배속 없음 |
|Y51| guest op가 base op100개 사용 | 비용100개분;32op/tick 후보에서 최소4tick |
|Y52| guest recursion으로 nested executor 생성 시도 | 신규 무료 quota 없음; 기존 memory/VM만 사용 |
|Y53| 잘못된 token·무한 loop | 결정론 중단/재개·비용, 전체 host hang 없음 |
|Y54| guest의 위조/stale world handle | base ABI와 동일 접근 실패 |
|Y55| interpreter revision은 동일,data만 다름 | 실제 실행 차이가 data 의미의 증거로 남음 |
|Y56| data만 전달,decoder 없음 | host가 의미/번역기 자동 제공 안 함 |
|Y57| code draft 변경 중 기존 frame 실행 | 기존 frame revision 고정,새 activation에서 채택 |
|Y58| learned code 획득,생식 기록 미복사 | offspring에 자동 문화 주입 없음 |
|Y59| 자연 조상에 손작성 interpreter 없음 | fixture 결과와 R3 자연 출현을 별도 기록 |
|Y60| 같은 seed/input,workers1/2/3 | 동일 numeric backend의 commit hash 일치 |
|Y61| field half-step·guest PC·퇴적 flux 저장 후 재개 | 중단 없는 실행과 hash 일치 |
|Y62| coupling 실패 후 tick rollback | field phase·층·종·PC·RNG·외부유입 모두 복구 |
|Y63| 카메라·UI·분석기·MAX설정 변화 | 코어 hash 불변; 실제 wall-clock 속도만 변화 |
|Y64| R7 save를 CT1로 변환 | 개입 branch 표시; 기존 자연사의 무변경 replay 아님 |

수치 oracle의 초기 목표는 정수 원소·전하 정확 일치, reference 정지/평형 fixture의 normalized residual≤1e−8, coupled witness의 누적 에너지 잔차≤1e−4다. 분모는 각 fixture에서 공개한 비영 에너지 규모를 쓰며 서로 상쇄되는 거대한 저장량으로 오차를 숨기지 않는다. step·grid refinement에서 더 작은 residual만이 아니라 관심 응답의 수렴도 검사한다. 이 숫자는 초기 engineering target이고 실측 결과가 아니다.

복합 witness W-CT1은 원료→조립→특성→분해 회수, W-CT2는 유료 전류→파동→수동 구조→실제 수신 반응, W-CT3은 생체 기체 교환→대기/물 변화→다른 생체 영향, W-CT4는 침식·퇴적→구조/재고/기록의 변화, W-CT5는 raw record→guest interpreter→실제 작업→다른 개체 전달이다. W-CT6은 이 중 세 영역 이상이 이어지는 경로를 사용하되 제작·유지·복사 비용까지 포함한다.

이 witness는 설계자가 만든 R1/R2 검증이다. R3/R4는 최소 여러 seed에서 미개입 자연 실행을 보존하고 조상·모든 개입·검출기 버전·코드/물질 전달을 공개해야 한다. 어떤 경로를 반드시 발견하라는 성공 보상은 없다. no-signal·no-transfer·decoder removal 대조는 별도 분석 branch로 수행한다. 새 형상을 감지한 기록만으로 발명·언어·문명을 확정하지 않는다.

**경계:** Engineering Contract는 기대값·회귀·수렴·증거 단계다. W/P는 검증 대상이다. E는 자연 실행에서 별도로 확인한다. U는 시험 통과와 자연 발견을 구별한다.

## 107 R8 개발 순서·실패 모드·동결 상태

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 구현 의존 순서

첫 단계는 Core의 단독 소유와 원소·전하·에너지 oracle, microstructure의 재고 예약, 고정 재료의 작은 Maxwell reference, 보존형 surface flux다. 그 뒤 조립/반응·field/material work·지층/대기 교환을 각각 검증한다. guest interpreter 회귀 fixture는 기존 VM 위에서 병행 가능한 독립 검증이며 실제 조상에는 넣지 않는다.

두 번째 단계는 같은 작은 세계에 네 축을 합치고 W-CT1..W-CT6을 통과시키는 것이다. 특히 moving material+Maxwell, environment+support geometry, hydrostatic pressure work는 기능 목록에 체크하는 수준으로 끝내지 않는다. 고정 재료 파동, 정적인 지형의 수문 등 하위 단계만 통과했다면 그대로 표시한다.

세 번째 단계는 v2-Continuum의 UI·headless·저장·replay·계통/기술/표현 observer를 결합하고, 미개입 장기 실행과 실제 TPS를 보고하는 것이다. R7의 전체 큰 규모 목표를 다시 검토한다. 시각화가 좋아졌다는 이유로 물리/생태 검증을 대신하지 않는다.3D·신경·진화하는 mutation operator는 이 단계에 포함하지 않는다.

### 추가 실패 모드와 대응

| 실패 | 예방·판정 | trade-off |
| --- | --- | --- |
| 원료보다 조립 재고가 많아짐 | material slice 독점·transaction·Y01/02 | owner 추적·예약 비용 |
| 자기조립이 실제로 전혀 발견되지 않음 | 접촉·중간 이득·장벽/수송 witness; 열 운동 제한 명시 | 미동결 물성 실험 필요 |
| 고분자 hash만 늘고 기능 동일 | geometry와 실제 응답·재사용 증거 확인 | observer와 물성 계산 비용 |
| 촉매/파동 결합이 무한동력 | 같은 energy owner·passivity·closed-cycle gate | 결합 solver 비용·실패 중단 |
| R7 MQS와 Maxwell을 중복 계산 | CT1에서 field 단독 소유,legacy 분리 | 기존 자기 solver 재작업 |
| 파동이 사실상 거리에 따른 신호값 | phase·회절·cavity·재료 반응 witness | 높은 시공간 해상도 비용 |
| 작은 전파를 최적화로 삭제 | 정확한0조건만 생략,카메라 독립 | 큰 지도에서 생략 효과 제한 |
| 지형 변화가 물질/일을 생성 | 유한 column·보존형 flux·지지 work | terrain solve 복잡도 |
| 기후가 문명 유지용 조절기로 변질 | population/기술을 환경 입력에서 제외 | 전멸·정체·불리한 역사 허용 |
| 언어 승격이 host 능력 unlock | guest data만 해석,기존 VM 비용·권한 | interpreter가 느릴 수 있음 |
| decoder 전달 없이 문화를 자동 공유 | 실제 매체·호환 실행·실패 기록 | 기술 전파가 더 어려워짐 |
| 표현력 확대로 탐색이 오히려 멎음 | 유용한 짧은 경로의 가능성부터 별도 검증 | 가능성과 발견 사이 간격 유지 |
| 모든 축을 켜면 성능 붕괴 | CT1-Reference부터 실측,정확성 우선 최적화 | 초기 world/개체 규모 축소 |

### 현재 동결된 것과 남은 구현 입력

이번 개정에서 확정한 것은 네 축의 범위,3D/신경/변이 연산자 제외, 고수준 primitive 금지, 화학/미세구조의 단독 재고, 전기·자기의 단독 field 소유,2D 파동·비간섭 복사의 한계, 실제 환경 flux, 고정 VM 위의 guest 표현, 소유권·phase·저장 대상·검증 계약이다.

| 항목 | 아직 동결해야 할 구체 입력·증거 |
| --- | --- |
| MicrostructureData-CT1 | patch/solvent/가교 계수·허용 범위·구성 자유 에너지·coarse 응답 수렴 |
| ChemistryData-CT1 | R7의 valence/transition-state 실제 값과 새 표면 반응·광 전이 채널 |
| EmData-CT1 | 이산 에너지·자화 법칙·passive coefficients·PML·charge deposition reference |
| MovingMaterialCoupling | 보존형 remap·force/stress·전도체 절단·반복 solve의 실행 증거 |
| RadiationBandMap-CT1 | W/R의 비중첩 유효 대역·단위·반응 owner·평형 자료 |
| TerrainData-CT1 | 입자/침식/퇴적 계수·well-balanced flux·지지면 work와 수치 reference |
| AtmosphereData-CT1 | EOS·hydrostatic closure·pressure work·복사·응결의 수치 자료 |
| ABI/VM·CIVCT001 | sensor/service wire·code 채택 boundary·정확한 field encoding·golden vectors |
| Benchmark·자연 실행 | CPU/RAM/TPS·오차·retry·다중 seed와 미개입 R3/R4 증거 |

따라서 R8은 **네 축의 구현 방향과 계약을 통합한 연구·기술 설계 개정**이다. 실행 가능한 프로그램, 모든 계수와 solver가 확정된 제작 도면, 무한 기술 발전의 증명은 아니다. 해결하기 어려운 수치 결합을 “나중에 최적화”라는 말로 완료 처리하지 않는다. 위 표는 구현 전후에 반드시 채워야 할 작업 목록이다.

자유도를 높이는 방향 자체는 프로젝트 목적과 맞는다. 다만 실제로 더 나은 세계가 되었는지는 가능한 현상의 수, 기능적 중간 경로, 자연 발견·전달·소실, 장기 변화, 같은 계산 예산에서 관측 가능한 역사의 길이를 함께 보고 판단한다. 네 축을 모두 추가했는데 세계가 느리기만 하고 유용한 조합을 찾지 못한다면 모델을 성공으로 평가하지 않는다.

**경계:** Engineering Contract는 개발 의존·미동결 목록·완료 판정이다. W/P는 확정된 저수준 표현에 한정한다. E는 실험으로 확인할 역사다. U는 설계·구현·가능성·실제 발견의 상태를 정직하게 표시한다.


## 108 R9 범위와 교체표: 여섯 제약의 개선과 비용 통제

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

이번 개정은 원소 수, 화학 graph 크기, 계산 표현, 신경망·생애학습, 변이 연산자, 코드·기억 용량을 개선한다. **사용자의 최신 지시에 따라 신경망과 생애학습을 모두 포함한다.** 도중의 제외 요청은 이 최종 방향으로 교체한다. 작은 선택적 신경 모듈과 유료 학습을 기본 VM에 결합하며 모든 개체에 거대한 신경망을 강제하지 않는다. 생물 형태·발생·대사·생태와 R8의 화학·파동·환경 법칙은 유지하며 자유로운3D 공간은 추가하지 않는다.

목표 ruleset은 civ-continuum-2-draft, VM/world ABI는 CT2, 제품 목표는 v2-Continuum이다. R9는 CT1의 실행 중 옵션이 아니라 별도 규칙 revision이다. 임의의 크기 제한을 없애는 것과 유한 컴퓨터에서 무한 세계를 실행하는 것은 다르다. 본 개정은 첫 번째를 목표로 하며 두 번째를 약속하지 않는다.

### 무엇을 바꾸고 무엇을 남기는가

| 지적된 제약 | R9 결정 | 비용·한계와 교체 위치 |
| --- | --- | --- |
| 8개 원소 고정 | 실행 시작 시 고정하는 확장형 ElementRegistry; 기본 후보16종 | 물성 검증은 별도 필요;109절 |
| 유한 화학 graph | 64 heavy atom·4,096 monomer라는 모델 상한 폐지; page graph와 반복 구조 | 물질·공간·주소·host RAM은 여전히 유한;110절 |
| 고정 base VM | 최소 kernel 유지, 넓은 주소·재사용 가능한 guest 표현·세대 간 interpreter 진화 | 물리 법칙 자체를 프로그램으로 바꾸지는 않음;111절 |
| 신경망·생애학습 없음 | 선택적 희소 recurrent module + 유전되는 학습 절차 + 생애 내 가변 상태 | 계산·기억·학습·전달 비용과 상속 경계 필요;112절 |
| mutation operator 진화 없음 | 생식 과정에서 유료로 실행되는 유전 가능한 일반 VM 프로그램 | 의도된 변이와 물리 복사 잡음을 분리;113절 |
| 유한 메모리·코드 | 실제 backing에 비례해 증가하는 page/arena, 개체별 임의 길이 상한 폐지 | 무한 자원·무료 복사·자동 pruning 없음;114절 |

현재 계약의 우선순위는108..120절의 명시적 교체,95..107절의 유지 부분,81..94절의 유지 부분,68..80절의 유지 기반 순이다.06..67절은 Grid-5 회귀용이다. 과거 문서의 신경망 제외·8원소 배열·분자 길이 상한·v3 변이 연산자 유보를 CT2의 현재 요구사항으로 읽지 않는다. 학습·성체 코드 채택은112절의 비용과 revision 경계를 따른다.

기존261개 기대 사례는 해당 legacy profile의 회귀 계약으로 보존한다. 새 Z01..Z64는 CT2 계약이다. 총325개가 모두 같은 ruleset에서 동시에 실행된다는 뜻은 아니다. 모든 사례는 **아직 구현되지 않은 시뮬레이터에 대한 기대값**이며 통과한 시험 결과가 아니다.

### 네 범주의 공통 경계

W는 보존·접촉·수송·반응·실행 비용·실제 복사와 고정된 최소 kernel이다. P는 국소 감각·힘·물질 조작·기록·정수 연산이다. E는 그 조합으로 가능한 구조·절차·유전 방식·표현 체계다. U는 실제 계보·기능·사용 증거를 바탕으로 붙이는 이름이다. 원소 수·분자 크기·언어 깊이를 늘렸다고 문명 발전으로 점수를 지급하지 않는다.

**경계:** Engineering Contract는 개정 범위와 우선순위다. W/P는 아래 계약만 바뀐다. E의 자연 발견과 누적은 관측 대상이다. U는 구현 완료와 가능성·자연 발생을 구별한다.

## 109 원소 registry와 확장 가능한 화학 법칙

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 세 가지 방법의 비교

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 8종을16종 enum으로 교체 | 수정이 쉽고 비용 예측 가능 | 같은 고정 한계를 다른 숫자로 옮김; 낮은 복잡도 | 기본 데이터만16종으로 둠 |
| 검증된 원소·상태 데이터 registry | 새 조합을 열면서 보존·일관성 검사 가능 | sparse 조성·물성 자료 검증 필요; 중간 이상 | **채택** |
| 실행 중 임의 원소·법칙 생성 | 겉으로 표현 공간을 계속 확장 | 에너지·원자가·선택압이 바뀜; 높은 검증 비용 | 현재 제외 |

ElementKey는 registry 안의 u32 색인이다. 원소의 수는 세계 생성 시 결정되고 모든 원소는 처음부터 같은 법칙에 포함된다. agent가 기술을 발견했다고 새 원소가 unlock되지 않는다. 초기 매장량이0인 원소는 외부 유입이 명시되지 않는 한 얻을 수 없다. 핵반응·원소 변환은 이번 개정에 없다.

### 기본16종과 데이터의 의미

후보 기본 순서는 기존 C,H,O,N,P,S,Fe,Si 뒤에 Na,Mg,Al,Cl,K,Ca,Cu,Zn을 붙인다. 계산용 mass_unit은 각각12,1,16,14,31,32,56,28,23,24,27,35,39,40,64,65다. 이는 정확한 정수 보존을 위한 인공 질량값이며 자연 동위원소 평균이나 실제 반응 예측 자료가 아니다. 이름과 기호는 [IUPAC의 원소 표기](https://iupac.org/what-we-do/periodic-table-of-elements/)를 참고한다. 이 출처가 본 모델의 물성 계수를 보증하지는 않는다.

새 원소가 열어줄 가능성은 실제 전하 상태·결합·용해·계면·전도 모델에서 나온다. 예를 들어 Cu라는 이름을 가진 것만으로 회로 능력을 주지 않는다. 실제 재료의 연결과 constitutive law가 전류·열·힘을 만들어야 한다. 생물이 원소를 선택적으로 흡수하는 현상도 기존 막·수송·반응의 결과다.

ElementDef는 stable_symbol, mass_unit:u32, nuclear_label:u16, allowed_state_span, interaction_class, parameter_revision을 가진다. 허용 state에는 formal_charge, valence constraints, local coordination, 에너지 항의 참조가 있다. nucleus label은 보존·식별용이며 원자핵 시뮬레이션을 뜻하지 않는다. 초기 원소16종의 모든 허용 상태와 계수는 ChemistryData-CT2로 별도 동결해야 한다.

### 조성 저장과 보존

종별 조성은 정렬된 ElementCount(ElementKey:u32,count:u64)의 sparse span이다. 처음4개 항은 작은 inline 저장, 초과는 arena에 둔다. 소분자의 count는 내부적으로 좁은 정수로 압축할 수 있지만 canonical serialization은 같은 값을 보존한다. 전체 원소 ledger는 registry 길이의 checked u128 배열이다. 원소 수에 비례하는 배열을 모든 tile·agent·결합에 복제하지 않는다.

반응·결합·분할·복제·침식은 모두 입력과 출력의 원소 합과 전하가 같아야 commit된다. 계수와 양의 곱은 checked u128로 승격한다. ledger overflow를 wrap·saturate해 보존 성공으로 처리하지 않는다. 표현 범위를 넘으면 마지막 완료 tick을 보존하고 RunHalt::NumericRange로 중단한다.

질량 M은 각 원소 수와 mass_unit의 내적이다. 같은 material slice를 graph, stock, 조직, 기록 backing에서 동시에 소유할 수 없다. 각 표현의 참조는 해당 원소 재고를 설명할 뿐 새로운 재고를 만들지 않는다. 전자·용액 전하와 EM 전류의 기존 소유권도 유지한다.

### 새 원소를 넣을 때 필요한 검증

반응 후보 생성은 기존 국소 rewrite 문법을 사용한다. species 전체 목록을 사람이 작성하지 않는다. 새 국소 상태의 에너지·장벽·수송·passive EM 파라미터가 공통 함수나 검증된 자료로 정해져야 한다. 알 수 없는 조합을 임의 random 물성으로 대체하지 않는다. 의도적으로 금지한 상태와 데이터가 누락된 상태를 구별하고, 후자는 실행 시작 전에 오류로 보고한다.

정·역반응은 같은 상태 함수와 장벽 모델에서 파생한다. 촉매가 장벽만 바꾸는 경우 평형의 기본 자유 에너지 차이는 유지한다. 닫힌 반응 cycle이 물질이나 에너지를 생산하면 데이터팩을 승인하지 않는다. registry 확장 시 초기 자원 배치도 새 manifest에 기록하며 기존 실행의 계속된 자연사로 위장하지 않는다.

성능 비용은 주로 실제 존재하는 국소 화학 다양성에 비례한다. registry 자체에 원소가 많다는 이유로 모든 tile에서 모든 원소 쌍을 검사하지 않는다. 단, 가능한 반응을 CPU 예산 때문에 임의로 제외하는 것은 허용하지 않는다.

**경계:** W는 원소·상태·반응의 고정 법칙과 보존이다. P는 기존 물질 이동·합성·가열·감각이다. E는 새로운 조합·자원 이용·재료다. U는 조성·반응 경로·재료 계보를 표시한다.

## 110 화학 graph: 작은 분자와 큰 연결 구조를 같은 보존계에서 다룬다

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 표현 선택

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 모든 분자를 작은 정규 문자열로 제한 | 비교·캐시가 쉽고 빠름 | 큰 고분자·분기·망상 구조 차단 | 모델 상한으로 사용하지 않음 |
| 모든 원자를 매번 전역 canonicalize | 정확한 동일성 비교 | 큰 대칭 graph·분할에서 비싼 전역 처리 | 작은 분자에만 사용 |
| 작은 종 interning + 큰 instance graph + 정확한 반복 압축 | 긴 사슬·가교·국소 결함 허용 | 표현 전환·열역학·소유권 계약이 복잡 | **채택** |

64 heavy atom과4,096 monomer는 화학적 생존 조건에서 제거한다. 대신 graph는 유한한 실제 원료, 공간, 결합 상태와 실행 자원으로 제한된다. 64bit 주소도 유한하다. “무한 분자를 구현했다”거나 “현실의 모든 화학을 표현한다”고 표시하지 않는다.

### 물리 graph와 구현 저장의 분리

AtomKey는 저장 chunk와 slot·generation을 가리키는 opaque key다. 결합은 두 endpoint와 국소 상태를 가진다. graph의 주소는 공간 좌표가 아니다. 연결된 거대분자라고 해서 한쪽 끝의 반응물이 다른 쪽으로 즉시 이동하지 않는다. 기존 MicroNode·material slice·공간 접촉과 함께 실제 수송·기하를 유지한다.

GraphPage의 기본 저장 단위는256개 atom slot이고 bond는 별도의 SoA arena에 둔다. 이 숫자는 host 할당 단위이며 분자 길이·원자가·에너지 규칙이 아니다. 삭제된 slot을 재사용할 때 generation을 증가시키며 다른 원자의 오래된 참조가 새 원자를 가리키지 못하게 한다. 주소·generation overflow는 안전 정지다.

정규화 비용이 작은 분자는 SpeciesDef를 공유해 StockLot의 양으로 다룬다. 큰 분자는 각 실제 instance의 graph와 공간 상태를 가진다. 분기·가교·분할·재결합은 graph edit transaction이다. 반응 중심의 반경2 환경만으로 후보와 국소 에너지 항을 찾되, 장거리 전기·탄성·용매 항은 해당 solver가 계속 계산한다. 반경2라는 이유로 모든 물리 효과가 국소화되었다고 가정하지 않는다.

### 동일성과 열역학의 함정

큰 graph의 InstanceKey·hash·작성 순서는 화학적 정체성이 아니다. 같은 graph를 다른 순서로 만들었다고 다른 물질로 세거나 혼합 entropy 보너스를 주지 않는다. 국소 hash는 후보 검색과 cache에만 쓰며 충돌 가능성을 남긴다. 정확한 동일성 검사가 필요한 작은 종은 graph 비교로 확인한다.

큰 instance의 힘·반응·에너지는 실제 원소·결합·배치에서 계산한다. 전역 canonical species 이름이 아직 없어도 규칙은 동일하게 실행된다. observer의 approximate family label이나 canonicalization 완료 시점은 반응률에 영향을 주지 않는다. 거대 대칭 graph를 해석하느라 host가 느려져도 그 물질의 세계 내부 반응이 임의로 느려지지 않는다.

StockLot 표현과 explicit instance 표현 사이 전환은 개수·연결·배치·자유 에너지·entropy의 대응을 입증해야 한다. CT2 초기 reference에서는 한 화학 domain의 표현 방식을 생성 시 고정하고 임의 크기 threshold를 넘었다는 이유만으로 자동 전환하지 않는다. 소분자 bulk domain과 연결 고분자 instance domain 사이 반응은 명시적 생성/소모 transaction으로 연결한다. 표준 상태와 화학 퍼텐셜의 대응은 별도 동결 gate다.

### 반복 압축과 국소 변이

완전히 반복되는 사슬은 반복 block DAG, 길이, endpoint mapping으로 lossless 저장할 수 있다. host가100만 반복을 짧은 descriptor로 압축해도 실제 원자 수·질량·접촉·길이·확산·복사 비용은100만 반복에 해당한다. 물리 시간을 건너뛰는 자동 반복 실행도 없다.

국소 결함·가교·절단은 해당 구간만 copy-on-write로 펼친다. 압축을 풀어야 할 host RAM이 부족하면 해당 생물만 죽이거나 결합을 거부하지 않는다. tick을 rollback하고 중단한다. 최적화된 반응 후보 집계는 같은 반응 경로·extent·보존을 산출하는 경우에만 reference를 대체한다.

연결 성분 분할, 장거리 응력, graph 대칭 판정은 최악의 경우 graph 전체를 순회한다. “국소 graph니까 모든 연산 O(1)”이라는 성능 약속을 하지 않는다. 이런 작업은 worker에 나눠 host 시간을 줄일 수 있지만 다음 authoritative phase는 완료를 기다린다. wall-clock 제한에 걸렸다는 이유로 일부 성분만 갱신하지 않는다.

**경계:** W는 실제 화학 결합·기하·열역학이다. P는 기존 국소 반응·힘·물질 조작이다. E는 긴 고분자·가교망·재조합 재료다. U는 graph 크기와 기능 증거를 별도로 표시한다.

## 111 계산 표현의 진화와 넓은 주소 VM

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### fixed VM 문제에 대한 결정

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 실행 중 native opcode 추가 | 빠른 새 기능 | 새 세계 능력·권한·비용 우회; 검증 매우 어려움 | 금지 |
| 기본 VM 교체를 유전 | 여러 ISA를 직접 실행 | 각 ISA의 의미·비용 oracle 필요; 구현 편향 증가 | 현재 제외 |
| 고정 kernel 위에 유전되는 interpreter·표현 | 언어·절차·계층을 코드와 함께 재조합 | 해석 비용과 탐색 난도 증가 | **채택** |

최소 산술·분기·읽기·쓰기·시간·권한은 고정한다. 이 규칙까지 바꾸는 것은 진화가 아니라 다른 우주를 실행하는 일이다. 개선 대상은 특정 언어·작은 주소·짧은 stack에 개체를 가두는 제약이다. 새로운 표현도 같은 저수준 물리 효과만 요청할 수 있다.

guest decoder, 데이터 규약, 조합된 절차는 일반 유전 프로그램·기록이다. host가 모듈의 의미를 분석해 자동으로 더 좋은 명령을 만들어 주지 않는다. compiler를 손으로 작성한 fixture는 표현 가능성 R1 증거이며 초기 조상의 기능으로 숨겨 넣지 않는다. 세대 간 복사·재조합과 생애 중 유료 draft 채택을 지원하되 둘의 provenance를 구별한다. code adoption은112절의 경계를 따른다.

### CT2 VM의 구체 변경

명령은 기존32bit encoding과16개의 u16 register를 유지한다. PC, code length, memory word address, stack cursor는 u64다. 작은 정수 산술의 overflow 의미는 기존 modulo65,536이다. 주소 계산은 checked 연산이며 overflow를 다른 주소로 wrap하지 않는다. 명령 word는32bit, 일반 memory word는16bit로 구별한다.

| opcode | 의미 | 경계·비용 |
| --- | --- | --- |
| 0x40 LOAD64 | ra..ra+3의 little-endian64bit memory 주소 + signed imm에서16bit를 읽어 rb에 기록 | a=0/4/8/12; 지역 backing만;2 CU |
| 0x41 STORE64 | 같은 주소에 rb의16bit 값을 기록 | base code는 쓰기 불가;2 CU |
| 0x42 JUMP64 | ra..ra+3의 code word 주소 + signed imm으로 이동 | 현재 executable record 범위만;1 CU |
| 0x43 CALL64 | return PC를 paid stack에 저장하고 같은 wide code 주소로 호출 | frame 용량 필요;2 CU |
| 0x44 NCYCLE | 자기 물리 backing의 희소 recurrent graph를 한 epoch 진행 | 호출1 CU +112절의 edge/node/input/output 비용;중간 재개 가능 |

새로운 세계 행동 opcode는0개다. 앞의 네 명령은 넓은 내부 주소 계산, NCYCLE은 명시적으로 비용을 내는 저수준 신경 계산이다. READ·WRITE·COPY·DRIVE 등 물리 service와 sense4/effect1의 controller당 국소 bandwidth는 유지한다. RANDOM은 기존 명령을 사용하며113절의 재현 가능한 stream을 공유한다. NCYCLE이 행동·목표·보상을 해석하지 않는다.

주소 source register들은 instruction 시작의 값으로 읽는다. LOAD의 결과 register가 주소 register와 겹쳐도 주소는 먼저 완성한다. LOAD64 실패는 rb=0, BadAddress, PC+1이며 STORE64 실패는 무쓰기, BadAddress, PC+1이다. 잘못된 a 값은 InvalidOperand, branch/call 범위 오류는 BadAddress다. CALL64는 return frame 저장을 모두 할 수 있을 때만 stack과 PC를 바꾼다. 실패 시 StackCapacity와 PC+1, stack 불변이다. 실행 비용은 실패해도 지불한다.

short LOAD/STORE는 넓어진 memory의 낮은 주소를 읽는 단축형이다. short branch는 checked PC-relative 주소이고 code record 범위를 벗어나면 PC+1과 fault다. CALL tag는 단축 lookup이며 전체 module 수의 상한이 아니다. 긴 호출은 CALL64로 표현한다. RET는16byte frame을 복원하며 빈 stack에서는 entry로 돌아가 yield한다. frame은 return PC와 executable revision으로 구성한다. 실행 중 frame의 revision은 고정하며 새로운 code 채택 후에도 과거 frame의 주소를 새 code에 적용하지 않는다.

### 논리 비용과 물리 compute capacity

CU는 모델이 청구하는 논리 연산 단위이며 host CPU cycle이 아니다. 보통 산술·분기·RANDOM·NOP은1 CU, local load/store와 CALL/RET은2 CU다. 외부 service의 기존 추가 물리 비용은 별도다. DEFAULT 에너지는1 CU당4q로 두며 숫자는 인공 모델 단위다. 메모리 write의 backing·마모·유지는 이 실행 비용에 묻어 무료로 만들지 않는다.

starter controller의 처리량은32 CU/tick이다. 더 큰 조직이 처리량을 늘리려면 실제 compute backing을 더 가져야 한다. CT2의 인공 compute scaffold 한 단위가32 CU/tick을 제공하며 동일한 물질·부피·대사·열 비용을 반복해서 부담한다. scaffold의 정확한 재료 계수는 Data-CT2 동결 대상이다. 이것은 원자에서 CPU가 저절로 발생하는 모델이 아니라 기존 생명 controller의 명시적인 coarse law다.

tick 시작의 유효 backing으로 capacity를 확정하고 해당 tick에 새로 만든 backing은 다음 tick부터 쓴다. 행동 프로그램·신경 계산·학습 절차·발생 절차·변이 프로그램은 같은 controller의 CU pool을 공유한다. pool을 복제하거나 guest nesting마다 quota를 추가하지 않는다. 사용하지 않은 CU는 다음 tick에 누적하지 않는다. 정수 용량과 남은 에너지가 모두 충분할 때만 다음 연산을 시작한다. PC를 진행하지 않은 quota yield와 실제 fault를 구별한다.

guest token 하나가 base VM operation100개를 요구하면 각각의 CU·에너지·tick을 모두 부담한다. 한 번의 CALL로 비싼 절차를 무료 처리하지 않는다. compute backing 증가만으로 sense 범위·손의 수·외부 action 횟수가 늘지 않는다. 새로운 sensor/actuator·controller를 만들면 실제 몸과 전달 비용을 각각 부담한다.

### host 최적화의 허용 범위

predecode·동일 PC batch·superinstruction은 host 실행 시간을 줄일 수 있다. 그러나 논리 instruction prefix, I/O 시점, PC, register, fault, CU, 에너지, yield와 tick 결과는 reference interpreter와 같아야 한다. 합친 block 중간에서 연료가 끝나면 정확히 그 instruction 앞에서 멈춘다. 전체 block이 성공한 것처럼 처리하고 다음 tick 비용을 차감하면 안 된다.

host cache key는 code bytes hash, CT2 semantic hash, CU table, address ABI, backend revision을 포함한다. hash 일치만으로 bytes 동일성을 검증 없이 가정하지 않는다. 캐시 유무는 저장할 미래 상태가 아니어야 한다. cache warmness가 세계 시간을 바꾸면 최적화 승인을 취소한다.

**경계:** W는 고정 kernel·처리량·비용·권한이다. P는 일반 계산·호출·신경 수치 계산·국소 I/O다. E는 변화하는 interpreter·자료구조·절차 체계다. U는 코드 길이보다 실행·복사·재사용 증거를 보여준다.

## 112 작은 신경 모듈과 생애학습

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 실행 모형 비교와 선택

| 방식 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| VM만 사용 | 매우 작고 전달·재현 쉬움 | 연속·시계열 적응 절차를 직접 구성해야 함 | 유효한 최소 controller로 유지 |
| 모든 개체에 큰 dense RNN | batch 처리 편리 | 사용하지 않는 연결까지 비용; 큰 고정 표현 편향 | 제외 |
| spiking neural graph | 사건 기반 처리 가능 | 시간 적분·가소성·결정론 계약이 더 복잡 | 후속 비교 연구 |
| VM + 선택적 희소 recurrent graph | 프로그램·시계열·국소 학습 결합, topology 진화 가능 | mutable weight·진행 상태·학습 비용 증가 | **채택** |

NeuralModule은 controller 내부의 수치 표현이다. 생물 몸에 완성된 뇌 기관을 제공하는 enum이 아니다. node·edge·weight·state를 가지며 실제 compute와 memory backing을 사용한다. default 개체에 neural module을 강제로 달지 않는다. 유전 template·발생·복사를 통해 크기가 달라질 수 있고, 필요 없는 계통은0개를 유지할 수 있다. 노드32개라는 영구 상한 대신 실제 비용을 적용한다.

### 입력·출력과 의미의 경계

VM이 정당하게 읽은 감각·내부 상태를 유료 local input span에 쓴다. 신경 모듈은 그 span만 읽는다. 주변 자원 좌표·직업·문명 ID·전역 novelty·상대 genome 같은 새로운 정보는 얻지 못한다. output은 local scalar다. output0=Walk, output1=Trade처럼 완성 행동에 연결하지 않는다. 기존 VM이 값을 읽고 원래의 force·transfer·signal service에 사용할 수 있다.

서로 떨어진 Cell의 신경망을 무료로 하나로 합치지 않는다. 각 module은 자신의 Cell backing 안에서 계산하고 다른 Cell과의 정보 전달은 실제 전기·화학·빛·기록 경로를 사용한다. 전체 몸의 빠른 공유 신경망을 만들려면 실제 전달 구조와 지연을 모델링해야 한다. CT2 reference는 이런 무료 공유를 지원하지 않는다.

### 결정론적 최소 recurrent kernel

reference 상태와 weight는 signed16 고정소수점이며 scale은16,384다. state의 출력 범위는−1..1, weight 범위는−2..약2다. 각 non-input node의 accumulator는 signed64로 두고 bias:i32를 동일한 product scale로 더한다. 모든 edge는 이전 완료 epoch의 source state 또는 이번에 고정한 input을 읽는다. node 번호 순서에 따른 즉시 전달을 허용하지 않는다.

node 갱신은 sum=bias+Σ(weight×source), next=clamp(round_even(sum/16,384),−16,384,16,384)다. saturating linear activation을 쓰며 platform별 tanh·exp 차이를 만들지 않는다. bias의 표현 단위는 state×weight와 같다. accumulator overflow는 정의된 endpoint에서 saturate하거나 wrap하지 않고 NumericRange fault와 epoch 취소로 처리한다. 실제 실행 가능한 graph의 degree·수치 범위 검사는 시작 시 확인한다.

edge traversal은 destination node, stable EdgeUID 순서다. NCYCLE은 a..a+3의 local descriptor 주소를 읽고 b·imm은0이어야 한다. descriptor에는 input/output span, graph revision, state revision, weight revision이 있다. host world pointer를 넣을 수 없다. descriptor가 잘못되면1 CU를 쓰고 InvalidOperand 또는 BadAddress, PC+1로 끝난다.

정상 epoch의 논리 비용은 C=1+4I+8M+4N+2O CU다. I는 고정할 input 수, M은 edge 수, N은 갱신할 non-input node 수, O는 복사할 output 수다. 호출1, input staging 각4, edge 처리 각8, node finalize 각4, output copy 각2 CU를 순서대로 청구한다. CU마다111절의 에너지를 내며 buffer도 paid memory다. 이는 공개된 인공 계산 비용 모델이며 실제 뉴런의 생리학 단위가 아니다.

quota가 부족하면 다음 atomic 작업 전에 멈추고 phase·cursor·accumulator·staging 값을 보존한다. 다음 tick에 해당 작업부터 재개한다. NCYCLE이 끝나기 전까지 이 controller의 VM은 해당 instruction에서 대기한다. 새 감각으로 중간 input을 바꾸지 않는다. output과 node state는 모두 계산한 후 한 번에 publish하고, 일부 계산한 output을 미리 행동에 쓰지 않는다. 세계는 기다리지 않으므로 큰 망은 낡은 input으로 늦게 반응할 수 있다.

물리 손상·사망이 진행 중 graph/backing을 무효화하면 epoch를 취소한다. 이미 쓴 계산 에너지를 환불하지 않고 부분 staging을 유효 state로 채택하지 않는다. host 수치 retry는 취소와 달리 journal 전체를 되돌리므로 에너지를 중복 청구하지 않는다. 변경할 weight와 topology는 진행 중 epoch의 revision을 수정하지 않는다.

### 생애학습의 최소 경로

학습은 신경망 가중치 변화뿐 아니라 VM의 유료 table·procedure·memory 수정도 포함한다. 유전된 LearningProgram이 실제로 받은 감각, 자기 상태, 수행 receipt, 이전 신경 state를 읽고 PendingUpdate를 작성한다. 어떤 경험을 저장하고 언제 바꿀지 역시 프로그램이 결정한다. host가 무료 경험 dataset이나 성공 label을 축적해주지 않는다.

초기 reference가 지원하는 것은 weight·bias·trace read/write와 새 code draft의 유료 채택 경로다. 자동 backpropagation, 전역 optimizer, 인간이 작성한 생존 reward 함수, offline 최고 개체 학습은 기본 실행에 없다. 프로그램이 그러한 수학을 스스로 표현하면 일반 연산·기억 비용을 낸다. imitation도 타인의 행동을 전역 archive에서 정답으로 받는 방식이 아니라 실제 감각·신호·기록으로만 가능하다.

LearningProgram은 controller가 일반 CALL로 호출한다. 모든 tick에 host가 무료 학습 단계를 끼워 넣지 않는다. PendingUpdate에는 target local span, expected revision, index, 실제 기록할 raw value가 들어간다. 같은 위치에 대한 쓰기는 실제 program/write ordinal 순서이며 마지막 유효 쓰기가 남는다. revision이 다르면 StaleRevision과 무쓰기를 반환하고 시도 비용은 낸다. 새 weight에 대한 성능 비교·자동 채택·실패 시 이전 우수 weight 복원은 없다.

topology 변화도 원료·기억·기록·발생 비용을 내고 새 graph revision을 만드는 방식이다. 기존 epoch를 중간에 다른 topology로 해석하지 않는다. 연결을 추가했다고 공짜 전도·감각 경로가 생기는 것이 아니며, Cell 사이 연결은 실제 매체가 필요하다. 유전·학습 과정이 node/edge를 만들 수 있는 표현과 그 변화가 적응에 유익하다는 판단을 분리한다.

검증 fixture로는 pre/post state와 내부 modulator를 곱해 weight delta를 만드는 짧은 국소 규칙을 사용할 수 있다. modulator는 개체가 실제로 읽을 수 있는 값으로 계산하며 host fitness score가 아니다. 이 fixture를 조상에 넣으면 seeded-learning 실험으로 표시한다. 자연 발생 R3 실험에서는 그 학습기를 몰래 주입하지 않는다.

연결 weight와 학습 규칙을 함께 다루는 연구 선례로 Miconi 등의 plastic network 연구를 참고한다. 그 연구의 외부 훈련 방식과 성능을 본 시뮬레이터가 재현한다는 뜻은 아니다. CT2의 생태적 선택·유료 학습은 별도 설계이며 실행 검증이 필요하다. [Miconi, Stanley, Clune 2018, 원 논문](https://proceedings.mlr.press/v80/miconi18a.html)

### 유전·학습·문화 상태의 분리

GermlineState에는 초기 graph·초기 weight·발생·학습 절차가 있다. SomaticState에는 현재 weight·trace·working memory·경험 record가 있다. CultureRecord에는 실제 외부 매체에 쓰인 module·data·출처가 있다. 학습한 weight와 절차는 자동으로 자식에게 유전되지 않는다. 생식 기록에 실제 복사된 경우에만113절의 유전 경로에 들어간다.

LearningProgram 자체도 raw program이다. 새로운 revision은 자기 draft를 유료로 작성·검증한 뒤 명시적 adoption boundary에서 채택할 수 있다. verifier는 구문·범위·권한만 보며 학습 성능을 평가하지 않는다. 기존 실행 frame·NeuralEpoch·생식 operator는 시작 때의 revision을 끝까지 pin한다. 새 revision은 새 activation에만 적용된다. pinned backing을 유지할 자원이 없으면 미완료 activation을 취소하고 비용을 환불하지 않는다.

문화 전달은 record copy, 실제 weight/module 사용, 기능 재현을 각각 기록한다. module을 받았다는 사실만으로 기술 획득을 확정하지 않는다. 개체가 죽으면 내적 학습 상태는 해당 매체의 물리 상태에 남거나 소실되며 무료 cloud memory로 남지 않는다. 외부 기록·살아 있는 후계자·생식 template이 없으면 배운 절차는 사라질 수 있다.

### 학습의 판정과 한계

같은 유전 정보·현재 자극에서 경험 이력이 다른 개체가 다른 반응을 보이는지 비교한다. 관찰용 분기에서 학습 state를 초기화했을 때 차이가 없어지는지도 본다. 단순 age 변화·자원 차이·숨은 genome 변이와 구별한다. 실험 분기는 원본 자연사에 반영하지 않는다. 더 좋은 신경망 점수로 번식권을 지급하지 않는다.

학습은 적응을 도울 수 있지만 계산비·불안정성·잘못된 경험·망각·기생 신호·과도한 plasticity 때문에 불리할 수도 있다. 학습이 없는 짧은 VM 계통이 살아남는 결과도 허용한다. 신경망을 넣었다는 사실만으로 지능·언어·문화 누적을 구현 완료했다고 표시하지 않는다.

**경계:** W는 수치 kernel·기억·계산 비용·revision·실제 전달이다. P는 raw input/output·수치 갱신·일반 기록 조작이다. E는 학습 방식·표현·기억·통신·모방·절차 개선이다. U는 학습·유전·문화 변화의 실제 증거를 구별한다.

## 113 유전되는 변이 연산자와 생식 transaction

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 방법 비교

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 고정 확률표만 변이 | 빠르고 해석 쉬움 | 개발자가 정한 변이 메뉴 안에서만 변화 | 회귀·비교군 |
| 변이 확률·구간 길이 유전 | 연산자 전체보다 저렴 | 절차 구성과 새로운 재조합 제한 | 초기 seed가 쓸 수 있는 일반 데이터 |
| 일반 VM이 offspring draft를 작성 | 복사·검사·분할·재조합 순서까지 진화 | 비용 증가·실패·편향 검증 필요 | **채택** |

EvoOperatorProgram은 특별한 신경망이나 host optimizer가 아니라 유전된 일반 VM module이다. source template과 실제 접근 가능한 donor record를 읽고 offspring draft에 word를 쓴다. “좋은 module 찾기”, “자식 fitness 계산”, “유용한 돌연변이” service는 없다. block 삽입·삭제·이동은 복사 구간과 literal 기록의 조합이다.

### 부모 실행과 자식 연산자의 구별

생식 event를 시작할 때 부모의 실행 연산자 revision, source template revision, event ordinal을 고정한다. 부모에서 실행 중인 연산자와 자식 draft 안에 복사되는 연산자는 다른 객체다. 자식의 연산자 구간도 다른 genome word처럼 복사 오류·삽입·삭제·재조합의 영향을 받는다. 변이 연산자 자신의 유전을 host가 보호하지 않는다.

연산자가 자기 복제 방식을 바꾸는 것은 후손의 연산자 기록을 바꾸는 방식이다. 현재 실행 중인 native 의미나 생식 event의 과거 RNG 결과를 덮어쓰지 않는다. 부모가 생애 중 새 operator revision을 만들더라도 진행 중 생식 event는 시작 revision을 유지한다. 다음 생식 event부터 새 revision을 사용할 수 있으며 germline 기록 갱신은 별도의 실제 복사다. 사용할 source가 아직 물리적으로 전달되지 않았으면 read/copy는 실패한다.

### 단계와 실패 의미

생식 상태는 Empty → Allocating → Writing → Sealing → Ready → Partitioned다. 실제 원료·조직·기억 backing을 먼저 확보하고, operator가 공유 CU와 에너지로 draft를 작성한다. Sealing은 저장 형식·참조 범위·물질 보존·몸의 분할 가능성만 검사한다. 행동의 유용성·정상적인 생존·자식의 미래 성공은 검사하지 않는다.

에너지·원료 부족은 진행 중단이며 다음 tick에 같은 cursor부터 재개할 수 있다. 생식 예약을 유지하는 실제 조직도 유지 비용을 낸다. 부모 사망은 배아의 물질과 기록을 마법처럼 지우지 않는다. 연결이 끊기면 남은 구조는 일반 물리 규칙을 따른다. 자립 가능한지 여부는 기존 생체 조건의 결과다.

형식이 깨진 word는 VM 실행 시 InvalidOpcode가 될 수 있다. source 범위를 벗어난 복사 요청은 무쓰기와 오류이며 쓴 비용을 환불하지 않는다. Sealing 실패 후 살아 있는 부모가 다시 쓰는 것은 실제 시간·자원을 사용하는 행동이다. host는 성공할 때까지 자동 재시도하거나 정상 자식으로 교체하지 않는다.

### 의도된 변이와 복사 잡음

연산자가 기록 순서·구간·확률을 바꾸는 것과 물리적인 복사 오류를 분리한다. 복사 오류는 매체·손상·온도·작업·검사에 따른 고정 noise law다. 기본 reference에서는 각 실제 기록 word에 독립적인 error event를 정의하고 결과 bit 선택까지 주소화된 RNG로 결정한다. 정확한 오류율 함수는 CopyNoiseData-CT2에 들어간다.

유전 프로그램이 오류 검사를 수행하려면 원본·복사본을 실제로 읽고 비교·재기록해야 한다. host가 checksum 또는 의미를 알아서 고쳐 주지 않는다. 짧은 계통은 적게 바꾸고 다른 계통은 큰 block을 재조합할 수 있다. 그러나 다양성을 강제하기 위해 minimum mutation rate를 UI에서 몰래 올리지 않는다. 오류율0 조건도 해당 매체 법칙이 허용하면 인정한다.

변이·재조합률 자체가 선택의 영향을 받을 수 있다는 이론적 근거는 있다. 환경 변화 조건에 따라 결과가 달라지므로 “진화하는 연산자면 반드시 더 잘 진화한다”는 근거로 쓰지 않는다. [Carja, Liberman, Feldman 2014, 원 논문의 초록](https://pubmed.ncbi.nlm.nih.gov/25427794/)

### RNG·동시성·기록

operator RANDOM의 key는 run seed, parent CellUID, reproduction EventUID, operator revision, logical draw ordinal, purpose다. physical copy noise는 EventUID, destination physical word ordinal, 실제 write ordinal, noise purpose를 사용한다. 두 stream은 domain을 분리한다. worker, host cache, 재시도 횟수, wall time은 key에 넣지 않는다.

rollback은 draw cursor·쓰기·비용·재고를 함께 되돌린다. 완료한 write를 다시 시행하는 생물 행동은 새 write ordinal을 사용하고 다시 비용을 낸다. 수치 retry는 같은 사건의 재계산이므로 새 draw를 주지 않는다. RNG 결과를 더 많이 보고 선택하는 프로그램은 그 연산 비용을 내지만 자식을 미래 world에 시험 실행하는 oracle은 얻지 못한다.

GenomeProvenance는 parent revision, 실제 donor 범위, operator revision, 의도한 edit span, copy-noise edit, 최종 bytes hash를 분리한다. 빈 edit와 실패한 edit도 비용 사건으로 추적한다. 이 provenance가 자식의 genome 전체를 무료로 공개하는 감각 채널이 되지는 않는다.

**경계:** W는 복사 잡음·형식·비용·분할 법칙이다. P는 일반 기록·복사·비교·RANDOM·생식 투자다. E는 달라지는 변이·재조합·검사 전략이다. U는 연산자의 계보와 실제 변이를 구별한다.

## 114 코드·기억의 증가와 유한 자원

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 저장 방식의 선택

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 개체마다 최대64word·고정 genome | 예측 가능한 작고 빠른 상태 | 성공한 복잡성도 임의 숫자에서 차단 | CT2에서 폐지 |
| 개체마다 host Vec 자유 증설 | 구현 쉬움 | 무료 기억·heap 분산·재현성 없는 OOM | 직접 사용 금지 |
| 실제 backing + 논리 span + 공유 host page | 자원에 맞춰 길이·stack·기억 확장 | 소유권·fragmentation·journal 복잡 | **채택** |

논리 memory는 u64 word address의 sparse page map이다. host page는 기본4KiB, 즉2,048개의16bit word다. code page는 같은 크기에1,024개의32bit instruction을 담는다. 이 page 크기는 구현 단위이며 실제 매체의 최소 제작 단위와 같다고 가정하지 않는다. 사용하지 않는 host padding은 개체의 물질 비용에 넣지 않는다.

### 실제 backing과 두 종류의 비용

PhysicalRecord는 material slice, 유효 기록 용량, 마모·손상 상태, accessible geometry를 소유한다. LogicalSpan은 이 backing의 어느 구간에 어떤 word가 놓이는지 연결한다. working memory·base code·germline·offspring draft·return stack·외부 기록 모두 비용 대상이다. host가 따로 만드는 무료 parser stack이나 유전 operator scratch는 없다.

capacity는 기록 재료의 양·구조·상태가 정한다. 증설은 실제 기록 가능한 재료를 확보하는 WRITE/기존 backing 조작을 통해 진행된다. internal memory의 직접 LOAD/STORE는 자기 controller의 물리적으로 연결된 backing에만 허용한다. 멀리 있는 외부 기록은 world READ/COPY를 통해 실제 감각·전달 지연을 부담하며 local address로 cast할 수 없다.

기록 cost law는 생성·쓰기·읽기·복사·유지·수리를 구별한다. 모든 매체가 매 tick 같은 유지 에너지를 써야 한다는 가정은 넣지 않는다. 수동적 기록이 오래 남을 수 있으나 공간·물질을 차지하고 마모·환경 손상을 받는다. 생체의 활성 working memory와 수동 외부 기록의 물성은 데이터팩에서 공개한다.

### host 공유와 실제 소유

동일한 불변 genome은 host에서 interning할 수 있다. 그러나 개체100개가 같은 code를 가진다면 세계에는100개의 물리적인 유전 기록과 복사 비용이 있다. 한 개체가 죽어도 다른 개체의 host shared page를 해제하지 않으며, 반대로 마지막 host 참조가 사라졌다고 세계의 실제 외부 기록을 삭제하지 않는다.

host COW는 수정 시 필요한 저장 공간을 늦게 할당한다. mutation이 긴 genome에서1word만 바꿔도 실제 생식 복사는 완성에 필요한 전체 기록 길이·시간·원료를 낸다. host가 rope slice를 O(1)에 만들었다는 이유로 생물 복제가1tick에 끝나지 않는다. 유전된 프로그램이 실제 압축 format을 사용했다면 더 짧은 기록의 물리적 이득은 가능하지만 decoder 저장·해석 비용도 낸다.

64word를 넘는 working memory,8frame을 넘는 call stack,9,216cell을 넘는 code가 물리 backing이 있으면 유효하다. code 길이는 후손 draft나 성체의 유료 code adoption으로 증가할 수 있다. 실행 중 frame은 이전 revision을 유지한다. 신경 weight·staging·학습 trace·경험 기록 역시 실제 backing에 포함한다. host 편의를 위해 학습 buffer만 무료로 둘 수 없다.

### 생태적 실패와 host 실패

NoBacking·NoVolume·InsufficientMatter·InsufficientWork는 실제 세계 내부 실패다. host arena capacity, 파일 공간, 운영체제 RAM, 주소 표현 범위 부족은 RunHalt다. 후자의 경우 journal을 rollback해 마지막 완료 tick을 보존하고 실행을 중지한다. 특정 복잡한 개체의 요청만 생물학적 실패로 바꾸면 host 구현 한도가 선택압으로 작동하므로 금지한다.

사용자가 host memory budget을 지정하면 실행 전 추정과 preflight reserve에 사용한다. budget 초과 시 개체를 자동 제거하거나 긴 genome을 자르지 않는다. 별도 작은-world 실험을 시작할 수 있지만 기존 역사의 계속된 실행이라고 표시하지 않는다. GC는 참조 없는 host cache를 회수하며 원본 규칙에 따라 남아야 하는 물리 기록을 회수하지 않는다.

**경계:** W는 실제 기억 매체·용량·손상·읽기와 복사의 비용이다. P는 유료 국소 기억·기록 조작이다. E는 더 큰 코드·계층적 절차·분산 기록이다. U는 물리 용량과 host RAM 사용을 별개로 보여준다.

## 115 결과를 보존하는 최적화

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 세 종류의 변경을 분리한다

| 분류 | 허용 조건 | 예와 판정 |
| --- | --- | --- |
| A: 정확한 구현 최적화 | 같은 완료 tick 상태·event·RNG·논리 비용 | SoA, interning, predecode, bit-exact SIMD; reference hash 일치 |
| B: 수치 근사 변경 | 별도 DiscretizationSpec, 보존·수렴·오차·현상 gate | AMR, homogenization, 다른 solver; 같은 trajectory 보장 안 함 |
| C: 모델 범위 변경 | 새 ruleset/profile와 새 자연사 | 파동 끄기, 반응 삭제, 신경 비용 변경; 단순 가속으로 표시 금지 |

render FPS, 카메라, UI 열림 여부, worker 수, 저장 속도는 A의 결과에 영향을 주면 안 된다. 반면 B의 작은 수치 차이는 긴 생태 역사에서 다른 결과를 만들 수 있다. B를 A와 같은 “완전히 동일한 세계”로 광고하지 않는다. wall time은 profiler 정보이고 생물의 비용 입력이 아니다.

### hot path와 데이터

ElementRegistry와 작은 immutable SpeciesDef는 실행 전체에서 공유한다. 국소 조성은 실제 비어 있지 않은 항만 순회한다. agent hot state·reaction endpoints·neural edge·graph atom은 SoA나 작은 AoSoA로 batch 처리하고, code·계보·긴 graph metadata는 cold arena로 분리한다. 여러 필드를 항상 함께 쓰는 VM register와 return frame은 작은 AoS를 유지한다.

긴 가변 데이터에 개체별 Vec 수십 개를 붙이지 않는다. 64bit 논리 span→chunk directory→dense page로 두 단계 주소화하고 자주 쓰는 첫 page를 cache한다. chunk별 free list와 generation ID로 생성·삭제를 처리한다. 작업 중 신규 ID는 worker의 임시 local key이며 Commit의 원인 순서에서만 세계 UID로 확정한다.

화학 반응 후보는 실제 active interface·접촉·농도 변화로 dirty가 된 국소 영역만 재생성한다. 다만 “변화가 작아 보인다”는 이유로 멈추지 않는다. exact dormant certificate는 외부 flux·reaction propensity·구동·손상·scheduled boundary가 모두 정확히0인 경우에만 유효하다. incoming event가 생기면 해당 face와 이웃 의존성을 계산 전에 깨운다.

### VM·신경망·학습의 batch

동일 code revision과 PC에 있는 controller를 묶어 SIMD로 처리할 수 있다. diverge한 lane은 scalar로 되돌리고 stable agent 순서의 effect intent를 보존한다. 개체별 CU·에너지·backing·fault·query budget은 분리한다. 같은 gene라도 배운 weight와 somatic memory가 다르면 mutable state를 공유하지 않는다.

희소 신경망은 동일한 immutable topology의 edge order를 공유하고 state·weight·pending update는 개체별로 둔다. 구조가 다른 graph를 맞추려고 의미 없는 edge를 계산하면 host overhead가 증가하므로 작은 graph batch와 scalar fallback을 비교한다. GPU는 기본 결정론 backend에서 제외하고, 나중에 동일 정수 결과·overflow·중단 prefix를 입증한 backend만 A로 승인한다.

NCYCLE을 SIMD로 빨리 풀어도112절의 CU와 지연을 줄이지 않는다. 학습 프로그램이 weight를 갱신하지 않는 기간에는 동일 revision cache를 활용할 수 있다. revision이 바뀌면 영향받는 decoded span과 weight lane만 무효화한다. host가 자주 쓰이는 network를 더 많이 훈련하거나 드문 topology를 버리지 않는다.

### 화학·기록·저장

국소 motif fingerprint는 candidate index다. hash 충돌 시 실제 원소·edge·state를 대조한다. canonicalization·에너지 계산 cache는 모든 의존 revision, 온도·phase·geometry 등 해당 함수의 입력을 포함한다. 냉각·응력·용매 변화 뒤에 예전 반응률을 재사용하는 것은 정확 최적화가 아니다.

복제·code rope·반복 graph의 host 압축은 authoritative material cost와 분리한다. inactive code page는 압축할 수 있지만 PC가 접근하면 같은 bytes를 복원한다. host disk paging 때문에 해당 controller의 세계 tick만 늦추지 않는다. 완료를 기다려 전체 wall time이 늘어나거나 RunHalt해야 한다.

checkpoint는 dirty page와 content-addressed immutable blob을 공유하고 정기적인 full root를 저장한다. delta chain 길이는 예를 들어 기본32개 checkpoint에서 새 base를 만드는 저장 정책으로 제한한다. 이는 world rule이 아니다. 복원에 필요한 code revision·graph·신경 state·RNG cursor·material ownership을 먼저 보존한다. observer의 자세한 raw trace를 sampling할 경우 샘플링 범위를 명시하고 확정 provenance event와 혼동하지 않는다.

### 최적화 도입 순서

우선 allocation·복사·cache miss·반응 candidate 수·NN edge 처리·EM stencil·rollback을 phase별로 측정한다. 다음으로 slab/arena·dirty index·immutable sharing·predecode를 적용한다. 이후 실제 병목인 kernel에 SIMD와 batch를 적용한다. 모든 subsystem을 미리 병렬화하거나 JIT부터 만드는 것은 목표가 아니다.

필수 비교는 scalar single-thread, optimized single-thread, optimized multi-thread의 같은 seed·manifest·입력·tick hash다. 다르면 모델이 같다고 표시할 수 없다. A의 속도 개선은 전체 workload에서 보고하며 cache warm run만 골라 발표하지 않는다. R9는 이 최적화를 설계했으며 속도 개선 배수를 측정한 상태가 아니다.

**경계:** Engineering Contract는 같은 세계 결과를 유지하는 저장·실행 방식이다. W/P의 비용·권한은 유지된다. E는 최적화 전후 동일해야 한다. U는 TPS와 논리 비용을 분리한다.

## 116 파동·장·환경의 비용을 줄이는 조건

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### R8의 큰 병목은 그대로 남는다

R8 reference는2D3성분 Maxwell, dx=dy=0.25tile, c=32tile/tick, Courant S=0.5다. dt=S·dx/c=1/256tick이므로 tile당16개 fine cell을256번 갱신한다. 즉 tile당4,096 cell-update/tick이다. 원소 registry와 VM을 빨리 만들어도 이 비용은 사라지지 않는다.

이산 파동은 격자·시간 간격에 따른 안정성과 해상도 제한을 가진다. [Meep의 FDTD 공식 설명](https://meep.readthedocs.io/en/latest/Introduction/)을 수치 원리의 참고로 사용한다. 아래 AMR 결합은 본 프로젝트의 설계 후보이며 Meep가 이 이동 재료·수문·진화 세계를 해결해 준다는 뜻은 아니다.

### 세 대안

| 방법 | 장점·개방성 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 전체 fine grid | 가장 단순한 reference, 모든 곳 동일 해상도 | 메모리·시간이 세계 면적에 강하게 비례 | 정확성 기준 유지 |
| multi-level field grid와 국소 subcycle | 긴 파장·매끄러운 영역에서 절약 가능 | interface 반사·전하·에너지·moving material 결합이 어려움 | **최적화 연구 목표**, gate 전 기본 비활성 |
| 먼 영역의 파동 삭제·즉시 신호 변환 | 매우 빠름 | 약한 통신·간섭·공진을 다른 세계로 바꿈 | CT2 동일 모델 최적화로 금지 |

### 구체적인 AMR 후보

2:1 refinement만 허용하고 level0 dx=1tile, level1=0.5tile, level2=0.25tile로 둔다. 해당 substep 수는64,128,256/tick이다. refinement는 카메라가 아니라 material geometry, source bandwidth, field truncation estimator, 이동 경계의 예측 범위로 정한다. topology와 결정 기준은 DiscretizationSpec-CT2-AMR에 고정한다.

coarse/fine interface는 staggered D/B와 charge/current의 compatible transfer를 사용한다. interface flux register에 실제 전류와 EM 에너지 이동을 모아 coarse step 완료 때 정산한다. divergence 제약·charge continuity·field/material work가 동시에 맞아야 한다. 서로 다른 격자의 값을 단순 평균한 뒤 남는 에너지를 열로 지급해서 오류를 숨기지 않는다.

reference와 비교할 파장 해상도 기본 gate는 최소16cell/파장이다. 이는 충분조건을 증명한 물리 상수가 아니라 오차 시험의 시작 기준이다. level0은 λ≥16tile, level2는 λ≥4tile에 해당한다. source가 짧은 파장을 만들면 그 파동이 지나갈 영역도 필요한 해상도를 가져야 한다. 발생지 주변만 fine으로 두면 먼 곳의 짧은 파장 신호가 사라진다.

field front가 coarse cell에 도착하기 전에 refine하도록 propagation bound와 guard halo를 둔다. 동시에 약한 파동을 magnitude threshold만으로0으로 지우지 않는다. 공진 cavity·도체 slit·박막·피부 깊이·재료 dispersion의 해상도 요건도 독립적으로 확인한다. world 전체가 fine이 되는 상황은 정상적인 최악 사례다.

moving material의 ownership remap과 field refinement가 겹치면 같은 coupling journal에 포함한다. 실패 시 hierarchy, staggered time, PML, charge, constitutive history, boundary ledger까지 rollback한다. adaptive cell 배치와 예정 refine queue는 미래를 바꾸므로 checkpoint에 저장한다. 원래 run을 재생할 때 관찰자의 hardware에 맞춰 임의로 grid를 바꾸지 않는다.

### 절약량의 상한을 과장하지 않는다

단순한 leaf-only 계산 예에서 level0 영역 비율1−f, level2 영역 f라고 두면 업데이트 수는 tile 수×[64(1−f)+4,096f]다. level1 전이 영역·halo·coarse predictor·reflux·반복 solve·검사 비용은 이 식에 없다. 실제 AMR의 실행 시간을 예측하는 식이 아니다.

f=1%이면 tile당104.32, f=5%이면265.6, f=100%이면4,096이다. 전체 fine 대비 cell-update 수가 각각 약39.26배,15.42배,1배로 줄어드는 이상적인 산술 예다. **TPS가 그만큼 증가한다는 예측이 아니다.** 파동이 전 영역을 채우거나 coupling이 비싸면 이 이점은 작아진다.

### 환경·화학의 다중 시간척도

느린 침식·퇴적·대기 reservoir를 적은 빈도로 계산하려면 그 사이 실제 교환 flux를 누적하고 양방향 영향을 같은 시간 구간에서 풀어야 한다. thermal·chemical rate가 변했는데 예전 값으로 오래 extrapolate하면 다른 선택압이 생긴다. error tolerance와 step schedule은 manifest에 넣고 불연속 사건·재고 고갈에서 step을 끊는다.

고분자·구조물의 homogenization도 B다. 평균 물성이 강도·파괴·phase·수송·신호 응답을 보존하는 범위를 먼저 확인한다. 개체가 최적화의 경계 오차를 이용하면 수치 결함으로 분류하고 해당 run의 결론을 유보한다. 작은 feature를 지워서 생긴 효율 증가를 발명으로 해석하지 않는다.

**경계:** Engineering Contract는 해상도·시간 적분·오차 검증이다. W/P는 명시한 연속 모델을 목표로 한다. E의 장기 역사 일치까지 보장하지 않는다. U는 reference/근사 profile·오차·실측 성능을 표시한다.

## 117 Rust 구성·자료형·소유권·phase·저장

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### crate와 의존 방향

| crate/module | 소유하는 규칙·자료 | 의존과 역할 |
| --- | --- | --- |
| civ_types / civ_numeric | key·span·checked 정수·고정소수점·시간 | 다른 domain 의존 없음 |
| civ_chem::elements, graph, rewrite, thermo | 원소 registry·graph view·반응·에너지 | types/numeric만 기반; 실제 store는 core 소유 |
| civ_vm::isa, paged_memory, meter, kernels | CT2 instruction·논리 비용·실행 | 세계를 직접 소유하지 않음 |
| civ_neural::graph, epoch, plasticity_io | 희소 graph·정수 kernel·학습 state I/O | types/numeric; VM의 kernel 요청으로 실행 |
| civ_bio::development, reproduction, inheritance | 발생·생식·source/draft·operator 실행 경계 | VM·chem의 read view 사용 |
| civ_material / civ_physics / civ_environment | 기존 조립·기계·장·수문·기후 | DTO로 결합, 서로의 store 직접 수정 금지 |
| civ_core::phases, stores, journal, audit | 모든 authoritative store·transaction·rollback | 각 domain의 view/delta를 조정 |
| civ_runner / civ_storage | worker pool·속도·checkpoint·replay | Core를 단독 소유하는 runner |
| civ_observer / civ_viewer | 행동·학습·계보 분석 / Bevy 화면 | 읽기 전용; core로 label·보상 반환 금지 |

civ_neural이 civ_vm을 다시 의존하는 cycle을 만들지 않는다. LearningProgram은 VM bytecode이므로 core가 같은 controller의 예산과 I/O view를 제공해 실행한다. neural crate는 숫자와 graph를 처리하며 agent의 목표·생존 점수·직업을 알지 못한다. neural 기능이 없는 agent에는 빈 graph handle만 필요하고 edge arena를 할당하지 않는다.

### 핵심 struct와 enum의 필드 계약

아래는 Rust 구현에 옮길 데이터 계약이며 전체 소스코드가 아니다. field 크기 합과 Rust struct의 실제 size_of는 alignment 때문에 다를 수 있다. binary save는 struct 메모리를 그대로 dump하지 않는다.

| type | 핵심 필드 | authoritative owner |
| --- | --- | --- |
| ElementRegistry | Vec<ElementDef>, sorted symbol index, spec_hash | immutable RunSpec |
| ElementCountSpan | inline4 또는 arena span, count:u32 | ChemStore |
| MolecularGraphStore | atom/bond SoA pages, generation, component cache revisions | ChemStore |
| LogicalSpan | record key, offset:u64, len:u64, rights, revision:u64 | RecordStore |
| PhysicalRecord | backing slice, capacity, damage, geometry, revision | Material/Record 단독 연결 |
| VmStateCT2 | regs:[u16;16], pc:u64, code revision, stack cursor, fault, CU remaining, pending kernel | ControllerStore |
| NeuralGraph | node span, edge span, input/output mapping, topology revision | immutable 또는 pinned graph record |
| NeuralState | node values, somatic weights, trace span, state/weight revisions | ControllerStore의 전용 span |
| NeuralEpoch | input snapshot, graph/weight pins, phase, cursor, accumulator, staged state/output | ControllerStore |
| LearningRecord | actual experience span, update draft, source revisions, paid backing | RecordStore |
| ReproductionTxn | EventUID, source/operator pins, draft, cursor, draw/write ordinals, reservations | BioStore |
| MutationProvenance | parent/donor/operator revisions, actual edit/noise spans, final hash | immutable event journal |
| RunHalt | HostMemory, HostStorage, NumericRange, Conservation, UnsupportedSchema | Runner status, agent fault 아님 |

RecordKind는 Executable, Germline, Somatic, External, OffspringDraft 같은 접근·수명 분류이며 지식의 의미를 나타내지 않는다. KernelPhase는 InputStage, EdgeAccumulate, NodeFinalize, OutputStage, Publish다. ReproductionPhase는113절 상태다. OptimizeClass는 Exact, Discretization, ModelChange다. 코드에는 Job·Civilization·TechnologyUnlock 같은 enum을 추가하지 않는다.

에너지 사용은 local metabolic work를 소비해 해당 thermal owner로 열을 옮기는 기존 ledger에 기록한다. CU 비용을 단순히 숫자에서 빼고 총 에너지에서 사라지게 하지 않는다. neural·learning·mutation도 같은 경로이며 host 검증·캐시 관리 CPU 비용은 세계의 추가 열이 아니다.

### 한 tick의 실행 순서

1. Preflight: 불변 manifest·입력·scheduled boundary를 고정하고 필요한 host reserve와 rollback journal을 준비한다. 물리 capacity·backing·의존 revision을 snapshot으로 만든다.
2. Sense/Compute: 국소 감각 frame, 일반 VM, 진행 중 NeuralEpoch, 유전 프로그램이 호출한 학습/발생/생식 절차를 controller별 shared CU 안에서 실행한다. 다른 controller의 미완료 상태를 읽지 않는다.
3. Intent/Reserve: 실제 물질·work·접촉·기록 쓰기·새 backing·code adoption 요청을 안정된 원인 순서로 예약한다. 한 controller의 신경 학습과 복제가 같은 원료를 두 번 쓸 수 없다.
4. Coupled trial: R8의 반응·수송·기계·조립·EM·복사·환경을 같은 시간 구간에서 푼다. 개체 수나 학습 성공을 물리 forcing 입력으로 사용하지 않는다.
5. Audit/Retry: 원소·전하·에너지·비관통·양수 재고·record ownership·field residual을 검사한다. 실패 시 계산 state·learning delta·RNG cursor까지 함께 rollback한다.
6. Viability/Activation: 생식 분할·사망·backing 손상과 유효한 code/weight/topology adoption을 확정한다. 진행 중 frame·epoch·operator의 revision pin은 유지하고 새 activation부터 새 revision을 사용한다.
7. Commit: canonical event order로 UID와 species record를 확정하고 완료 tick hash·facts·immutable snapshot을 발행한다. viewer·observer 처리 시간은 다음 tick의 생물 예산을 바꾸지 않는다.

학습 program이 새 weight를 작성한 tick에 이미 시작한 NeuralEpoch의 weight를 소급 변경하지 않는다. code adoption 요청이 물리 backing 손상 때문에 실패하면 host가 이전 source를 무료 복구하지 않는다. birth는 germline/template 실제 복사 결과이며 부모의 모든 SomaticState를 복제하는 memcpy가 아니다.

### threading과 결정론

Runner의 simulation thread가 Core를 단독 소유한다. 고정 worker pool은 phase별 read view와 서로 겹치지 않는 local delta buffer를 받는다. controller 계산·국소 candidate 생성·stencil interior는 병렬로 실행하고, 공유 stock·접촉·code write·face flux는 canonical owner가 해결한다. thread 완료 순서로 ID·난수·우선권을 결정하지 않는다.

global reduction은 고정된 chunk/UID 순서 또는 결정론적 reduction tree를 사용한다. RNG는 stream key와 logical ordinal을 저장한다. CT2 neural reference는 정수이지만 기존 물리 solver에는 부동소수점이 있으므로 1/2/N worker hash 일치와 플랫폼 재현성을 별도 검사한다. 모든 CPU·compiler에서 bitwise replay가 된다고 아직 보장하지 않는다. NumericBackend fingerprint가 다른 경우 지원 상태를 표시한다.

### CT2 save와 replay

magic은8byte CIVCT002다. 필수 manifest hash는 ElementRegistry, ChemistryData, GraphRepresentation, BaseVmCT2, ComputeCost, NeuralKernel, CopyNoise, MemoryBacking, Material/EM/Environment, DiscretizationSpec, NumericBackend다. 값이 달라지면 같은 replay로 처리하지 않는다. CT1은 legacy backend로 열거나 명시적 intervention branch를 만든다.

가변 code·graph·memory의 length/offset은 save에서도 u64 LE다. page payload length는 해당 block 범위 안에서 검증한다. section의 offset+length, count×stride는 checked 연산이며 파일 밖 참조·겹치는 독점 backing·중복 key·알 수 없는 필수 enum을 거부한다. u32 list length를 남겨 놓아 다시 숨은 크기 상한을 만들지 않는다. 실제 파일/메모리 reserve는 검증 후에 수행한다.

중간 NeuralEpoch의 input, frozen weight/graph revision, cursor, accumulator, staged result, pending learning update를 저장한다. 생식 source/operator revision, cursor, RNG/write ordinal, 예약 재고도 저장한다. chemical graph·microstructure·D/B stagger·constitutive history·환경 reservoir·AMR hierarchy와 retry가 아닌 미래에 필요한 history를 모두 보존한다. cache는 버려도 같은 다음 tick이 나오는 것만 제외한다.

같은 입력에서 visual/headless의 tick hash가 같아야 한다. 빠른 replay는 실제 이벤트 기록만으로 world를 임의 근사하지 않고 checkpoint+입력으로 authoritative core를 재실행한다. observer timeline은 원본 fact와 사후 해석 version을 별도로 저장한다.

**경계:** Engineering Contract는 자료·주소·저장·thread·phase다. W/P는 위의 검증된 경계로만 state를 바꾼다. E는 코어가 계산한 결과다. U는 상태·비용·계보·학습 증거를 읽기 전용으로 보여준다.

## 118 메모리 산정과 성능 목표

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 산정의 전제

아래 수치는 배열 크기와 연산 수 계산이다. Rust size_of 실측, profiler 결과, TPS benchmark가 아니다. 사용자에게는 실제 실행 후 CPU·RAM·compiler·seed·밀도·반응 수·NN graph·retry율·관찰/저장 설정을 함께 보고해야 한다. memory budget에는 rollback·snapshot·allocator overhead·최고 동시 사용량을 포함한다.

| 항목 | 계산 예 | raw 크기 또는 작업량 |
| --- | --- | --- |
| 원소 global ledger | 16×u128 | 256byte; tile마다 복제하지 않음 |
| 불필요한 tile별 원소 배열 | 10,240,000tile×16×u16 | 312.50MiB; 실제 조성 span으로 회피 |
| 100,000 controller의1KiB memory | 100,000×1,024byte | 97.66MiB; page slack 별도 |
| 같은 개체의16KiB memory | 100,000×16,384byte | 1,562.50MiB |
| 각4KiB genome의 논리 기록 | 100,000×4,096byte | 390.63MiB; 물리 복사 비용은 공유 불가 |
| 동일 genome이1,000종일 때 payload cache | 1,000×4,096byte | 3.91MiB; 개체 refs·물리 매체·변이 page 별도 |
| NN node 계획 stride32byte | 100,000개체×16node×32 | 48.83MiB; 실제 struct 검증 필요 |
| NN edge 계획 stride32byte | 100,000개체×64edge×32 | 195.31MiB; weight/state packing 후보 |
| controller 기본 CU | 100,000×32/tick | 3,200,000 CU/tick; host instruction 수 아님 |

신경망 예의 node+edge만 약244.14MiB다. input/output staging·학습 trace·physical backing·revision pin·topology index·page 낭비는 별도다. 모든 개체에4KiB page를 하나씩만 더 붙여도390.63MiB다. 따라서 작은 somatic state는 shared slab 안의 분리된 slot으로 저장하고 큰 state만 page로 승격한다. 이 승격은 물리 용량을 바꾸지 않는다.

I=8,M=64,N=16,O=4인 신경 epoch는 C=1+32+512+64+8=617 CU다. 기본32 CU/tick controller는 다른 작업이 전혀 없어도 최소20tick이 필요하다.100,000개체가8tick마다 이런 epoch를 수행하려면 평균7,712,500 CU/tick이 신경 계산에만 필요하다. 기본 총3,200,000 CU로 그 빈도를 달성한다고 가정할 수 없다. 더 많은 실제 compute backing이나 낮은 빈도·작은 망이 필요하다.

학습·VM 감각 처리·복제 비용은 이617 CU에 포함되지 않는다. 예를 들어 학습 절차가64edge에 각각12 CU를 쓰면768 CU가 추가되고 각 weight의 실제 write 비용도 낸다. GPU batch가 빨라졌다는 이유로 세계 내부 계산량을 무료로 늘리지 않는다.

### 파동장 원가

| world | fine field cell 수 | field cell-update/tick | D/B 6×f64만 |
| --- | --- | --- | --- |
| 64×64 | 65,536 | 16,777,216 | 3MiB |
| 128×128 | 262,144 | 67,108,864 | 12MiB |
| 512×512 | 4,194,304 | 1,073,741,824 | 192MiB |
| 3,200×3,200 | 163,840,000 | 41,943,040,000 | 7,500MiB |

D/B만으로 실제 메모리를 추정하면 안 된다. geometry·charge·current·constitutive history·halo·PML·rollback을 포함해 planning192byte/cell을 잡으면 마지막 행은 약29.30GiB다.192byte는 확정 layout이 아니라 memory planning 예산이다. 여기에 지형·화학·생물·기록·신경 state가 더해진다.

AMR의 이상적인 fine 비율1% 예도1,024만tile에서1,068,236,800 cell-update/tick이다. 기준 fine보다 줄어들지만 여전히 큰 계산이다. 넓은 세계·짧은 파장·많은 복잡한 agent·높은 TPS를 동시에 보장할 수 없다. “NN 최적화로 전체 세계가 빨라진다”는 주장도 phase 비중을 측정한 뒤에만 한다.

전체 wall time의10%를 차지하는 controller를10배 빠르게 해도 전체는 약1.10배다.90%인 physics를10배 빠르게 하면 약5.26배다. 이 산술은 병목의 중요성을 설명하는 예이며 실제 subsystem 비율은 아직 측정되지 않았다.

### 측정 profile과 gate

| profile | workload | 우선 판정과 목표 |
| --- | --- | --- |
| CT2-Kernel | 고정 fixture에서0/16/64/256node, 다양한 graph·code·memory | 비용·중단·복원·단일/다중 thread hash; TPS보다 CU/s·edge/s 보고 |
| CT2-Reference | 64²tile,100controller,2,000Cell,10,000MicroNode,20,000Bond, full fine field | 12 CPU core/32GiB급 기준1 TPS를 provisional 목표; 미실측 |
| CT2-Research | 128²tile,1,000controller,20,000Cell,100,000MicroNode,200,000Bond | 장기 학습/유전/생태·오차·메모리·retry 측정; TPS gate 동결 전 |
| CT2-Scale | 512² 이상, chemistry·field·NN 밀도 공개 | exact 최적화 이후 실측; AMR은 별도 profile |
| 대규모 최종 목표 | 1,024만tile,10만agent와 복잡한 세계 | R5의10 TPS를 CT2 보장치로 재사용하지 않음 |

reference의 controller/Cell 차이는 다세포 몸과 무controller 조직을 허용하기 때문이다. agent 수만 보고 비용을 비교하지 않는다. 각 profile에 원소·species·graph atom·bond·반응 후보·active field·NN edge·기록 byte와 최대값을 함께 기록한다.

1x는 wall time에 대한 목표 시뮬레이션 tick 비율이며 PhysicsSpec의 dt를 변경하지 않는다.5x/50x/500x/MAX도 같은 논리 tick을 더 많이 수행하려는 요청이다. 달성 TPS가 낮으면 실제 속도를 표시한다. MAX는 snapshot·UI 빈도를 줄이고 효과를 생략하되 physical tick·학습·mutation·field 계산을 선택적으로 생략하지 않는다.

**경계:** Engineering Contract는 비용 산정·benchmark·메모리 예산이다. W/P의 에너지·계산량과 host wall time은 다르다. E를 장기 관측할 수 있는지는 실측에 달려 있다. U는 목표·추정·실측을 구별한다.

## 119 R9 검증 계약:64개 기대 사례와 복합 witness

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

아래는 구현 시 통과해야 할 조건이다. 지금 실행한 simulator test가 아니다. legacy261개와 함께 번호를 보존하지만 다른 profile의 서로 다른 규칙을 한 test suite의 동일 기대값으로 합치지 않는다. 문서의 산술·표 구조 검사는 이 물리·생태 시험을 대신하지 않는다.

### 원소·조성 Z01..Z08

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z01 | 기존8종 뒤에8종을 추가한 registry | 기존 symbol의 의미 유지, 길이16; core의 배열 길이8 가정 없음 |
| Z02 | Na23unit 질량의 원소1개와 Cl35unit 원소1개를 결합 | 총 질량58unit, 원소별 개수 각1; 이름으로 추가 능력 없음 |
| Z03 | 미등록 state·누락 interaction 자료가 있는 데이터팩 | world 생성 전에 명확한 검증 실패; random coefficient 대입 없음 |
| Z04 | 초기에 재고0인 원소를 일반 화학 반응으로 생성 요청 | 원소 보존 검사 실패; 새 원소 unlock 없음 |
| Z05 | sparse 조성 항 순서가 다른 동일 입력 | 정렬 후 같은 조성·질량·canonical bytes |
| Z06 | 수량×계수가 u128 범위를 초과 | rollback 후 NumericRange halt; wrap·saturation으로 성공 금지 |
| Z07 | 같은 material slice를 graph와 자유 stock이 동시에 소비 | 두 번째 예약 거부 또는 audit 실패; 물질 생성 없음 |
| Z08 | registry만 다른 checkpoint를 현재 backend로 재생 | manifest 불일치; 무음 변환 없음 |

### 큰 화학 구조 Z09..Z16

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z09 | 충분한 원료·공간의65 heavy atom graph | 크기64라는 이유로 반응 거부·삭제하지 않음 |
| Z10 | 실제 원료로4,097 monomer 사슬을 형성 |4,096 상한 적용 없음; 실제 질량·길이·비용 반영 |
| Z11 | 동일 graph를 다른 atom 삽입 순서로 작성 | 같은 물리 응답; software ID 차이로 mixing 이득 없음 |
| Z12 | 국소 fingerprint가 충돌하는 다른 graph | 실제 비교로 구별; cache 결과 무단 공유 없음 |
| Z13 | 반복 압축과 완전 전개 graph의 같은 국소 사건 | 원소·결합·extent·에너지·완료 tick 상태 일치 |
| Z14 | 압축 사슬 중간 절단과 가교 | 실제 연결·성분·기하 변경, 손상 구간만 host COW 가능 |
| Z15 | 큰 graph의 global connectivity 검사 중 host 처리 지연 | world 시간·반응률을 임의 변경하지 않음 |
| Z16 | bulk/instance 전환에 entropy 대응 자료 없음 | 해당 변환 capability 미승인; 누락 에너지를 자동 생성하지 않음 |

### VM·계산 표현 Z17..Z24

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z17 | 유효 backing의65,536 이상 local word 주소 | LOAD64/STORE64 정상; 낮은16bit 주소로 alias되지 않음 |
| Z18 | LOAD64의 결과 rb가 주소 register와 겹침 | instruction 시작의 주소 사용 후 결과 기록 |
| Z19 | CALL64에서 paid frame 공간이 부족 |2 CU 지불, StackCapacity, PC+1, stack 불변 |
| Z20 | 넓은 주소+signed offset이 음수 또는 u64 overflow | BadAddress; world pointer나 반대쪽 메모리로 wrap 없음 |
| Z21 | guest 해석이100개의1 CU 연산을 수행,32 CU/tick | 최소4tick,100 CU; CALL 하나 비용으로 대체 금지 |
| Z22 | 같은 activation에서 behavior·학습·mutation 실행 | 하나의 controller CU pool 공유; 각 절차에 새 pool 없음 |
| Z23 | compute backing을 tick 중 추가 | 해당 tick capacity 불변, 다음 tick부터 증가 |
| Z24 | cache fused block 중간에서 연료 부족·fault | scalar와 PC·비용·fault·I/O·tick hash 일치 |

### 신경망·생애학습 Z25..Z32

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z25 | source0.5, weight0.5, bias0의 단일 edge | node output0.25; fixed-point 정수 결과 일치 |
| Z26 | node A/B 사이 양방향 edge와 서로 다른 초기값 | 두 갱신 모두 이전 epoch 값을 사용; 순서 의존 없음 |
| Z27 | I8/M64/N16/O4, 에너지 충분,32 CU/tick |617 CU, 최소20tick; 중간 output publish 없음 |
| Z28 | NeuralEpoch 도중 저장·복원·worker 변경 | staging·cursor·frozen input·다음 output·비용 동일 |
| Z29 | epoch 진행 중 새 weight revision 채택 요청 | 현재 epoch는 이전 weight, 다음 activation은 새 revision |
| Z30 | 학습한 somatic weight를 germline에 복사하지 않고 생식 | 자식은 실제 germline 초기값; 학습 state 자동 상속 없음 |
| Z31 | 같은 genome·현재 자극, 다른 경험의 검증 fixture | 학습 state 차이·반응 차이 관측; reset branch에서 원인 대조 |
| Z32 | neural graph/backing 손상 또는 CU·에너지 부족 | 손상은 취소·쓴 비용 유지; 일시 부족은 정확한 중단/재개 |

### 변이 연산자 Z33..Z40

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z33 | 부모 operator가 자식 operator 구간을 수정 | 현재 생식은 pin한 부모 revision, 자식은 기록된 새 bytes |
| Z34 | 물리적으로 접근하지 못한 donor code 요청 | local read/copy 실패; registry에서 code 무료 획득 없음 |
| Z35 | 연산자가 유효하지 않은 명령을 offspring에 기록 | 형식/실행 계약대로 실패; host가 유익한 명령으로 수리하지 않음 |
| Z36 | operator CU 부족으로 draft 중간 중단 | cursor·재고·비용 보존 후 재개; 무료 완성 없음 |
| Z37 | 동일 source와 key에서 thread 순서·수치 retry 변경 | 동일 edit·noise·draw cursor·최종 offspring |
| Z38 | 생물이 같은 word를 실제로 다시 기록 | 새 write ordinal·새 물리 비용; 수치 retry와 구별 |
| Z39 | 생식 중 부모 사망 | 실제 배아·기록·물질의 물리 상태 유지, 자동 정상 자식 없음 |
| Z40 | copy-noise0 조건과 변이를 안 하는 유전 operator | 무변이 복사 허용; novelty 유지를 위한 강제 변이 없음 |

### 기억·코드 Z41..Z48

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z41 | backing이 있는65word memory·9frame stack | 과거64word·8frame 상한 때문에 실패하지 않음 |
| Z42 |9,217cell 이상의 유효 code record | 과거9,216cell 상한 없음; 일반 주소·물리 비용 적용 |
| Z43 | 동일 genome100개를 host에서 공유 | host payload 절약, 실제100개 기록·복사 비용은 유지 |
| Z44 | 긴 rope에서1word 수정·전체 offspring 복제 | host COW 비용과 실제 전체 복사 시간·원료 구별 |
| Z45 | backing은 충분하나 host RAM reserve 실패 | tick rollback·RunHalt; 특정 개체의 NoBacking으로 변환 금지 |
| Z46 | 멀리 있는 외부 record 주소를 local pointer로 위조 | 접근 실패; 실제 접촉·전달 경로 필요 |
| Z47 | host page padding이 다른 두 storage backend | logical capacity·원료·tick 결과 동일 |
| Z48 | old code가 pinned된 상태로 새 code 채택 | old frame 주소 유지, 새 activation만 새 revision; 무료 backing 없음 |

### 정확 최적화·물리 근사 Z49..Z56

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z49 | scalar/predecode/SIMD와1/2/N worker | 동일 규칙에서 완료 tick·event·RNG hash 일치 |
| Z50 | host cache cold/warm와 화면 zoom·MAX 변경 | 세계 state·신경/학습 비용 동일; wall time만 다를 수 있음 |
| Z51 | dormant 영역에 외부 flux·파동·접촉 유입 | 계산 전에 의존 영역 wake; 실제 반응·전파 누락 없음 |
| Z52 | scalar NN과 topology batch, somatic weight 서로 다름 | 각 개체 output·CU 동일; 학습 weight가 서로 공유되지 않음 |
| Z53 | AMR coarse/fine 경계를 통과하는 파동 | reference 대비 반사·위상·에너지·Gauss 오차를 측정하고 gate 만족 |
| Z54 | 약하지만 유효한 짧은 파장 신호가 멀리 전파 | threshold 삭제 금지; 필요한 경로 refine 또는 profile 미지원 표시 |
| Z55 | AMR hierarchy 변경 뒤 coupling retry | hierarchy·charge·D/B·flux·재료 상태 전체 rollback |
| Z56 | topology pruning 또는 빠른 field 삭제 제안 | Exact가 아닌 ModelChange/Discretization으로 분류; 무음 적용 금지 |

### 저장·관찰·결합 Z57..Z64

| ID | 입력·조건 | 기대 결과 |
| --- | --- | --- |
| Z57 | 중간 생식·학습·NN epoch를 포함한 checkpoint | 다음 tick부터 state·비용·offspring·event 동일 |
| Z58 | CT1 save를 CT2 ruleset으로 그냥 열기 | UnsupportedSchema/manifest 불일치; legacy 실행 또는 명시적 분기 |
| Z59 | code·graph section의 offset+length overflow·중복 backing | 할당/실행 전에 CorruptSave; 부분 world 채택 금지 |
| Z60 | observer가 새 기술·문명·높은 novelty 판정 | primitive 권한·번식·energy·학습 reward 변화0 |
| Z61 | learned module을 실제 외부 기록·복사로 전달 | copy·adopt·실제 사용·기능 증거를 각각 기록 |
| Z62 | 전문 개체 사망·기록 손상·기존 구조 유지 | 지식과 artifact의 수명 분리; 기술 소실 가능 |
| Z63 | NN·learning·mutation이 에너지를 소비하는 닫힌계 | work 감소와 thermal 증가 정산; 총 에너지 오차 gate 만족 |
| Z64 | 동일 seed/config/input의 headless·visual·재시작 | 지원 NumericBackend에서 동일 완료 tick hash |

### 기능 witness와 자연 발생 증거

W-CT2-1은16원소 후보 중 추가 원소를 실제로 포함한 재료의 반응·수송·구조 기능을 원료부터 검증한다. W-CT2-2는 과거 분자 상한을 넘는 사슬의 국소 결함·가교·파괴를 검증한다. W-CT2-3은 같은 base VM 위에 만든 두 guest decoder의 전달·비호환·비용을 검증한다. W-CT2-4는 작은 neural module의 유료 생애학습·망각·체내 state와 외부 기록의 차이를 검증한다.

W-CT2-5는 유전 operator가 달라진 후손의 변이 경로와 실제 copy noise를 구별한다. W-CT2-6은 긴 code와 공유 host page의 물리 복사 비용을 검증한다. W-CT2-7은 물리 환경 변화로 기록·대사·학습의 유용성이 달라지는 결합 사례다. W-CT2-8은 동일 입력의 optimized/reference 실행과 별도 AMR 오차를 비교한다.

각 witness는 R0 표현 가능→R1 손작성 실행→R2 비용을 포함한 이익→R3 미주입 자연 발견→R4 전달·재사용·누적의 단계를 구별한다. 학습·변이 연산자·decoder를 직접 작성해 성공시킨 fixture를 R3로 올리지 않는다. 해당 메커니즘을 가진 공개 seed를 사용했다면 seeded 실험으로 별도 보고한다.

**경계:** Engineering Contract는 기대값과 증거 단계다. W/P의 구현을 검증한다. E는 실제 run에서 확인한다. U는 미실행·통과·자연 발생·재현 실패를 별개 상태로 표시한다.

## 120 개발 순서·실패 대응·현재 완료 상태

**R10 적용 안내: 이 절은 이전 ruleset의 기반·회귀 계약이다. R9의 원소·graph·VM·신경/학습은108..120절, 핵·양자·법칙장·세계 확장과 최적화의 최신 교체는121..138절이 우선한다. 현재 실행·자료형은135절, 비용·완료 상태는129·136·138절을 따른다.**

### 구현 순서와 의존성

첫 단계는 CT2의 원소 ledger, wide VM, paged record, shared CU, copy-noise stream과 저장 형식을 작은 fixture에서 고정하는 일이다. neural scalar kernel·epoch 중단/복원도 이 단계에 포함한다. 물리 계수 검증 전에 큰 세계를 실행하지 않는다. 기존 Grid-5·CT1 회귀는 별도 backend로 유지한다.

두 번째 단계는 source→operator→offspring draft, germline/somatic/culture 분리, 실제 backing과 code/weight adoption을 결합하는 일이다. 학습기 fixture와 스스로 생긴 학습을 구별하는 UI·기록을 함께 만든다. code·기억·graph 길이를 늘려도 host 부족이 생물학적 실패로 변하지 않는지 확인한다.

세 번째 단계는 작은 CT2-Reference에 R8 화학·기계·파동·환경을 결합하고 Z/W-CT2 및 기존 적용 사례를 실행하는 일이다. reference가 안정된 후에 정확 최적화를 적용한다. AMR·homogenization은 독립적인 B gate를 통과한 뒤 별도 numerical profile로 추가한다. 물리 오차와 생태 정체를 단순히 “진화가 못 찾는다”로 묶지 않는다.

네 번째 단계는 headless 장기 실행, save/replay, 실제 2D viewer, lineage·기술·학습·변이 연산자 계보를 통합하는 일이다. Far/Mid/Close의 세계 표현은 기존 구조를 유지하고 신경망은 선택 agent의 상세 패널에서만 펼친다. 모든 agent의 edge를 매 frame 그리지 않는다. 관찰자가 원하면 genome 초기값·학습 후 state·전달 record를 나란히 볼 수 있다.

v0은 최소 생태·VM·자원 흐름의 회귀 기반, v1은 구조·전달·집단, v2-Continuum은 이번 CT2와 R8의 통합 목표다. 개발 profile에서 기능을 끄고 시험할 수 있지만 최종 run 도중 기술 발전에 따라 새 법칙을 켜지 않는다. v3 후보였던 신경·생애학습·변이 연산자는 이번 요청에 따라 CT2로 이동했다. 진짜3D·핵물리·양자화학·거대 모델의 외부 사전학습은 현재 범위에 추가하지 않는다.

### 추가 실패 모드와 대응

| 실패 | 예방·판정 | trade-off |
| --- | --- | --- |
| 원소가 늘었지만 실제 조합은 무의미 | state/에너지·중간 기능 witness와 유효 niche 확인 | 더 많은 물성 실험, 원소 수 자체는 성공 지표 아님 |
| 큰 graph가 host 비용 때문에 도태 | host 부족은 halt, 실제 크기 비용만 세계 규칙 | run이 느려지거나 멈출 수 있음 |
| graph hash 증가를 화학 혁신으로 오인 | 실제 구조·반응·성질·재사용 증거 | 분석 비용 증가 |
| 큰 code·NN가 비용 없이 지배 | backing·compute·복사·staging·열 정산 | 복잡성이 실제 이득 없으면 사라질 수 있음 |
| NN 계산이 너무 느려 행동 못 함 | per-epoch CU와 지연 공개, 선택적 작은 망·유전된 scheduling | 신경망 우세를 보장하지 않음 |
| 학습이 무료 fitness optimizer가 됨 | 현지 관측만 입력, 유전된 절차, 학습·검사 비용 | 학습이 나타나지 않거나 불안정할 수 있음 |
| 배운 내용을 모든 후손에게 자동 상속 | germline/somatic/record 소유권·실제 copy 분리 | 누적 문화와 유전이 느려질 수 있음 |
| 학습 중 weight 공유로 다른 개체가 즉시 배움 | immutable topology만 공유, mutable state 분리 | 더 많은 RAM |
| mutation operator가 무료 자식 탐색 수행 | 미래 fitness oracle 없음, 실제 draft·CU·원료 비용 | 연산자 진화가 단순 복사보다 불리할 수 있음 |
| 변이율이0에 수렴 | 자연 선택 결과로 기록; 환경·trade-off를 별도 실험 | 다양성을 자동 보장할 수 없음 |
| 자기 수정 code가 replay를 깨뜨림 | revision pin·adoption boundary·진행 state 저장 | record 수명·메모리 관리 복잡 |
| AMR 오차가 발명으로 악용됨 | reflection·Gauss·에너지·수렴 witness, profile 표시 | 필요한 해상도가 올라가 이점 감소 |
| 기능을 모두 켠 뒤 성능 붕괴 | 단계별 reference·phase profiler·host memory gate | 초기 세계를 작게 실행 |
| 높은 자유도가 탐색을 더 어렵게 함 | 유용한 짧은 경로 R1/R2 검증 후 장기 multi-seed 관찰 | 자유도와 발견 가능성을 따로 연구 |

### 다양성과 연구 평가

신경 node 수·code 길이·원소 수가 큰 계통에 보상을 주지 않는다. 지리적 격리·유한 자원·서로 다른 수송/열/빛 조건·공진화는 기존 선택압으로 유지한다. observer는 실제 행동·물질 사용·기록 전달·신경/VM trace의 표본에서 새로운 기능군을 탐지한다. raw neural weight distance만으로 새 기술을 선언하지 않는다.

같은 compute budget의 VM-only, VM+neural 비가소성, VM+learning, evolvable operator 유무를 명시적 비교 실험으로 둘 수 있다. 비교 profile은 개입이므로 각자 새 RunManifest를 가진다. seed·관찰 기간·모든 멸종/정체 결과를 기록하고 잘된 run만 남겨 성공률을 과장하지 않는다. 장기 학습이 적응을 도왔는지와 문명 기술이 누적되었는지는 별도 질문이다.

### 설계로 정한 것과 아직 필요한 것

이번 개정은 여섯 제약에 대한 선택, 신경·학습의 최신 포함 범위, 물질·기억·계산 비용, 유전/학습/문화 소유권, 실행·저장·최적화·검증 계약을 구체화했다. 아직 시뮬레이터 소스는 작성하지 않았다. 문서를 길게 썼다는 사실이 물리 solver와 데이터가 완료되었다는 증거는 아니다.

| 남은 입력·증거 | 완료되어야 하는 내용 |
| --- | --- |
| ChemistryData-CT2 |16원소 상태·장벽·수송·조립·상평형 자료와 closed-cycle 검사; bulk/instance 표준 상태 대응 |
| Memory/Compute/CopyNoiseData | 실제 backing 용량·생성/유지/수리·32 CU scaffold·오류율의 단위와 계수 |
| CT2 wire/validation fixtures | wide record descriptor·service payload·fault 숫자·신경 descriptor·save directory의 정확한 bytes와 golden vector |
| R8 물리 미동결 항목 | moving material+Maxwell, energy-compatible remap, 환경 pressure work, passive constitutive 데이터 |
| AMR DiscretizationSpec | 보존형 interface operator·안정성·반사/위상·수렴·적응 시 에너지 회계 |
| Implementation evidence | Z01..Z64·W-CT2·legacy 회귀·deterministic replay의 실제 결과 |
| Performance evidence | stride·peak RAM·phase time·NN/chem/field workload별 TPS·장기 저장 비용 |
| Emergence evidence | 미주입 R3/R4, 학습·변이 방식의 실제 진화, 전파·소실·재발견·장기 변화 |

따라서 현재 상태는 **R9 확장 설계와 검증 계약 작성 완료, 실행·물성·수치 결합·성능·자연 창발 검증 전**이다. “이제 구현만 하면 수정 없이 끝난다”, “무한 기술 발전을 보장한다”, “AAA 게임 수준의 상호작용이 자동으로 나온다”는 판정은 하지 않는다. 불변 세계 규칙·원자적인 조작·비용·검증은 구현할 수 있지만 어떤 문명이 발견될지는 실험으로 확인해야 한다.

**경계:** Engineering Contract는 개발 순서·실패·완료 조건이다. W/P는 추가한 범용 기초다. E는 가능한 것을 실제 역사로 만든 결과다. U는 설계·구현·자연 발견·누적의 차이를 유지한다.

## 121 R10 적용 범위와 다섯 제약의 교체

R10의 목적은 물리적 표현 공간을 확장하면서 실제 계산 가능한 엔진 계약을 만드는 것이다. 신경망·생애학습·형태·유전 연산자·외부 기록은 R9대로 포함한다. 새 문명 행동 opcode는 추가하지 않는다. 핵반응과 양자화학이 있다고 해서 생물이 그것을 이용할 수 있거나 진화가 발견한다는 뜻은 아니다. 접근 가능한 중간 단계와 비용을 실제 세계에서 검증해야 한다.

현재 ruleset은 civ-continuum-3-draft, 저장 식별자는 CIVCT003이다. CT2의 VM 명령 encoding은 유지하되 world ABI와 저장 schema는 CT3로 분리한다. 121..138절은 108..120절과 충돌하는 항목에 우선한다. 추가 Q01..Q80은 실행 전 기대 계약이며 총405개 사례에 포함한다.

| 이전 제한 | R10 추천 결정 | 남는 한계와 비용 |
| --- | --- | --- |
| 핵반응 없음 | 동위원소·핵반응 channel·방사선 수송·질량 결손 회계 추가 | 유한 데이터와 유효 에너지 범위; 전 세계 원자별 추적 아님 |
| 양자화학 없음 | 국소 전자구조 solver와 보존형 embedding 추가 | 근사 전자구조이며 전 세계 many-body wavefunction 아님 |
| 물리법칙 자체 고정 | 기본 profile은 고정; 별도 LawField 실험에서 유효 계수 공진화 | 고정 meta-law는 반드시 남음; 현실의 법칙 진화라는 주장 없음 |
| 유한 컴퓨터 | 계층 저장·정확한 캐시·이종 병렬·분산·실험 병렬 | 시간·RAM·VRAM·디스크·주소는 여전히 유한 |
| 유한 세계 | 넓은 좌표의 희소 청크와 엄격한 미관측 영역 시간 계약 | 동시에 실현·진행되는 세계는 유한; 진짜 무한 계산 없음 |

### 세 종류의 profile

Base-CT3는 핵·국소 양자·생명·파동을 결합하는 목표 규칙이다. LawField-CT3는 126절의 가상 동역학까지 포함한 별도 연구 세계다. Bench-CT3는 성능 원인을 분리하는 시험 구성이다. Bench에서 화학이나 파동을 끈 TPS를 Base의 대규모 통합 성능으로 표시하지 않는다. Capability를 실행 중 켜서 자연 발명으로 취급하지 않는다.

ReferenceBackend는 작은 세계의 판정 기준이다. ProductionBackend는 같은 계약을 빠르게 실행하는 구현이다. A는 동일한 규칙·연산 결과·세계 계산비용을 보존하는 최적화, B는 허용 오차와 discretization이 달라지는 수치 근사, C는 세계 법칙 또는 경계 조건의 변경이다. B나 C도 쓸 수 있지만 A라고 부르지 않는다. 시각적 결과가 비슷하다는 이유로 동일한 역사라고 하지 않는다.

### 기존 불변 조건의 정확한 수정

I01-CT3: 화학에서는 원소·동위원소의 핵 수를 보존한다. 핵반응에서는 원소별 수를 보존하지 않으며, 채택한 반응군의 핵자 수와 총 전하를 입자·매질·경계 전체에서 보존한다. 중성미자 등 출구도 사라짐이 아니라 경계 장부다. 핵자-반핵자 생성 등의 더 넓은 고에너지 입자계는 현재 유효 범위 밖이다.

I02-CT3: 총에너지는 정지에너지 기준 차이, 전자·화학·여기 에너지, 운동·중력·탄성·열, 전자기장·복사·핵 입자, 선택적 LawField와 외부 경계 항을 포함한다. 핵의 Q값, 전자구조 에너지와 기존 반응열을 중복 계상하지 않는다. 기존 정수 에너지 q와 새 고정밀 ledger 사이의 변환 잔여도 저장한다.

I07-CT3: 호스트 thread·캐시 hit·GPU 완료 순서·페이지 fault·카메라·저장 속도는 같은 BackendManifest 안에서 미래 상태를 바꾸지 않는다. 서로 다른 부동소수점 backend의 bitwise 동일성은 별도 시험을 통과한 경우에만 주장한다. I10은 완결 tick 체크포인트를 유지하며 tick 내부 solver stage는 rollback용 일시 상태다. 나머지 I03..I12는 유지한다.

**경계:** W는 보존·국소 상호작용·선택한 수치 모델이다. P는 기존 감각·작업·배치·기록·계산이다. E는 새로운 에너지 이용·재료·환경과 기술의 관계다. U는 핵/양자 현상과 오차·증거 수준 표시다.

## 122 동위원소·핵반응·질량 결손

### 표현 선택

| 방법 | 장점 | 단점·성능·복잡도 | 개방성 영향 |
| --- | --- | --- | --- |
| 이름 붙은 핵연료와 발전 recipe | 매우 저렴하고 쉽게 디버깅 | 장치·에너지 이용법을 미리 정함 | 목표와 맞지 않아 제외 |
| 모든 핵·입자 개별 사건 | 작은 계의 의미와 통계가 분명 | 물질량에 비례한 사건 수; 대형 세계 불가 | reference에 유용하나 규모 제한 |
| 동위원소 재고·반응 channel·입자 수송 결합 | 물질 변환과 에너지 경로를 일반적으로 연결 | 데이터·단위·다중 시간척도 검증 필요 | 추천; 장치 종류를 정하지 않고 물리 공간 확장 |

추천안은 핵종을 물질 구성의 또 하나의 층으로 표현한다. ElementDef는 원자번호와 화학 특성을, NuclideDef는 Z·N·준안정 여기 상태와 정지에너지·붕괴 channel을 가진다. 같은 원소의 동위원소는 별도 재고다. 분자 graph의 원자 node는 필요할 때 isotope key를 가지며 bulk는 정렬된 희소 동위원소 조성 벡터를 사용한다. 모든 tile에 핵종 전체 배열을 두지 않는다.

NuclideRegistry는 실행 시작에 고정된다. 처음 세계에 존재하는 원소·핵종의 수와 registry 지원 범위는 다르다. 활성 반응의 모든 도달 가능한 생성물·붕괴 딸핵·전자/입자 carrier를 닫힘 검사에 포함한다. 모르는 핵종에 임의 물성을 생성하지 않는다. 일부 채널이 비어 있으면 해당 데이터 영역을 지원한다고 표시하지 않는다.

IAEA LiveChart와 Geant4의 붕괴 문서는 핵종·붕괴 데이터와 반응별 모델이 필요함을 확인하는 참조다. 외부 문서를 runtime에 조회하지 않는다. 승인한 데이터 묶음의 원본·버전·단위·변환 규칙·지원 범위·hash를 DataManifest에 저장한다. 데이터의 불확실성은 세계가 바뀌는 숨은 난수가 아니라 실험 manifest의 일부다. [IAEA API](https://www-nds.iaea.org/relnsd/vcharthtml/api_v0_guide.html), [Geant4 radioactive decay](https://geant4.web.cern.ch/documentation/dev/prm_html/PhysicsReferenceManual/hadronic/Radioactive/radDecay.html)

### 정지에너지 규약

기준은 bare nucleus의 정지에너지와 전자·양전자 정지에너지를 분리하는 convention이다. 원자 질량 표를 받아들일 때는 같은 전자 결합 에너지 기준으로 변환한다. neutral atom mass를 그대로 더한 뒤 자유 전자 질량을 다시 더하지 않는다. 전자 포획은 실제 전자를 소비하며, 베타 반응은 channel에 맞는 전자·양전자·중성미자와 recoil을 생성한다.

핵반응의 Q는 같은 에너지 기준에서 E_rest,in - E_rest,out - ΔE_excitation으로 계산한다. Q를 열에 모두 넣은 다음 동일 에너지의 방사선도 발사하지 않는다. channel의 운동학 규칙으로 생성 입자·recoil·여기 상태에 분배하고, 흡수·충돌 때 열이나 이온화로 넘어간다. 음의 Q를 가진 반응은 입사 에너지나 해당 열적 분포에 접근할 수 있어야 한다.

큰 정지에너지끼리 f64로 빼서 작은 반응열을 얻지 않는다. 보존된 핵자·전하 기반의 공통 기준항을 빼고 저장하는 signed fixed-point rest-energy offset과 checked i128 지역 누계, checked wide 전역 누계를 사용한다. 수치 폭·양자 단위는 UnitSpec 데이터 gate에서 정한다. overflow는 wrapping이나 에너지 포화가 아니라 RunHalt다. 질량 결손은 ‘원소 수가 줄어서 재료가 사라지는 버그’가 아니다.

R9의 C=12, Fe=56 같은 정수 명목 질량은 핵종 질량이 아니다. CT3에서 그것을 c²와 곱해 핵 에너지를 계산하는 경로는 금지한다. 화학적 질량 근사와 실제 데이터 기반 핵 Q를 섞는 경우 별도 effective-model mass convention을 명시하고 오차를 검증한다. 기본 목표는 동일 NuclideDef에서 관성·중력 질량과 에너지 차이를 유도하는 것이다.

### 사건과 화학의 연결

NuclearChannel은 입사 종류·핵종·에너지 범위, 반응물/생성물 계수, 확률률 또는 단면적, 최종 상태 sampler의 식별자다. ‘발전’, ‘발명’, ‘장치 완성’ 상태는 없다. 열·입자 노출·물질 배치가 channel의 실제 조건을 바꾼다. fusion/fission도 법칙 데이터의 반응군이지 agent 명령이 아니다.

핵종의 Z가 바뀌면 화학 graph의 원자명만 교체하고 기존 valence를 유지할 수 없다. 반응 결과는 recoil·전하·국소 여기 상태와 함께 해당 화학 영역을 dirty로 만들고 125절의 전자/결합 재평가를 요구한다. 방사선의 기록 손상은 같은 물리적 침적 사건으로부터 발생한다. 별도의 ‘방사선이 진화를 돕는 보너스’를 넣지 않는다.

### 단위와 모델 유효 범위

R8의 c=32 tile/tick은 저속 인공 세계의 파동 수송 설정이다. 이를 검증 없이 핵 질량-에너지 환산의 c²로 쓰지 않는다. UnitSpec은 tile 길이·tick 시간·에너지·질량·전하의 환산과 물리 정지에너지 상수를 하나로 정의한다. reduced-speed transport를 유지한다면 그것은 비상대론적 유효 모델이며 완전한 상대론 세계가 아니다.

핵 입자의 상대론적 에너지-운동량은 승인한 입자 수송 모델에 한정하고, 느린 물질 solver에 전달하는 힘·열·recoil의 유효 범위를 검증한다. 물질 속도·압력·온도가 지원 범위를 넘으면 더 높은 유효 모델로 검증된 전환을 하거나 ModelDomainExceeded로 정지한다. 안정성을 위해 속도를 조용히 잘라내고 정상 계산으로 보고하지 않는다.

**경계:** W는 핵종·반응률·운동학·보존이다. P는 물질 이동·열·국소 힘과 감각이다. E는 에너지 이용·변성·차폐·손상에 대한 적응이다. U는 핵종·침적·에너지 변환 후보를 설명한다.

## 123 핵반응의 수송·시간척도·통계적 비용

### 두 해상도의 명시적 관계

작은 검증계에서는 개별 사건의 event-driven reference를 사용한다. 지역 반응 hazard, 누적 optical depth, 다음 사건 번호를 저장한다. 수송 경로를 청크로 잘라도 동일한 물질 경로라면 random draw의 의미가 바뀌지 않아야 한다. RNG 주소는 emission UID·track UID·collision ordinal·purpose이고 worker·GPU block·재시도 횟수가 아니다.

대규모 추천 backend는 동위원소 cohort와 가중 transport packet, 국소 depletion 적분을 결합한다. 이것은 B 근사다. cohort의 핵 수 w, packet의 대표 수, 수송 energy group, 각도 분해능, 적분 허용 오차를 DiscretizationSpec에 기록한다. w=1 reference와 비교해 평균뿐 아니라 변동·상관·희귀 손상·자기증폭 거동이 유지되는 범위를 확인한다. 큰 w는 사건 수만 줄이는 무해한 압축이 아니다.

cohort의 분할은 정수 핵 수를 보존한다. 반응 extent와 packet 생성 수의 잔여는 사라지지 않는 지역 residual로 누적한다. 핵반응 생성물과 에너지 장부는 같은 extent를 공유한다. 평균 반응률을 사용하면서 출생·손상에는 독립적인 임의 사건 수를 사용하지 않는다. 생산 backend의 time-leap는 음의 재고를 만들면 reject 후 고정 순서로 더 작은 dt에 재시도한다.

### 수송 공간과 에너지 sector

세계는 계속 2D slab이다. 입자는 평면 위치와 내부 방향 성분을 가질 수 있으나 slab 위/아래로 나간 에너지는 boundary escape다. 2D 원형 확산을 3D 구면 확산이라고 표시하지 않는다. 두께·입사 분포·out-of-plane 이탈 규칙을 저장하며, 관통·산란은 물질 경로와 실제 밀도를 사용한다.

기존 W coherent Maxwell, R 비간섭 광학 복사, 새 H 고에너지 입자/광자 sector는 단일 에너지 소유권을 가진다. 하나의 photon을 W와 H에 동시에 저장하지 않는다. H의 흡수·down-conversion으로 R·열·전자 여기 상태가 생길 때 source debit과 destination credit을 하나의 transaction으로 기록한다. 낮은 에너지로 내려갔다는 이유만으로 임의 삭제하지 않는다.

중성미자는 기본 모델에서 상호작용을 무시하는 탈출 carrier로 둘 수 있다. 이는 명시적 C 모델 선택이며 즉시 열로 바꾸지 않는다. 양전자 소멸이나 전자 이온화 등을 지원한다고 선언한 profile은 관련 carrier와 endpoint 회계까지 포함해야 한다. 구현하지 않은 반응을 이름만 registry에 넣어 지원으로 세지 않는다.

### event 폭증 제어

| 방법 | 절약 효과 | 보존·성능 위험 | 채택 조건 |
| --- | --- | --- | --- |
| 안정 핵종 event 비생성 | 필요 없는 스케줄 제거 | 안정성 데이터 오류에 민감 | 같은 규칙에서 hazard=0이면 A |
| next-event calendar | 매 tick 전량 스캔 감소 | hazard 변화 때 재계산 필요 | frozen 구간 또는 정확한 누적 hazard |
| cohort·그룹 수송 | 사건·track 수를 크게 줄일 가능성 | noise와 스펙트럼·공간 상관 변화 | B, 수렴 및 희귀 효과 gate |
| packet 합치기 | 메모리와 큐 감소 | 서로 다른 방향·에너지 평균화 | 동일 상태의 lossless merge만 A |
| 낮은 에너지 track 종료 | 매우 빠름 | 멀리 가는 신호·화학 손상을 놓침 | 승인된 local deposition 근사일 때만 B |

물리 사건 수가 예산을 넘으면 다음 사건을 다음 tick으로 몰래 미루지 않는다. host queue를 늘리거나 spill하고 wall time을 늘린다. 불가능하면 tick rollback 후 ResourceLimit로 멈춘다. 발생한 에너지나 손상을 버리지 않는다. 높은 반응 활성도는 실제로 엔진의 최악 부하다.

공통 global substep을 가장 빠른 핵 사건에 맞추지 않는다. 핵 수송은 자체 event time을 사용하고 접촉하는 물질·전하·열 solver와 정해진 교환 시점에서 보존량을 주고받는다. 지연된 열·전하 전달의 오차가 coupling tolerance를 넘으면 그 영역의 교환 간격을 줄인다. 1 tick 끝에 모든 방사선 열을 몰아 넣는 방식은 coupled reference와 동등하지 않다.

**경계:** W는 시간·확률·수송·경계 조건이다. P는 추가하지 않는다. E는 노출에 따른 재료·생태·에너지 경로다. U는 평균 침적과 통계 오차를 함께 표시한다.

## 124 국소 양자화학의 계산 영역

### 모델 비교와 추천

| 방법 | 장점 | 단점·성능·복잡도 | 개방성 영향 |
| --- | --- | --- | --- |
| 전 세계 정확한 전자 파동함수 | 원리상 넓은 상호작용 표현 | 목표 규모에서 계산 불가능; 핵·환경과 더 복잡 | 채택하지 않음 |
| 사전 양자 계산의 고정 물성표 | 실행이 매우 저렴 | 표 밖 구조의 물성을 알지 못함 | bulk 기준 모델로 유지 |
| 국소 semiempirical 전자구조와 embedding | 새 국소 구조의 전하·결합·에너지 재평가 가능 | parameter domain·SCF·경계·반응 경로 검증 필요 | 추천; graph 문법만보다 넓으나 보편 화학은 아님 |

CT3 QuantumBackend의 첫 후보는 xTB 계열의 검증된 C API adapter다. DFTB+는 독립 비교용 backend 후보로 남긴다. 외부 계산기는 전체 세계 소유자가 아니며 frozen patch input을 받아 energy·gradient·전자상태·진단을 반환한다. xTB 문서는 semiempirical 전자구조와 C interface를 제공하지만, API가 원자번호를 받는 것과 선택한 method의 모든 원소·상태 지원은 다르다. [xTB 문서](https://xtb-docs.readthedocs.io/en/latest/), [C API](https://xtb-docs.readthedocs.io/en/latest/capi.html), [DFTB+](https://dftbplus.org/)

### QuantumPatchSpec

patch는 actual atom UID 목록, 3성분 내부 좌표, isotope 질량, 총 전하·전자 수·spin sector, embedding charges와 경계, method/parameter hash, solver precision, input revision으로 정의한다. 초기 trial density나 restart state가 수렴 가지 선택에 영향을 주면 그것도 authoritative state다. output은 energy·forces·전하 분포·수렴 잔차와 method validity다. ‘촉매 기술 발견’ 같은 의미 label은 포함하지 않는다.

원자의 내부 3D 좌표는 입체 화학을 위한 미시 상태다. macro body가 2D slab 밖으로 자유롭게 이동하거나 다른 물체를 통과하게 해 주지 않는다. 내부 원자도 기존 재고와 부피·질량·열·결합 장부에 속한다. 계산 경계의 link atom은 수학적 보조 변수이지 채집할 수 있는 수소가 아니다.

첫 reference benchmark는 16·32·64개 원자와 여러 전하·spin 상태의 작은 patch로 시작한다. 이것은 검증 workload이지 ‘65번째 원자는 반응 불가’라는 세계 법칙이 아니다. 큰 반응 중심이 필요한 경우 patch 확대·검증된 coarse closure·지원 범위 초과 정지 중 명시된 경로를 따른다. 무제한 patch가 유한 RAM에서 항상 가능하다고 약속하지 않는다.

### 언제 계산하는가

국소 결합 변동, 새로운 화학 구조, 강한 전하·여기·변형, 핵종 전환 등의 **물리적 오차 지표**가 reevaluation을 요구한다. 개체가 쳐다본 곳이나 UI가 흥미롭다고 고른 곳을 계산하지 않는다. 같은 상태에서 같은 trigger와 solver input이 만들어져야 한다.

전자의 ground-state energy를 얻는 것만으로 실제 반응 속도와 장벽이 결정되지 않는다. 최소점 사이의 전이 경로·활성화 장벽·반응 좌표와 온도 의존률은 검증된 kinetics 모델이 필요하다. graph rewrite 후보는 가능한 국소 변환을 열거할 뿐, 에너지가 낮다는 이유만으로 매 tick 즉시 변환하지 않는다. 양자 tunneling과 광여기 반응을 지원하려면 각 근사와 데이터 gate를 별도로 통과한다.

모든 미지 분자에 신뢰할 만한 물성을 제공한다고 선언하지 않는다. support mask는 원소·전하·spin·온도·변형·배위 환경을 포함한다. validity 밖은 physics error다. 예상치 못한 구조라는 이유로 새 화학을 조용히 기존 물질로 치환하거나 분해해 탐색 공간을 닫지 않는다.

### 계산비용의 주체

QuantumBackend를 실행하는 host FLOP은 세계 물리를 계산하는 비용이다. 생물의 VM CU에서 빼지 않는다. 반대로 agent가 전자구조 해답에 직접 접근하지도 못한다. 실제 감각·측정·계산을 통해 알아내야 한다. 외부 solver가 오래 걸리면 wall time만 늘어난다. GPU가 있는 컴퓨터의 생물이 더 싼 화학 측정 능력을 받지 않는다.

**경계:** W는 선택한 양자 근사·embedding·반응 kinetics다. P는 기존 국소 감각·물질·에너지 조작이다. E는 재료·촉매·생체 구조와 기술의 새로운 조합이다. U는 지원 범위·전자구조·반응 증거를 보여 준다.

## 125 양자·고전·열·핵의 결합 계약

### 에너지의 한 명세

기본 결합식은 E_total = E_low(whole) + E_QM(patch) - E_low(patch)다. 실제 구현은 embedding과 경계 항까지 같은 분해로 정의해야 한다. QM 에너지를 추가하고 기존 bond energy를 남겨 두지 않는다. xTB ONIOM 문서의 subtractive 구성은 참고할 수 있으나 CT3의 보존성과 moving boundary까지 자동 해결해 주지는 않는다. [xTB ONIOM](https://xtb-docs.readthedocs.io/en/latest/oniom.html)

force는 같은 E_total의 좌표 미분에서 나온다. patch 전환의 weight를 쓰면 weight의 위치 의존 미분도 포함한다. 전자 charge 재분포는 실제 전하 수송·경계 항에 연결한다. 열로 바꿀 electronic relaxation energy는 다른 energy channel에서 debit한다. 수치 residual을 물리적 반응열이라고 해석하지 않는다.

초기 구현은 한 원자가 동시에 두 독립 patch에 속하지 않도록 한다. 상호작용하는 patch는 canonical UID 순서로 합치거나 하나의 검증된 coupling solve에 포함한다. 겹친 atom의 force·heat를 두 번 적용하지 않는다. 자유도가 증가하면서 patch가 거대해질 수 있는 최악 비용을 인정한다.

### 영역 전환

low→QM 전환은 같은 원자·좌표·전자 수·운동량에서 시작한다. 에너지 zero point를 맞추고 representation 변환 residual을 저장한다. 임의의 에너지 차를 주변 열에서 몰래 빼거나 더하지 않는다. 두 모델이 일치하지 않으면 기준 모델을 유지한 채 확장 계산을 하거나 transition gate가 통과할 때까지 그 생산 경로를 비활성으로 둔다.

QM→low도 구조·charge·에너지·force 근사 오차를 확인한 후 수행한다. 새로운 구조를 기존 coarse species로 합칠 때 실제 열역학적 state 수와 혼합 entropy를 보존해야 한다. R9의 bulk↔instance 변환 미동결 항목은 여전히 선행 gate다. 호스트 메모리가 부족해서 반응할 수 없는 coarse 물질로 바꾸지 않는다.

### solver 완료와 실패

QM input은 stage revision에 고정한다. jobs는 병렬 완료 가능하지만 commit 순서는 patch UID·stage·request ordinal로 정한다. 오래 걸리는 계산을 다음 tick의 물리 결과보다 뒤에 적용하지 않는다. 외부 solver timeout은 생태 사건이 아니며 Runner 진단 정지 사유다.

SCF는 고정된 convergence 조건과 retry sequence를 사용한다. 실패 시 guess·damping·정밀도·iteration budget을 명시된 순서로 바꾸며 각 결과와 residual을 기록한다. 모든 retry 실패는 StageFailure다. 수렴하지 않은 낮은 energy 한 번을 ‘발명 가능한 물질’로 채택하지 않는다. 가장 빠른 worker의 답을 선택하지 않는다.

같은 입력에서 매번 cold start를 쓰면 정확한 결과 캐시를 만들기 쉽지만 계산이 느리다. previous density를 이어 쓰면 빨라질 수 있으나 그 상태를 checkpoint와 hash에 포함한다. 임의 캐시 hit가 SCF 가지를 바꾸면 A 최적화가 아니다. geometry 반올림·유사 구조 캐시·학습 surrogate는 B 또는 C로 분리한다.

### 결합 검증과 생태 해석

작은 물질 집합의 정적 에너지·force finite difference, 무외력 운동의 drift, 반응 전후 에너지·charge, 영역 왕복 변환, 방사선 이온화 후 재결합을 기준 fixture로 둔다. isotope substitution의 화학적 영향과 핵 변화의 영향은 서로 다른 시험이다. ground state solver만으로 광합성·의식·지능을 재현했다고 주장하지 않는다.

직접 만든 촉매나 신호 회로의 witness는 R1 가능성 증거다. 해당 구조가 자원·수명·복제에서 이득을 주는 R2, 자연 계통에서 나타나는 R3, 실제 전달·재조합·소실을 보이는 R4를 별도로 기록한다. quantum이라는 이름이 이 증거 단계를 건너뛰게 하지 않는다.

**경계:** W는 에너지·charge coupling과 검증된 모델 전환이다. P는 새 의미 행동을 추가하지 않는다. E는 재료 기능과 생태 적응이다. U는 물리 진단과 기술 후보의 증거를 분리한다.

## 126 물리법칙 진화의 가능한 의미

### 세 대안

| 방식 | 장점 | 단점·성능·복잡도 | 추천 판단 |
| --- | --- | --- | --- |
| 상태에 따른 물성 변화 | 현실과 가까운 온도·압력·조성 효과 | 기본 법칙은 그대로 | Base에서 반드시 유지 |
| 고정 meta-law 아래 동적 계수장 | 지역적으로 다른 유효 상호작용과 niche | 새 가상 물리, 추가 에너지·안정성 검증 | LawField 별도 실험으로 선택 |
| 임의 native law·상수·opcode 재작성 | 겉보기 표현력 큼 | 보존·재현·성능·의미가 무너질 수 있음 | authoritative world 안에서는 제외 |

무언가가 변하려면 ‘무엇이 어떻게 바뀌는지’의 상위 규칙이 필요하다. 그 상위 규칙까지 매번 바꿔도 다시 더 높은 기준이 남는다. 따라서 모든 법칙의 무제약 진화를 완성했다고 할 수 없다. 이 제한을 숨기는 대신, 일관되게 변할 수 있는 유효 물리의 폭을 넓힌다.

### LawField의 구체적 형태

각 활성 영역에 χ와 conjugate momentum πχ를 둔다. 총 에너지는 H_matter(x;χ) + Σ[πχ²/(2Mχ) + Kχ|∇χ|²/2 + V(χ)]로 정의한다. matter의 힘에는 χ에 의존한 potential을 쓰며 χ의 운동에는 -∂H_matter/∂χ가 되돌아간다. 에너지 전달의 양방향 항이 빠지면 물질이 공짜로 일을 하게 된다.

첫 실험은 하나의 국소 탄성/상호작용 계수만 k(χ)=k_min+(k_max-k_min)s(χ)로 바꾸며 s는 고정된 bounded smooth 함수다. V·Mχ·Kχ·s·허용 범위·경계는 시작 manifest에 고정한다. damping을 넣으면 감소한 field energy가 지정된 열 reservoir에 들어간다. χ를 random하게 바꾸고 energy를 맞추는 방식은 제외한다.

더 많은 계수장으로 확장하려면 안정한 constitutive matrix·passivity·열역학 관계를 유지하는 parameterization이 필요하다. 핵 질량·전자 전하·h_bar(플랑크 상수/2π)·c를 바꾸면서 이전 핵 데이터와 전자구조 parameter를 그대로 쓰지 않는다. 이 경우는 기존 물성의 보간이 아니라 새로운 이론·데이터 검증 과제다.

### 진화와 조작의 경계

agent는 SetLaw, SetGravity, RewritePhysics를 호출하지 않는다. 기존 물질·힘·열·전하의 상호작용이 χ와 결합할 때만 영향을 줄 수 있다. 어떤 생물이 주변 χ의 상태를 유지하고 그 환경에 의존하는 절차를 유전한다면 유효 법칙과 생태의 공진화로 분석할 수 있다. 그것이 가능하도록 집단 보너스나 환경 유지 장치를 자동 배치하지 않는다.

실행 간 PhysicsSpec를 변이시키는 universe ensemble은 별도 연구 기능이다. 부모 RunManifest·변경값·초기 조건·분기 이유를 기록하고 서로 다른 세계로 취급한다. 특정 법칙이 문명을 잘 만든다고 자동 선택해 기존 세계에 주입하지 않는다. LawField도 그 효과가 없거나 모든 생태를 붕괴시킬 수 있으며 결과를 숨기지 않는다.

**경계:** W는 변하지 않는 meta-law와 계수장 동역학이다. P는 기존 물리적 상호작용이다. E는 새로운 재료·환경 유지·적응 관계다. U는 ‘유효 물리 변화’와 ‘기본 법칙 변경 실험’을 명확히 구분한다.

## 127 넓은 세계·희소 공간·미관측 영역의 시간

### 공간 저장 선택

| 구조 | 장점 | 단점·성능·복잡도 | 선택 |
| --- | --- | --- | --- |
| 고정 dense 직사각형 | 주소·경계·보존 검사 단순 | 넓은 빈 영역도 메모리 사용 | reference와 밀집 benchmark |
| sparse directory + dense chunks | 넓은 주소 공간과 지역성 | 미생성 영역의 물리 시간을 처리해야 함 | 생산 세계 추천 |
| 끝없는 procedural terrain만 생성 | 시각적으로 쉽게 확장 | 방문 전 화학·기후·파동의 역사가 사라짐 | 실제 세계 시뮬레이션으로 채택하지 않음 |

WorldCoord는 signed i64 chunk 좌표와 u8 local 좌표 0..63이다. 변위·이웃 계산은 checked wide intermediate를 사용한다. 디렉터리는 호스트 hash 구조여도 iteration이 물리 순서가 되지 않는다. UID는 좌표·생성 origin·event ordinal에서 안정적으로 유도하고 충돌 검출을 둔다. 포인터나 메모리 주소는 저장하지 않는다.

물리적으로 실현된 영역은 항상 유한하다. 주소 공간이 넓다고 모든 좌표에 물질이 존재하고 그 동역학이 계산되었다고 주장하지 않는다. ‘무한 세계’ 대신 ‘확장 가능한 세계’라고 표시한다. 최대 materialized domain과 저장 한계를 RunManifest와 UI에 공개한다.

### 절차 생성의 보존 장부

InitialField(seed, coordinate)는 세계 시작 시점의 초기조건을 정의한다. 새 영역을 실현할 때의 재고는 InitialDomainRealization 항목이다. 현재 영역의 내부 반응이 새 재료를 만든 것으로 세지 않는다. 지역 control-volume 장부는 실현 시 초기 inventory와 이후 경계 flux를 분리한다. 무한한 잠재 세계의 총질량을 유한 수로 합산하지 않는다.

새로 방문한 영역을 그대로 tick=0 상태로 놓을 수 없다. 그곳에서도 붕괴·기후·확산이 진행되었을 수 있다. 도착 tick까지 정확한 독립 해, 보존된 boundary history를 이용한 catch-up, 또는 사전에 승인한 coarse evolution이 필요하다. none이면 그 영역은 해당 tick에 진입 가능한 세계로 제공할 수 없다.

### 활성 영역을 결정하는 기준

카메라·agent 거리만으로 물리를 끄지 않는다. 열·유체·EM·복사·입자·환경 source와 pending event의 의존 범위가 활성 영역을 결정한다. 유한 전파 속도와 compact interaction을 가진 부분계는 causal frontier를 사용한다. 세계 안에서 빛이 이미 통과한 먼 장소를 ‘아무도 안 봤다’며 초기화하지 않는다.

정확한 dormancy certificate는 외부 flux=0, 경계 조건 불변, pending event 없음 또는 정확한 독립 해가 존재함을 포함한다. 물체가 정지했다는 이유만으로 방사성 decay나 내부 열전도를 멈추지 않는다. certificate가 깨지는 tick까지 event scheduler가 깨운다. analytic decay는 단순 평균식인지 실제 선택한 stochastic process의 sample path를 보존하는지 구분한다.

elliptic electrostatic/gravity solve 같은 전역 제약이 있으면 유한 속도 cone 논리만으로 먼 영역을 생략할 수 없다. 전역 또는 승인한 multipole/boundary 모델이 필요하다. radiation과 파동이 전 세계를 채우면 활성 영역도 커진다. 2D 균일 영역에서 전파 반경이 시간에 비례하면 잠재 활성 면적은 시간의 제곱에 비례할 수 있다. lazy loading이 이 비용을 없애지 않는다.

### 경계 profile

closed·periodic·open boundary는 별도 물리 조건이다. 영역 확장은 기존 벽을 삭제하는 것과 다르다. 처음부터 열린 확장 경계를 가진 profile은 외부 초기조건과 유입·유출 규칙을 기록한다. 실험 도중 새 대륙·자원·기후를 생태 회복 목적으로 추가하면 intervention으로 명시하고 R3 자연 역사와 구별한다.

**경계:** W는 초기조건·경계·시간 진행이다. P는 국소 이동·탐지다. E는 이주·고립·전파·환경 개조다. U는 물리적으로 진행된 영역과 아직 실현하지 않은 공간을 구별한다.

## 128 유한 계산기에서 오래 살아가는 상태

### 저장 계층과 의미

Hot은 CPU RAM 또는 VRAM에서 현재 쓰는 상태, Warm은 RAM·SSD의 정확한 상태, Archive는 체크포인트와 재현 가능한 기록이다. 어느 계층에 있느냐는 생물에게 노출하지 않는다. page miss 때문에 굶거나 동결되는 개체는 없다. 필요한 state가 늦으면 세계 시간이 기다린다.

Cold라고 물리 변화가 없는 것은 아니다. 활성 field·chemistry의 state를 디스크로 내렸다가 매 substep 다시 읽으면 저장 계층이 병목이 된다. 먼저 active working set이 실제 resident 용량에 들어가는지 예산을 잡는다. 그렇지 않으면 decomposition이나 명시적 B 모델을 사용한다. 스왑을 켜면 무한 RAM이 생기는 것이 아니다.

| 기법 | 허용 의미 | 비용·한계 |
| --- | --- | --- |
| immutable genome/module interning | 같은 bytes의 host 중복 제거 | 물리 복사·유지 비용은 각 개체가 부담 |
| exact compression·repeat DAG | 원래 state를 lossless 복원 | 임의 고엔트로피 상태는 잘 줄지 않을 수 있음 |
| checkpoint + 입력 log | 모든 intermediate state 저장 없이 재현 | 되감기 때 계산 시간이 필요 |
| causal dormancy | 증명 가능한 무변화/독립 해만 생략 | 열·방사선·파동 때문에 적용 폭이 작을 수 있음 |
| 분산 memory·storage | 한 기계의 용량 확대 | 네트워크·장애·운영 비용, 총자원은 유한 |
| coarse model | 실제 계산량 감소 | B/C, 역사를 정확히 복원할 수 있다고 보장 못 함 |

### 예산 정책

HostBudget은 world law가 아니다. high-water mark는 새로운 tick 시작 전과 큰 transaction 예약 전에 확인한다. 실제 할당·spill·공유·snapshot release로 해결할 수 없으면 마지막 완결 tick으로 되돌리고 RunHalt(ResourceLimit)를 기록한다. 비싼 genome·새로운 분자·희귀 lineage를 우선 삭제하는 정책은 금지한다.

ResourceToken은 예약된 host bytes와 작업 slot의 소유권이다. 예약 순서는 canonical transaction order이며 host limit가 생태 성공 여부를 결정하지 않도록, 용량 부족 때는 전체 해당 stage를 멈춘다. 압축 thread 지연 때문에 일부만 실패하게 하지 않는다. 다른 컴퓨터에서 이어 실행할 수 있게 중단 이유와 필요한 추가 예산을 보고한다.

MemoryEstimator는 live state, solver scratch, rollback journal, output buffer, GPU halo, checkpoint pin, 최악 예상 birth/graph expansion을 따로 합산한다. live state만 RAM 사용량으로 소개하지 않는다. Double-buffer를 쓰면 그만큼 더 필요하고 COW snapshot 중 dirty page가 많으면 peak가 증가한다.

### 추가 store의 크기 예산

아래 stride는 host layout 설계 예산이며 Rust size_of 실측값이 아니다. pointer·padding·arena metadata·임시 buffer와 원자 graph 자체는 별도다. 실제 layout이 달라지면 표를 갱신한다.

| 항목 | 수와 가정 stride | payload 산술 |
| --- | --- | --- |
| sparse isotope inventory | 100만 material × 평균4entry ×16B | 약61.04MiB |
| 활성 radiation track | 100만 ×128B 계획 stride | 약122.07MiB |
| QM request/result header | 10만 ×128B 계획 stride | 약12.21MiB; matrix/gradient 제외 |
| LawField 기본 χ·πχ | 1,024만 cell ×2f64 | 156.25MiB; halo/history/energy 제외 |
| rollback dirty copy | 64GiB state의10% 변경 | 6.4GiB 추가; checkpoint pin 별도 |

모든 host field에 dynamic vector를 하나씩 두는 설계보다 key/count span과 shared immutable definition을 쓴다. 하지만 실제 활성 track이나 독립 화학 구조가 많으면 payload 자체가 증가한다. capacity를 넘어선 결과를 ‘희귀하므로 생략’하지 않는다. 통합 peak는 이 표와 R9의 개체/기억/신경,129절 field/QM scratch,134절 저장 pin을 함께 합산한다.

### 메모리·지식의 개방성과 한계

생물이 얻는 기억 공간은 자원·구조·유지 비용에 비례해 늘릴 수 있다. host address type·세계 물질·소유 매체는 유한하다. 무한 tape를 제공하지 않는다. 유전 code·학습 weight·문화 기록이 공유 host block에 저장되어도 실제 매체가 파괴되면 그 소유자의 지식은 소실된다.

하나의 seed만으로 모든 외부 입력·backend·데이터 변경·solver 상태 없이 임의의 긴 역사를 재현할 수 있다고 하지 않는다. 과거 상태 전체를 언제나 즉시 볼 수 있는 기능과 작은 save file은 trade-off다. 관찰자가 고른 snapshot·필수 provenance·checkpoint 간격을 조절하고 재계산 비용을 표시한다.

**경계:** Engineering Contract는 용량·캐시·저장·정지 정책이다. P는 세계 내 유료 기억만 접근한다. E의 성공과 실패는 host 최적화 때문에 바뀌지 않아야 한다. U는 실제 저장 범위·되감기 비용·메모리 한계를 표시한다.

## 129 병목을 찾는 기준과 파동장의 실제 계산량

성능 최적화의 첫 산출물은 ‘빨라 보이는 demo’가 아니라 동일 workload의 phase 비용표다. agent 수만으로 부하를 정의하지 않는다. 활성 field cell, substep, graph node/edge, 반응 채널, neural edge update, VM CU, 핵 충돌, QM patch 크기·수, dirty page, 기록량을 함께 기록한다.

### 필수 측정 항목

| 영역 | 직접 기록하는 값 | 문제를 판별하는 근거 |
| --- | --- | --- |
| 전체 tick | p50/p95/p99 wall time·CPU time·대기 시간·TPS | 평균은 빠르나 간헐적으로 멈추는지 |
| CPU kernel | instruction·branch miss·cache/TLB miss·실효 대역폭 | 계산 부족인지 데이터 이동인지 |
| GPU | kernel 시간·점유·메모리 전송·launch·synchronize | 장치 계산보다 왕복/동기화가 큰지 |
| chemistry/QM | 후보 수·SCF iteration·basis 크기·cache miss·실패 | 생화학 복잡도가 어디서 비용을 만드는지 |
| nuclear | event·track·collision·packet split·queue peak | 높은 활성도와 희귀 track tail |
| host memory | live·scratch·journal·pin·peak·allocation 수 | 평소 RAM과 checkpoint peak 차이 |
| 저장·분석 | bytes/s·queue depth·compression time·observer lag | 출력이 simulation을 막는지 |

모든 timer는 논리 세계 밖에 둔다. profiling을 켜고 꺼도 state hash가 같아야 한다. 값은 최근 rolling window와 장기 histogram으로 보되 per-agent 매 tick string formatting은 금지한다. CPU/GPU clock·전력 상태·compiler flags·thread affinity·NUMA·backend·dataset·solver tolerance를 benchmark manifest에 포함한다.

### 대역폭과 연산량의 하한 계산

한 단계의 예상 시간은 적어도 max(실제 memory bytes / 측정 대역폭, 실제 연산량 / 측정 연산 처리율)에 제약된다. 이것은 자원별 하한이며 cache miss latency·branch·동기화·I/O가 더해진다. 서로 의존하는 phase는 단순 max로 겹칠 수 없고 task DAG의 critical path를 합쳐야 한다. 이 구분은 Roofline 성능 모델과 같은 취지다. [Williams·Waterman·Patterson, 2009](https://escholarship.org/uc/item/5tz795vq)

R8 파동 reference는 tile당 fine cell16개, tick당256 substep이다. 따라서 tile당4,096 cell update다. 아래는 매 update에서6성분 f64를 한 번 읽고 한 번 쓰는 **96B streaming-pass 가정**이다. material·current·halo·추가 buffer는 제외한다. temporal cache blocking으로 DRAM bytes가 줄 수 있으므로 이것을 모든 구현의 절대 DRAM 하한이라고 하지 않는다.

| 세계 | cell update/tick | 가정 traffic/tick | 200GB/s에서 시간 | 1TB/s에서 시간 |
| --- | --- | --- | --- | --- |
| 64×64 | 16,777,216 | 1.61GB | 0.0081s | 0.0016s |
| 128×128 | 67,108,864 | 6.44GB | 0.0322s | 0.0064s |
| 256×256 | 268,435,456 | 25.77GB | 0.1288s | 0.0258s |
| 512×512 | 1,073,741,824 | 103.08GB | 0.5154s | 0.1031s |
| 3,200×3,200 | 41,943,040,000 | 4,026.53GB | 20.1327s | 4.0265s |

GB·TB는 decimal이다. 두 대역폭은 비교용 가정이지 특정 CPU/GPU 실측값이 아니다. 최종 행은 파동장만으로 각각 약0.050/0.248 TPS에 해당한다. 이 가정에서 대형 세계10 TPS는 데이터 이동 예산과 맞지 않는다. ‘Rust로 작성’ 또는 ‘GPU 사용’만으로 해결했다고 할 수 없다.

전 영역 fine field의192B/cell 계획 메모리는 약29.30GiB다. 여기에 생물·분자·열·물질·solver scratch·halo·journal을 더해야 한다. 32GiB RAM 컴퓨터에 전체 모델이 여유 있게 들어간다는 주장은 부정확하다. 최종 메모리는 실제 layout 측정 뒤 확정한다.

### 양자 solver의 별도 원가

patch의 원자 수만큼 중요한 값은 basis function 수 b와 SCF iteration I다. 일반적인 dense reference를 선택하면 주요 행렬 저장은 b², 일부 대각화 연산은 b³에 따라 증가한다. 특정 method의 실제 비용은 profiler로 확인한다. 6개의 b×b f64 행렬만 세면 b=512에서12MiB, b=1,024에서48MiB다. 8개의 동시 job은 각각96/384MiB이며 geometry·integral·gradient·restart·라이브러리 scratch는 추가다. atom64개가 곧 basis64개라는 계산은 틀리다.

patch를 작게 잘라 비용을 줄이는 것은 경계 결합을 바꾸는 근사다. 전자 비국소성이 큰 구조·금속성·전하 이동에서 독립 fragment 합산이 실패할 수 있다. field의 GPU bandwidth가 충분해도 QM p95 tail이 tick의 critical path를 지배할 수 있으므로 두 병목을 따로 측정한다.

### 가장 큰 절감 지점

우선순위는 계산할 필요 없는 동일 작업 제거, 물리적으로 허용되는 국소/다중 해상도, 데이터 상주·batching, 병렬화, SIMD·kernel tuning 순이다. 116절의 이상적 AMR fine fraction1%는 field update 수를 약39.26배 줄일 수 있지만 interface·reflux·재료·wave spectrum 비용을 뺀 계산이다. 파동이 모든 영역에 미세 해상도를 요구하면 절감이 사라진다.

한 phase만 빨라질 때 전체 speedup은 1/[(1-p)+p/s]다. 전체의10%를 차지하는 부분을10배 가속해도 약1.10배,90%라면 약5.26배다. 먼저 비용 비율 p를 측정하지 않은 ‘100배 최적화’ 계획은 성능 근거가 아니다.

**경계:** Engineering Contract는 계측·비용 모델·보고다. P 없음. E는 측정 때문에 달라지지 않는다. U는 목표·가정·실측을 다른 표식으로 보여 준다.

## 130 CPU·캐시·VM·신경·graph 최적화

### 자료구조와 소유권

tile·field·bulk stock은 chunk SoA, 함께 읽는 VM register와 작은 header는 AoS, 신경 edge·충돌 후보처럼 같은 연산을 하는 집합은 AoSoA batch를 후보로 둔다. 16/32 lane block 크기와64B 정렬은 benchmark 후보이며 agent의 형태나 세계 규칙이 아니다. 지나친 bit packing으로 매 접근마다 unpack·read-modify-write가 생기면 더 느릴 수 있다.

hot/cold split은 position·energy·local index·PC처럼 매 tick 쓰는 자료와 ancestry·긴 code·UI metadata를 분리한다. graph adjacency는 slab/arena의 연속 edge span으로 두고 작은 degree를 inline 저장한다. node별 Box와 VM instruction별 heap allocation은 피한다. 삭제는 generation handle로 안전하게 처리하고 compaction은 UID·논리 순서를 바꾸지 않는다.

청크 owner worker는 자신의 mutable span만 쓴다. 경계 효과는 target UID와 논리 시각이 포함된 delta buffer에 쓴다. spatial bins와 stable radix sort를 통해 deterministic resolve로 모은다. hash map의 순회·원자 연산 승자·lock 획득 순서가 행동 우선권을 만들지 않는다. 경쟁 정책 자체는 기존 공정성 계약을 유지한다.

### 병렬화와 NUMA

| 방법 | 효과 | 위험·채택 조건 |
| --- | --- | --- |
| 지역 worker arena | allocation·lock·false sharing 감소 | peak와 idle memory 낭비 측정 |
| NUMA first-touch·shard 고정 | 원격 메모리 접근 감소 | 개체 이동·부하 편차로 정적 배치가 나빠질 수 있음 |
| batch work stealing | 작은 작업의 분배 overhead 감소 | commit 순서와 RNG는 worker에 의존 금지 |
| spatial reorder | 이웃 조회·graph 접근 지역성 향상 | UID는 그대로, host index만 변경 |
| 경계 color/owner | conflict 없는 update 병렬화 | 물리적 순서가 바뀌지 않는 경우만 A |

NUMA 복제 cache에는 immutable 자료만 둔다. active world를 무조건 여러 node에 복제하면 bandwidth와 coherence 비용이 커진다. 하나의 thread pool이 CPU core budget을 소유하며 Rayon·BLAS·codec·observer가 각각 core 수만큼 thread를 만드는 중첩 oversubscription을 막는다. kernel이 메모리 포화라면 thread를 더 늘리지 않는다.

### build와 hot path

release 최적화·LTO·대표 workload 기반 PGO는 후보로 두고 compiler/target feature를 BackendManifest에 고정한다. opcode dispatch만 훈련한 PGO가 reaction-heavy 세계에서도 유리하다고 가정하지 않는다. 느린 오류 경로는 hot loop 밖으로 분리하고 per-tick allocation·formatting·로그 syscall을 제거한다. 경계 검사 제거를 위해 unchecked 접근을 쓰기 전에 bounds proof와 reference 대조가 필요하다. FFI·GPU buffer lifetime의 unsafe 범위는 좁게 유지한다.

lock-free라는 이름만으로 채택하지 않는다. 작은 task의 과도한 atomic·cache-line bounce가 더 비쌀 수 있다. 소유권이 분명한 bounded SPSC/batch queue와 phase barrier부터 측정하고 실제 경쟁 지점에만 더 복잡한 queue를 쓴다. sequence counter·resource reservation·fault propagation도 결정론 계약을 따른다.

### VM과 신경

bytecode는 revision별 decode cache를 공유한다. 같은 revision/PC를 실행하는 lane을 묶으면 dispatch와 branch를 줄일 수 있으나 lane마다 CU·memory 권한·yield·world intent 순서를 보존한다. loop를 통째로 host 계산으로 바꿔도 세계 내부의 instruction 비용과 시간이 사라지지 않는다. self-modifying code와 revision adoption은 decode cache를 정확히 무효화한다.

정수 신경 kernel은 packed sparse edge와 고정 reduction tree를 사용한다. 같은 topology를 가진 개체를 batch 처리하되 weight/state는 별도다. N=0 또는 작은 module이 흔하면 CPU가 유리할 수 있다. R9 예제617CU의 epoch를 GPU에서 빨리 계산해도32CU/tick 개체가20tick보다 일찍 출력을 받지 않는다.

학습 프로그램도 VM budget을 사용한다. host가 모든 개체의 gradient를 공짜로 미리 계산하는 경로는 없다. 같은 입력·같은 weights의 정확한 memoization은 계산 결과 cache일 뿐 세계 CU 면제는 아니다. 학습 trace와 provenance 전체를 매 tick observer로 복사하지 않는다.

### 화학 graph·핵종·QM

local reaction index는 node type·bond pattern·charge·temperature bin 등 후보 배제용 정보로 구성하되 bin 경계가 실제 reaction을 누락하지 않도록 보수적 후보 집합을 만든다. authoritative rate는 원래 상태에서 계산한다. 변경 없는 영역의 exact match cache와 immutable species table을 공유한다. 희귀 species를 budget 이유로 버리지 않는다.

SCF는 patch별 독립 job pool을 우선 사용하고 각 job의 내부 BLAS thread는 초기에1로 고정해 측정한다. 매우 큰 단일 patch에서만 BLAS 병렬과 job 병렬을 바꿔 비교한다. 외부 solver process pool은 crash 격리·버전 고정에 유리하지만 serialization 비용이 있다. 초기 reference는 격리 adapter, production FFI는 상태/오류/동시성 시험 뒤 선택한다.

cache key에는 모든 과학적 입력과 solver restart state를 포함한다. 총 전하·spin·isotope·embedding을 생략한 ‘같은 분자 이름’ cache는 금지한다. isotope가 전자구조 계산 결과에 영향을 주지 않는 method라도 kinetics·vibration 결과의 cache는 별도로 구분한다.

**경계:** Engineering Contract는 동일 계산의 배치·소유권·cache다. P의 비용·권한·시간은 유지한다. E의 차이가 발견되면 A 최적화 실패다. U는 kernel별 profile과 정확성 gate를 표시한다.

## 131 GPU 가속·데이터 상주·시간 타일링

### 이종 실행의 추천 배치

| 작업 | 초기 추천 위치 | 이유와 전환 조건 |
| --- | --- | --- |
| dense Maxwell·열·유체 stencil | GPU 생산, CPU reference | 규칙적인 대량 연산; resident working set 필요 |
| VM·복잡한 graph mutation·transaction | CPU | 분기·작은 task·pointer 의존; batching 후 재평가 |
| 작은 희소 neural module | CPU batch | 전송·launch보다 일이 적을 수 있음 |
| 큰 동종 neural/반응 rate batch | GPU 후보 | 충분한 batch와 고정 정수/수치 계약 필요 |
| QM | 승인된 backend별 CPU/GPU | method·basis·solver 특성에 의존, GPU 자동 가속 아님 |
| 고에너지 transport | 먼저 CPU reference | branch divergence·긴 track tail; 이후 상태별 batch |

Rust Runner와 `PhysicsBackend` 경계는 유지한다. 첫 고성능 GPU 경로는 별도 native CUDA adapter 후보, 다른 장치에는 검증된 대체 backend를 둘 수 있다. Bevy의 rendering API가 simulation 정밀도를 결정하지 않는다. 일반 shader 경로에 f64가 항상 있다고 가정하지 않는다. 특정 API 버전과 기능은 구현 시 capability gate에서 고정한다.

NVIDIA 공식 지침은 host-device transfer 최소화·device 상주·작은 전송의 batching을 강조한다. CT3도 field arrays를 tick마다 CPU로 되돌리지 않는 구조로 설계한다. 여기서 문서상의 원칙은 참고이고, CT3 속도 향상량은 별도 실측 대상이다. [CUDA Best Practices](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html)

### 권위 상태와 전송

GPU field shard는 해당 시간 구간 동안 단독 authoritative owner다. CPU mirror는 revision-tagged read cache다. CPU intent를 boundary source buffer로 보내고 GPU는 국소 감각·에너지 flux·coarse render tile만 되돌린다. checkpoint 때는 정해진 consistent stage에서 field를 저장한다. CPU가 오래된 mirror에 쓴 뒤 ‘최신 wins’로 합치지 않는다.

작은 감각 요청은 공간별로 묶는다. pinned staging buffer는 bounded pool로 재사용한다. 너무 많은 pinned memory는 호스트 운영을 방해하므로 필요한 전송 창만 고정한다. asynchronous copy와 kernel overlap은 실제 데이터 의존성이 없는 구간에서만 사용한다. 같은 source buffer를 DMA 중 재사용하지 않는다.

전체29.30GiB field 계획량은 하나의24GiB VRAM에 들어가지 않는다. compressed resident model·domain partition·더 큰 VRAM·다중 GPU 중 실제 조건에 맞춰 선택한다. Unified Memory page fault를 매 substep 일으키며 해결했다고 하지 않는다. VRAM 부족 때문에 카메라 밖 물리를 끄지 않는다.

### kernel fusion과 temporal blocking

fuse 가능한 인접 kernel은 중간 global write를 줄인다. 반대로 register pressure·shared memory·occupancy가 나빠지면 느려질 수 있으므로 작은 kernel과 비교한다. launch graph 재사용은 task topology가 반복될 때 후보이며 동적인 graph 확장을 숨기지 않는다.

k개의 stencil substep을 지역적으로 묶으면 시간 재사용으로 DRAM 이동을 줄일 수 있다. 이를 위해 stencil radius에 비례한 넓은 halo와 줄어드는 유효 interior가 필요하다. k=4와 radius1이면 독립 계산에 최소4cell의 dependency halo가 필요하다는 식으로 DAG를 확인한다. 중간 시각의 입사 source·전하 이동·물질 변화가 있으면 그 coupling도 포함하거나 거기서 fusion을 끊는다.

동일한 계산 순서와 반올림을 보존하는 temporal tiling은 A 후보다. 중간 source를 고정하거나 halo를 늦게 교환하면 B다. shared memory에 전 영역을 넣는 것이 아니라 tile 크기·halo·register·occupancy의 최적점을 찾는다. field만 빠르게 돌리고 charge continuity를 어기는 경로는 사용할 수 없다.

### 정밀도와 재현성

FP32·mixed precision은 큰 절감 가능성이 있으나 기본 f64와 동일하지 않다. energy drift·분산·Gauss residual·작은 신호·장기 선택압을 B gate로 검증한다. quantum SCF residual은 물리 결과에 영향을 주므로 낮은 precision의 validity를 별도 검증한다. `fast-math`·FMA·denormal 처리도 BackendManifest의 일부다.

atomic float add는 순서가 달라질 수 있다. cell owner gather·고정 reduction tree·정수 charge/extent 누계 등의 대안을 우선한다. GPU의 실행 순서 차이를 관찰자의 분석 cache 차이와 같은 수준으로 취급하지 않는다. strict CPU reference와 GPU production의 동일성 수준을 명시한다.

**경계:** Engineering Contract는 장치 분배·resident state·precision이다. P는 장치 종류를 알지 못한다. E는 backend 조건과 함께 검증한다. U는 physics FPS가 아닌 실제 TPS와 장치별 병목을 표시한다.

## 132 다중 GPU·분산 세계·독립 실험 병렬

### 무엇을 확장하는가

| 방식 | 장점 | 단점·성능·복잡도 | 추천 순서 |
| --- | --- | --- | --- |
| 독립 seed/parameter 실행 병렬 | 통신이 거의 없고 연구 표본 증가 | 하나의 세계가 빨라지지는 않음 | 가장 먼저 |
| 단일 node 다중 GPU/NUMA | 메모리·처리율 증가 | halo·장치 간 이동·부하 불균형 | resident 단일 GPU 이후 |
| 여러 node의 한 세계 | 훨씬 큰 총용량 | 매 substep network·장애·분산 commit | 단일 node 한계를 실측한 뒤 |

AMReX는 block-structured AMR·GPU·MPI 계층 병렬의 검증된 설계 참고다. CT3는 그 원리를 비교하되 도입 여부를 따로 결정한다. AMReX를 연결하는 것만으로 CT3의 charge·energy·생명·결정론 계약이 해결되지는 않는다. [AMReX Introduction](https://amrex-codes.github.io/amrex/docs_html/Introduction.html)

### domain ownership

물리 shard의 직사각형 field 범위와 local chunk store는 명확한 owner가 있다. ghost cell은 읽기 복사본이다. 공유 face flux는 한 번 계산하거나 양쪽이 동일한 canonical flux를 받아 debit/credit한다. moving body·chemical patch가 shard를 가로지르면 owner와 remote contribution을 고정하고 interface solve를 수행한다.

agent migration은 commit boundary에서 state·pending kernel·RNG ordinal·germline/somatic revision·local handle mapping을 함께 옮긴다. 도착 순서가 birth ID와 행동 우선권을 정하지 않는다. 값비싼 patch가 한 shard에 몰리면 job 계산만 다른 worker에 빌려 주되 authoritative state ownership은 transaction 완료까지 유지한다.

partition의 host mapping은 바꿀 수 있지만 physical stencil·reduction·arbitration 순서는 그대로다. partition이 부동소수점 reduction tree를 바꾸면 새로운 BackendManifest 또는 정확한 canonical reduction을 사용한다. 같은 seed인데 GPU 수에 따라 결과가 바뀌는지를 별도 시험한다.

### halo와 latency 예산

정사각 fine cell 영역 n×n, halo 폭 h,6개 f64 성분을 교환하는 단순 한 방향 payload는 대략4nh×48B다. corner·복수 stage·send/receive 양방향·material 데이터는 제외한 계산이다. n=1,024,h=2이면 약384KiB, n=256이면96KiB다. volume 대비 halo cell 비율은 각각 약0.78%,3.13%로 증가한다.

큰 node 수에 잘게 나눌수록 계산 면적은 줄지만 경계 비율과 고정 latency가 커진다. 256회 교환에 유효 latency20μs가 매번 critical path로 남는 가정이면 그것만5.12ms/tick이다. 이는 장비 실측이 아닌 산술 예다. global collective나 여러 stage면 더 커진다.

interior kernel과 halo transfer를 겹친 뒤 boundary kernel을 실행한다. persistent communication·batch message·GPU direct transfer는 지원 환경에서 측정 후보다. 그동안 이웃의 다음 시각 값을 추측해 사용하는 것은 exact overlap이 아니다. temporal halo는131절의 dependency 계약을 따른다.

### 장애와 비용

한 node가 실패하면 일부 shard만 다음 tick을 계속 진행하지 않는다. consistent checkpoint와 input log에서 전체 해당 logical epoch를 복구한다. 비동기 체크포인트는 각 shard의 같은 tick revision을 모아 manifest가 원자적으로 commit될 때만 유효하다. topology·환경 차이로 backend가 달라지면 bitwise replay 범위를 다시 선언한다.

분산 mode는 자동 cloud 증설을 실행하는 기능으로 설계하지 않는다. 사용자에게 자원 요청·최대 비용·예상 throughput을 표시하고 운영 범위 안에서만 사용한다. 본 설계 작업은 실제 장비·클러스터를 구매하거나 실행하지 않는다.

**경계:** Engineering Contract는 분할·통신·복구다. P와 물리 경계는 node 경계와 무관하다. E는 한 세계 또는 독립 표본별로 관찰한다. U는 단일 세계 TPS와 ensemble throughput을 분리한다.

## 133 다중 시간척도·해상도·근사의 승인

### 왜 전부 같은 tick 크기로 풀지 않는가

전자 relaxation·핵 사건·광파·열·몸 운동·기후·유전은 시간척도가 크게 다르다. 가장 빠른 현상을 모든 변수의 dt로 삼으면 다른 계산까지 폭증한다. 반대로 빠른 현상을 매 tick 평균으로 뭉개면 공진·손상·반응 feedback을 놓친다. 서로의 flux와 오차를 보존하며 각 부분계의 시간 간격을 나누는 것이 목표다.

| 전략 | 가능 이득 | 바뀌는 것·검증 |
| --- | --- | --- |
| exact event skip | 실제 사건 없는 구간 비용 제거 | 선택한 stochastic path/hazard 보존 필요 |
| local subcycling | 빠른 지역에만 짧은 dt | 경계 flux 누계와 causal ordering |
| implicit/IMEX | stiff 안정성 제약 완화 | 안정성은 정확성과 다름; 파동 분산·damping 검증 |
| quasi-static/QM ground state | 빠른 자유도를 유효 상태로 제거 | 비평형·여기·고주파 조건에서 validity 제한 |
| AMR | 공간·시간 update 수 감소 | charge·energy·momentum·interface reflection |
| coarse climate/geology | 느린 대규모 변화 비용 감소 | subgrid closure와 feedback 오차 |

기본값으로 local dyadic time lattice를 사용한다. macro tick을2의 거듭제곱 substep으로 나눠 상호 교환 시각을 정확히 맞춘다. 핵 track event는 선택한 수송 clock에서 실행하고 교환 시각까지의 침적을 모은다. dyadic lattice에 맞춘 event time quantization이 통계와 transport를 바꾸면 B다. 최소 dt보다 빠른 유효 현상은 더 깊은 level 또는 ModelDomainExceeded다.

### refine/coarsen 조건

오차 지표는 국소 wavelength·field gradient·Gauss residual·chemical rate stiffness·energy exchange residual·충돌/변형률·QM validity다. population이 적거나 lineage가 흔하다는 이유로 덜 정밀하게 계산하지 않는다. mesh 변경은 manifest에 고정된 조건과 hysteresis로 실행하며 host 부하에 따른 자동 threshold 변경은 금지한다.

coarsen 시 질량·전하·총에너지·momentum과 검증된 moments를 보존한다. 정보가 사라지는 mapping이면 A가 아니다. fine 구조의 topology·기록·기계적 기능이 coarse model에 표현되지 않으면 보존량만 맞아도 coarsen할 수 없다. 빈 것처럼 보이는 공간의 약한 신호도 유효한 통신 매체일 수 있다.

AMR transition은 수송 face flux register와 reflux, Maxwell의 discrete charge continuity·Gauss 제약을 함께 다룬다. material 이동으로 mesh가 바뀌는 경우도 같은 시험을 통과해야 한다. 에너지 보존만 확인하고 간섭·산란·위상 오차를 놓치지 않는다. reference보다25배 빠르더라도 장치 기능이 사라지면 지원하지 못하는 모델이다.

### surrogate와 자동 tuning

offline 물성 surrogate는 energy를 예측하고 그 gradient로 force를 얻는 conservative 형태를 우선한다. held-out 구조·charge·strain·temperature에서 검증하고 out-of-domain에서는 승인된 reference로 돌아간다. 실패를 감지하지 못한 surrogate는 진화가 악용할 수 있는 새로운 세계 법칙이 된다. model version을 실행 중 조용히 학습·교체하지 않는다.

host auto-tuner는 block size·thread count·job batch처럼 과학적 결과를 보존하는 A 항목만 자동 변경한다. dt·cutoff·precision·species pruning·SCF tolerance를 FPS에 맞춰 바꾸지 않는다. 수치 모델을 바꾸는 실험은 새로운 DiscretizationSpec과 시작/분기 기록을 요구한다.

### 과학적 동등성 gate

B backend는 작은 exact/reference 계의 반응률·확산·파동 위상·열/전하/원자 보존, 복합 witness 기능, 여러 seed의 생존·lineage·행동·기술 통계에 대해 refinement convergence를 보인다. 혼돈계의 장기 궤적 일치를 요구하지는 않지만 차이를 무시하지 않는다. calibration seed와 평가 seed를 분리하고 confidence interval·censoring·실패 run을 함께 공개한다.

미세한 결정론 차이가 오랜 뒤 다른 문명을 만들 수 있다. 따라서 ‘시각적으로 비슷한 결과 하나’는 통계 동등성의 근거가 아니다. 새로운 화학·기술을 찾는 실험에서는 기존 witness만 통과해도 아직 발견되지 않은 기능이 보존됐다고 보장할 수 없다. 표현 가능한 범위와 실제 검증 범위를 구별한다.

**경계:** W는 선택한 시간·공간 근사와 closure다. P는 변화시키지 않는다. E의 가능 공간은 근사 때문에 달라질 수 있다. U는 backend별 resolution·validity·연구 비교 조건을 보여 준다.

## 134 저장·역사·관찰·렌더링의 병목

### 기록량부터 계산한다

100,000개체가16tick마다 하나의32B event를 만들고10TPS로 실행한다는 가정은 초당62,500event,2MB/s,하루172.8GB다. 문자열·index·복제·압축 metadata는 제외한 raw 값이다. 모든 행동을 영구 저장하는 설계는 빠르게 디스크 병목을 만든다. 실제 event rate와 compressibility를 측정해야 한다.

64GiB state의2%가100tick마다 dirty이고10TPS라는 가정은10초마다1.28GiB,약131.07MiB/s의 raw dirty payload다. 이미 존재하는 불변 block은 다시 쓰지 않되 변경된 많은 field는 높은 압축률을 보장하지 못한다. 동시 checkpoint pin과 journal도 peak RAM에 포함한다.

### 저장 형태

체크포인트는 manifest + content-addressed immutable blocks + dirty page delta + input segments다. block hash와 corruption 검사를 분리하고 manifest commit 이전의 파일은 완결 save가 아니다. dedup은 실제 bytes가 동일할 때만 한다. Merkle tree의 root만 저장한다고 모든 과거 payload가 없어지는 것은 아니다.

세계 미래를 복구하는 데 필요한 RNG 주소·solver restart·residual·registry hash·모든 mutable state는 lossless다. 기술 분석의 raw trace는 별도 retention 정책을 적용할 수 있다. provenance를 줄인 구간은 ‘unknown’으로 표시하며 관찰기가 빈 역사를 상상해 채우지 않는다.

input log와 deterministic replay로 행동을 재계산할 수 있으면 모든 intent를 영구 log에 복제할 필요는 없다. 대신 birth·parent·module acquisition 등의 필수 계보 사실은 compact integer event로 유지하고 실제 크기를 예산에 넣는다. 전체 ancestry도 개체가 계속 태어나면 무한히 작을 수 없다. 오래된 segment는 archive할 수 있지만 exact lineage 보기에는 다시 읽어야 한다.

### 압축·I/O

chunk/page delta·bit plane·정렬된 stock sparse encoding·일반 lossless codec을 비교한다. codec 이름만으로 압축률을 약속하지 않는다. dirty mask 계산 자체와 hash CPU 비용도 profile한다. 압축 thread는 simulation의 core budget을 무제한 빼앗지 않는다. 저장 queue가 차면 checkpoint 간격을 규정 범위에서 바꾸거나 wall time을 늦춘다. 필수 기록을 무음 drop하지 않는다.

history의 일부를 summary로 남기는 선택은 세계 상태의 근사와 다르다. replay 자체를 잃지 않는 summary와 복구 불가능한 삭제를 구별한다. delta chain 길이에 상한을 두고 주기적으로 새로운 base index를 만들되 unchanged payload는 재사용한다. SSD 수명·free space·write bandwidth는 장기 운용 계측에 포함한다.

### 관찰·화면

Viewer는 최신 immutable render snapshot을 bounded mailbox로 받는다. 밀리면 오래된 **render snapshot**은 버릴 수 있다. physics event·checkpoint·입력은 같은 정책으로 버리지 않는다. MAX에서는 field heatmap·agent 표시·UI 통계 빈도를 낮추고 simulation thread를 기다리게 하지 않는다.

카메라 ROI와 zoom별 tile texture/instance LOD는 렌더링에만 적용한다. 먼 agent를 simulation에서 제거하지 않는다. deep zoom 시 genome·QM patch 상세 요청은 read-only revision에 붙고 결과 도착 때 해당 revision을 표시한다. observer clustering은 budgeted background job이며 느려지면 해석이 늦게 나올 뿐 세계가 바뀌지 않는다.

render FPS·simulation TPS·observer lag·snapshot age를 각각 표시한다. Pause는 완결 commit에서 멈춘다. 1x 기준10TPS,500x는5,000TPS 목표 요청이며 실현되지 않으면 실제 배속을 표시한다. 물리 dt를500배 키워 배속 숫자를 맞추지 않는다.

**경계:** Engineering Contract는 저장·전송·retention이다. P는 분석 데이터에 접근하지 않는다. E는 화면 여부와 무관하다. U는 원자료·분석·재현 가능 구간과 표시 지연을 구별한다.

## 135 Rust 구성·자료형·소유권·CT3 phase

이 절은 구현 경계와 데이터 계약이며 전체 소스코드가 아니다. R9의 crate와 VM/신경 구조를 유지하고 아래 모듈을 추가한다. 서로 다른 solver가 같은 store를 직접 수정하지 않도록 DTO와 transaction을 통해 결합한다. 컴파일 feature와 세계 capability는 다르며 실행 manifest가 최종 물리 범위를 고정한다.

### crate와 module

| crate / module | 소유하는 책임 | 의존 경계 |
| --- | --- | --- |
| civ_types / civ_numeric | 좌표·UID·typed unit·checked ledger·clock | Bevy·solver·observer 의존 없음 |
| civ_nuclear | nuclide·channel·inventory·transport·depletion | types/numeric, frozen matter view 입력 |
| civ_quantum | patch·request·result·validity·energy correction | types/numeric; backend adapter 격리 |
| civ_quantum_backend | xTB C API/process worker·오류·version 검사 | 외부 solver, world state 직접 수정 금지 |
| civ_physics::law_field | χ·πχ·potential·matter coupling | LawField profile에서만 사용 |
| civ_world::directory | sparse chunk·frontier·dormancy·realization | 물리 증명/flux DTO를 받아 storage 결정 |
| civ_backend | CPU/GPU shard·kernel plan·resident buffer | kernel interface, observer/UI 없음 |
| civ_core::coupling | phase DAG·ledger·residual·canonical commit | 모든 authoritative store의 최종 소유자 |
| civ_runner::resources | CPU pool·GPU executor·QM pool·backpressure | Core step과 wall-clock 자원 관리 |
| civ_storage / civ_observer / civ_viewer | CT3 save·새 사건 해석·방사선/오차 layer | read-only 세계 snapshot과 명시 입력 |

civ_vm은 civ_quantum_backend를 호출할 수 없다. civ_observer의 기술 ID·문명 ID는 civ_core로 들어가지 않는다. Bevy 의존성은 civ_viewer에만 둔다. native solver license·parameter 재배포·플랫폼 지원은 패키징 gate에서 확인하며 backend가 없으면 지원 profile을 시작하지 않는다.

### 핵심 struct의 필드 계약

| type | 필수 논리 필드 | 소유자 |
| --- | --- | --- |
| NuclideDef | key:u32, Z/N, excitation key, rest offset, decay span, support mask | immutable registry |
| NuclearInventory | material UID, sorted nuclide/count span, pending residual revision | MaterialStore |
| NuclearChannel | reactant/product spans, rate/model key, energy domain, sampler key | immutable data |
| RadiationTrack | UID, sector/kind, coordinate, direction, energy/weight, event ordinal, optical depth | NuclearStore |
| QuantumPatch | UID, atom span, input revision, embedding span, method hash, restart state ref | QuantumStore |
| QuantumRequest | patch/revision, stage clock, charge/spin, frozen input ref, retry ordinal | stage queue |
| QuantumResult | input hash, energy/gradient/charge spans, residual, validity, solver hash | stage result arena |
| LawFieldTile | χ, πχ, constitutive revision, local energy, boundary flux | FieldStore |
| DormancyCertificate | region, valid_from/until, state/boundary hashes, proof kind | WorldDirectory |
| ResidentShard | logical region, authoritative revision, backend key, buffer handles | BackendRuntime |
| StageLedger | matter/charge/energy deltas, boundary flux, numeric residual, source revision | Core coupling |
| MemoryEstimate | live, scratch, halo, rollback, checkpoint pins, reserved growth | Runner |

단위가 다른 에너지를 raw f64 하나로 섞지 않는다. typed quantity로 RestOffset·ChemicalEnergy·ThermalEnergy·FieldEnergy·RadiationEnergy를 구분하고 total reducer에서 공통 기준으로 환산한다. checked i128 등의 host type만 정하고 quantization scale을 임의로 생략하지 않는다. 확정 scale은 Data/Unit gate의 산출물이다.

### 핵심 enum과 interface

PhysicsProfile은 BaseCT3·LawFieldCT3·BenchCT3다. ApproxClass는 ExactA·DiscretizationB·RuleChangeC다. RadiationSector는 CoherentW·OpticalR·HighEnergyH이며 carrier의 단일 소유권을 나타낸다. SolverStatus는 Converged·Retryable·OutsideDomain·NumericalFailure다. RunHalt는 ResourceLimit·ModelDomainExceeded·ConservationFailure·SolverFailure·CorruptCheckpoint 등을 구별한다.

NuclideId와 SpeciesId는 역할·기술 enum이 아니다. ChannelKind는 붕괴·포획·산란 등 물리적 연산 dispatch이며 발전 방식이나 장치를 나타내지 않는다. DeviceId·ShardId는 host 주소로 agent의 READ에 노출하지 않는다.

QuantumBackend의 interface는 capability 조회, immutable request 제출, tagged result 수신, deterministic cancellation/cleanup이다. PhysicsBackend는 build_plan, stage_input, advance_stage, export_flux, checkpoint_view를 제공하는 계약이다. async completion 자체는 물리 시간이 아니다. world mutation 권한은 Core의 canonical commit에만 있다.

### 고정된 tick phase

| phase | 입력·처리 | 완료 조건 |
| --- | --- | --- |
| 0 Prepare | S_n pin, input 확정, host 예약, 이전 경계 flux 확인 | snapshot·manifest revision 고정 |
| 1 Maintenance | 기존 유지비·매체 상태·유효 권한 갱신 | 비용 ledger, 사망 후보만 표시 |
| 2 Sense/Think | 같은 phase view, VM·neural·학습·유전 operator | CU·에너지·yield 순서 보존 |
| 3 Resolve | 국소 intent 중재, source/force/material schedule | 실제 자원 예약과 canonical delta |
| 4 CoupledAdvance | t_n..t_n+1의 물질·장·열·핵·화학·QM·환경 | 모든 due result와 flux 결합 완료 |
| 5 LifeCommit | 발생·대사·배아·사망·잔해의 기존 순서 적용 | 중복 debit·출생 재고 검사 |
| 6 Audit/Commit | ledger·참조·한계 검사, S_n+1 발행 | 완결 hash·event batch·checkpoint view |

Maintenance와 Think의 비용으로 생기는 열·source의 논리 시각도 schedule에 포함한다. 죽은 agent를 제거한 뒤 남은 radiation·artifact·기록을 함께 삭제하지 않는다. birth는 다음 tick의 감각·행동에 참가한다. 오래 걸린 QM job 때문에 이미 commit한 과거 phase를 뒤늦게 수정하지 않는다.

### CoupledAdvance의 reference 구성

첫 reference는 정해진 local exchange interval h에서 다음 순서를 사용한다. 시작 상태로 QM/constitutive 요구를 확정하고 결과를 반영한다. 물질·장·수송을 h/2 전진하고, 국소 nuclear/chemical/thermal interaction을 h 전진하며 실제 topology·charge 변화 뒤 필요한 QM 재평가를 한다. 갱신된 상호작용으로 나머지 h/2를 전진하고 flux·residual을 감사한다. 환경의 느린 연산도 같은 ledger에 연결한다.

이는 구현할 operator 순서의 제안이며, nonlinear solve·event quantization·경계 재평가가 있어 자동으로 모든 항에 2차 정확도가 보장되는 것은 아니다. h와 h/2·h/4 reference 비교로 coupled order와 오차를 검증한다. 재평가가 interval 내부 더 이른 순간에 필요하면 h를 잘라 재시도한다. ‘Strang’이라는 이름만으로 검증 gate를 통과하지 않는다.

LifeCommit은 R9의 유효 code/weight/topology adoption도 포함한다. 진행 중 frame·NeuralEpoch·생식 operator의 revision pin은 끝날 때까지 유지하고 새 호출부터 새 revision을 사용한다. 생애학습 결과를 부모의 germline에 자동 반영하지 않는다.

body movement와 charge deposit, Maxwell force와 mechanical work, nuclear deposition과 chemistry는 양방향 interface다. 각 solver가 마지막 tick의 상태만 보고 독립적으로 한 tick을 진행한 후 합산하는 방식은 reference가 아니다. boundary를 여러 worker가 계산하더라도 stage plan과 input revision은 단일하다.

### threading·rollback·저장

Runner control thread가 phase 순서를 소유한다. CPU pool은 disjoint task와 tagged delta, GPU executor는 resident field, QM pool은 immutable patch 계산, I/O와 observer는 완결 snapshot만 처리한다. 이들은 의존성이 없는 일만 겹친다. commit thread가 모든 부동소수점 합을 직렬로 처리해야 한다는 뜻은 아니며 canonical reduction tree로 병렬화할 수 있다.

rollback은 dirty page journal·immutable revision·field checkpoint stage의 조합이다. journal 최대량을 예약하지 못하면 stage를 시작하지 않는다. GPU 일부 buffer만 복구하고 CPU inventory는 유지하는 불완전 rollback은 금지한다. host crash 후에는 마지막 완결 persistent checkpoint로 복구한다.

CIVCT003 manifest는 CT2 항목에 NuclideData, NuclearChannel, RestEnergyConvention, UnitSpec, QuantumMethod/Parameters, Embedding, Kinetics, RadiationTransport, LawField, WorldFrontier, BackendTopology를 추가한다. endian·integer width·array length·compression·checksum은 schema로 명시한다. field/patch raw pointer를 직렬화하지 않는다. CT2 save는 legacy로 열거나 명시적 migration branch를 만들며 새 핵/양자 법칙을 과거 역사에 소급 적용하지 않는다.

**경계:** Engineering Contract는 타입·phase·worker·저장이다. W는 phase에서 적용되는 승인된 법칙이다. P는 CT2의 저수준 명령을 유지한다. E와 U의 데이터 소유권은 분리한다.

## 136 성능 목표·benchmark·최적화 실행 순서

### 측정 가능한 목표

R10에서 ‘모든 물리를 켠1,024만 타일·10만 개체10TPS 달성’을 확정하지 않는다. 이전 Grid-5 목표를 확장 물리 성능으로 재사용하지 않는다. 목표를 작은 통합 reference, subsystem scaling, 생산 통합, 장기 ensemble로 나눈다. hardware·workload·backend·precision이 없는 TPS 수치는 발표하지 않는다.

| benchmark | 고정 workload 예 | 목표/판정 |
| --- | --- | --- |
| CT3-Micro | 64×64,100controller,2,000body cell,핵·QM 작은 fixture | 1TPS 후보 개발 목표; 정확성 선행, 미실측 |
| Controller-Scale | 1만/10만controller,동일 CU와 neural 분포 | ns/CU·edge update·allocator peak; 물리 통합 TPS 아님 |
| Field-Scale | 64/128/512²,같은 fine 해상도·256substep | cell update/s·bytes/update·오차·최대 resident 크기 |
| Chemistry-QM | 16/32/64atom,charge/spin/embedding 변화 | cold/warm SCF p50/p95·실패·job/s·메모리 |
| Nuclear-Scale | 등록된 source와 재고,활성도·산란 경로별 sweep | track/collision/s·분포·queue peak·보존 |
| Integrated-Stress | dense 접촉·wave spread·graph growth·고출생·checkpoint | p99 tick·OOM 복구·비선택적 host 정지 |
| Production-Scale | 승인된 B 모델,큰 지역·희소 active fraction | 목표10TPS에 필요한 유효 부하 범위 실측 |
| Long-Run | 10^6tick 또는 지정 wall budget의 여러 seed | drift·메모리 증가·기술 증거·extinction 보고 |

CT3-Micro의 실제 QM job 수·기준 분자·핵종·source intensity·body topology는 fixture manifest로 고정한다. 계산 부하가 상한을 넘으면 초과 사건을 지우지 않는다. workload 표의 수는 시험 입력 조건이며 세계 규칙의 작은 임의 상한이 아니다. 1TPS도 현재 보장이 아니라 측정해야 할 개발 목표다.

### 시간 예산과 queue 안정성

10TPS 목표는100ms/tick,1TPS는1,000ms/tick이다. 각 phase의 critical-path 예산을 측정치로 배분한다. 예를 들어 QM job이 실측20ms이고8개의 완전히 독립 worker를 쓸 수 있다고 가정하면 이상적 처리량은400job/s,10TPS에서40job/tick이다. 이는 예시 산술이며 실제 xTB 성능값이 아니다. 의존성·메모리·tail·SCF retry가 있으면 더 낮다.

새로운 구조가100job/tick을 요구하면 그 세계는 해당 장비에서10TPS로 계속될 수 없다. 큐에 넣고 ‘물리적으로 이미 계산된 것’처럼 진행하지 않는다. 더 많은 exact cache hit·독립 worker·검증된 B 방법이 없으면 TPS가 낮아져야 한다. deadline miss는 host 진단이며 생물의 작업 실패가 아니다.

render·observer·압축은 simulation과 경쟁한다. MAX profile은 처음부터 headless core benchmark와 비교해 부가 비용을 측정한다. UI 감소만으로 physics가100배 빨라지는 것은 아니다. simulation thread count를 늘려 메모리 포화가 심해지면 줄이는 것이 맞다.

### 병목별 대응 우선순위

| 관측된 병목 | 첫 조치 | 다음 조치·중단 기준 |
| --- | --- | --- |
| DRAM field traffic 포화 | resident SoA·fusion·temporal tile | 검증된 AMR, 추가 bandwidth; dt 몰래 확대 금지 |
| CPU VM branch/dispatch | revision decode·PC batch·inline hot state | SIMD 후보; CU/intent 순서 바뀌면 reject |
| graph pointer/cache miss | arena·adjacency span·spatial reorder | dirty subgraph cache; species pruning 금지 |
| QM queue 증가 | exact cache·job batch·중첩 thread 제거 | patch/embedding 개선 B 검증; 미해결 시 TPS 저하 |
| nuclear track 폭증 | event skip·material/rate cache·state batch | cohort/group B 검증; 미처리 track 삭제 금지 |
| GPU 왕복·launch | persistent state·transfer batch·graph launch | 작업 grain 조정; 작은 kernel은 CPU 재검토 |
| lock·직렬 resolve | shard owner·delta sort·독립 transaction 병렬 | genuine hotspot은 물리 의존성으로 인정 |
| 분산 halo·collective | interior overlap·큰 shard·통신 batch | 강한 scaling 한계; node 추가 중단 |
| checkpoint/observer | dirty delta·bounded read snapshot·표본 분석 | 저장 bandwidth 확보·wall time backpressure |
| 메모리 peak | scratch 재사용·정확한 공유·pin 수 제한 | capacity 확장·checkpoint 복구 정지 |

### 구현 순서

E0는 작은 strict reference와 보존 ledger다. E1은 CPU chunk·SoA·arena·allocation 제거·hot/cold 분리다. E2는 sparse nuclear·정확한 chemistry cache·QM adapter와 profiling이다. E3는 GPU resident field와 필요한 감각/flux 전송이다. E4는 kernel fusion·temporal tiling·batch neural과 VM 최적화다. E5는 AMR·cohort·coarse chemistry 등 B 경로의 과학적 검증이다. E6는 다중 GPU·분산·장기 ensemble다.

순서를 지키는 이유는 기준 의미가 없는 고속 구현을 나중에 교정하기 어렵기 때문이다. 각 단계에서 실측으로 다음 최적화 대상을 결정한다. 처음부터 모든 GPU·MPI·QM backend를 완성하려고 하면 통합 오류를 구분하기 어렵다. pure CPU reference는 유지하지만 최종10만 개체 확장 물리를 순수 CPU로 반드시 해결한다는 제약은 두지 않는다.

**경계:** Engineering Contract는 workload·목표·측정·개발 gate다. P를 성공시키는 benchmark script는 자연 진화 증거가 아니다. E는 별도 seed 실험으로 검증한다. U는 현재 달성 규모와 제한을 정직하게 표시한다.

## 137 R10 추가 기대 사례 Q01..Q80

아래80개는 아직 실행되지 않았다. 이전325개와 합친405개는 서로 다른 ruleset의 expected contract다. 앞선 방식이 교체된 사례는 해당 legacy profile로 검증한다. source code·물성 bundle·reference solver가 없는 현재 문서 검사는 이 물리 시험의 통과가 아니다.

### 핵·에너지·수송

| ID | 시험 입력 또는 상황 | 반드시 나와야 하는 결과 |
| --- | --- | --- |
| Q01 | 화학 반응 전후 같은 isotope 재고 | isotope별 핵 수·전하 보존 |
| Q02 | 핵 channel에서 Z가 다른 생성물 | 원소별 수는 변해도 channel 핵자·전하 장부 일치 |
| Q03 | Q=10q의 합성 시험 channel,입자6q·recoil4q | 전체 증가10q; heat10q 추가 지급 없음 |
| Q04 | 음의 Q=-3q,입사 에너지2q인 threshold fixture | 지원 모델상 불가능한 반응은 실행되지 않음 |
| Q05 | atomic mass와 bare mass 혼합 데이터 | startup convention 검사 실패 |
| Q06 | daughter nuclide가 registry에 누락 | 반응군 지원 선언 거부,임의 원소 생성 없음 |
| Q07 | 중성미자 carrier가 경계로 나감 | escape 장부 증가; 지역 열로 중복 변환 없음 |
| Q08 | slab 밖으로 나간 high-energy packet | 위치/에너지 boundary 기록 후 local stock에서 제거 |
| Q09 | 같은 경로를1/4개 host chunk로 분할 | exact transport의 optical depth·RNG ordinal·결과 동일 |
| Q10 | cohort100개 핵을37/63으로 분할 | 핵 수100 유지; 총 energy·charge residual 유지 |
| Q11 | 높은 활성도로 host event queue 초과 | wall time 증가 또는 전체 stage 정지; 사건 drop 없음 |
| Q12 | transmutation으로 기존 valence 불가능 | chemistry/QM 재평가 전 정상 bond로 사용하지 않음 |

### 국소 양자와 결합

| ID | 시험 입력 또는 상황 | 반드시 나와야 하는 결과 |
| --- | --- | --- |
| Q13 | 같은 geometry,다른 charge/spin | QM cache key가 구별됨 |
| Q14 | low whole100q,low patch20q,QM patch15q | subtractive energy95q,115q가 아님 |
| Q15 | 같은 atom이 두 patch에 포함 | canonical merge/coupling,이중 force·heat 없음 |
| Q16 | 경계 link hydrogen 생성 | 실제 H 재고·생체 inventory 증가0 |
| Q17 | low→QM→low 왕복 | 승인 tolerance 안에서 energy·charge·momentum 보존 |
| Q18 | SCF cold start와 restart가 다른 가지 선택 | restart 상태를 hash/save에 포함하거나 reference 고정 |
| Q19 | patch A/B 완료 순서가 뒤바뀜 | 같은 tagged canonical commit 결과 |
| Q20 | SCF retry 전부 실패 | StageFailure와 rollback; 임의 물성 채택 없음 |
| Q21 | 낮은 product energy지만 큰 장벽 fixture | ground state 결과만으로 즉시 반응하지 않음 |
| Q22 | method가 지원하지 않는 원소/전하 | OutsideDomain; 임의 nearest-element 치환 없음 |
| Q23 | 내부3D 분자 좌표 변경 | macro body의 slab 통과 권한이 생기지 않음 |
| Q24 | GPU/CPU QM 실행 시간이 다름 | solver 결과가 같다면 세계 CU·에너지 비용 동일 |

### LawField

| ID | 시험 입력 또는 상황 | 반드시 나와야 하는 결과 |
| --- | --- | --- |
| Q25 | χ를 바꿔 matter potential 감소 | -∂H_matter/∂χ 반작용과 field energy에 정산 |
| Q26 | χ damping으로2q 감소 | 지정 열 reservoir에2q,총energy 유지 |
| Q27 | coefficient bounded map 경계 접근 | k_min..k_max와 stability domain 유지 |
| Q28 | SetLaw opcode bytes를 실행 | 해당 world primitive가 존재하지 않음 |
| Q29 | c/핵 질량을 바꾸고 기존 parameter bundle 유지 | manifest/domain 검증 실패 |
| Q30 | LawField OFF의 BaseCT3 replay | hidden χ update·extra cost 없음 |
| Q31 | 다음 run의 PhysicsSpec를 변경 | 별도 run lineage; 같은 세계의 자연 발명으로 표시 안 함 |
| Q32 | 모든 agent 멸종 | χ나 상수를 자동 조절해 구제하지 않음 |

### 공간·유한 자원

| ID | 시험 입력 또는 상황 | 반드시 나와야 하는 결과 |
| --- | --- | --- |
| Q33 | 새 region materialization | InitialDomainRealization에만 초기 inventory 기록 |
| Q34 | tick1000에 처음 방문한 decay region | tick0 상태를 그대로 제공하지 않음 |
| Q35 | 카메라 이동 없이 파동이 먼 chunk에 도달 | 물리 의존성으로 chunk 활성화 |
| Q36 | agent가 없는 뜨거운 chunk | 열 변화가 있으면 dormant 처리 안 함 |
| Q37 | dormancy의 boundary hash가 변경 | certificate 무효화,정해진 논리 시각에 wake |
| Q38 | elliptic long-range interaction 활성 | 유한 cone만으로 외부 influence 삭제 안 함 |
| Q39 | 같은 initial coordinate를 다른 host 순서로 실현 | 초기조건·UID·재고 동일 |
| Q40 | signed i64 coordinate overflow 접근 | checked halt; 반대편 wrap teleport 없음 |
| Q41 | host RAM 부족,비싼 genome 다수 | 특정 lineage 제거 없이 whole-stage ResourceLimit |
| Q42 | SSD page fault를 인위적으로 지연 | world time과 survival 결과는 변하지 않음 |
| Q43 | shared code 소유자 한 명의 실제 매체 파괴 | 다른 소유자는 유지,파괴된 소유자의 접근은 소실 |
| Q44 | coarse archive에 없는 과거 미세구조 조회 | unknown/재계산 필요 표시; exact history 발명 없음 |

### CPU·GPU·분산·근사

| ID | 시험 입력 또는 상황 | 반드시 나와야 하는 결과 |
| --- | --- | --- |
| Q45 | profiling ON/OFF 동일 backend | state hash 동일 |
| Q46 |1/2/8 CPU worker와 다른 task stealing | exact path 결과 동일 |
| Q47 | host spatial reorder·arena compaction | UID·주소 권한·행동 순서 보존 |
| Q48 | GPU neural kernel을 즉시 완료 |617CU 예제의32CU/tick 지연20tick 이상 유지 |
| Q49 | neural weights가 다른 동일 topology 개체 | state/학습이 host 공유로 섞이지 않음 |
| Q50 | VM revision adoption 뒤 decode cache hit | 이전 opcode로 실행되지 않음 |
| Q51 | field temporal tile k=4,radius1 | 필요한 중간 source와 dependency halo를 포함 |
| Q52 | FP32 또는 fast-math 전환 | A로 자동 승인 안 함; 별도 B/Backend gate |
| Q53 | async DMA 중 source buffer 재사용 시도 | lifetime/fence 계약으로 방지 |
| Q54 | host가 stale GPU mirror에 수정 시도 | revision/owner 검사 실패 |
| Q55 | 같은 face를두 shard가 업데이트 | canonical flux 한 번만 debit/credit |
| Q56 | agent shard migration 중 pending NCYCLE | epoch·CU·frozen input·weight revision 유지 |
| Q57 | GPU/node 수 변경으로 reduction 순서 변화 | exact 재현 검증 또는 다른 backend 선언 |
| Q58 | 부하가 커져 auto-tuner 작동 | block/thread만 조정; dt·물성·tolerance 불변 |
| Q59 | 약한 장거리 신호가 coarse threshold 미만 | 임의 삭제 금지; 검증된 모델 조건 따라 처리 |
| Q60 | 미지원 구조를 surrogate가 낮은 energy로 예측 | validity 실패 검출과 reference/domain 처리 |

### 저장·재현·관찰

| ID | 시험 입력 또는 상황 | 반드시 나와야 하는 결과 |
| --- | --- | --- |
| Q61 | QM restart와 nuclear residual이 있는 save | load 후 이어진 미래가 같은 backend에서 일치 |
| Q62 | checkpoint 중 field dirty page 증가 | pin/journal 포함 peak budget 준수 |
| Q63 | manifest commit 전 host crash | 마지막 완결 checkpoint만 유효 |
| Q64 | 비동기 shard save가 서로 다른 tick | 하나의 consistent checkpoint로 등록 안 함 |
| Q65 | 전체100k개체,16tick/32B event,10TPS | raw 산술2MB/s·172.8GB/day |
| Q66 |64GiB의2%/10초 dirty | raw 산술131.072MiB/s |
| Q67 | render consumer가10배 느려짐 | 오래된 render snapshot만 drop,world hash 동일 |
| Q68 | observer clustering을 OFF | 세계 결과 동일,해석만 없음 |
| Q69 | 필수 provenance queue 초과 | backpressure/정지,무음 데이터 누락 없음 |
| Q70 | input log 일부 손상 | 무결성 실패; 다른 역사로 정상 재생 안 함 |
| Q71 | CT2 save를 CT3로 열기 | legacy 또는 명시 migration branch 요구 |
| Q72 | viewer500x 요청,실제20TPS | 실제2x 표시; physics dt500배 변경 없음 |

### 통합 성능과 연구 정직성

| ID | 시험 입력 또는 상황 | 반드시 나와야 하는 결과 |
| --- | --- | --- |
| Q73 |3200²tile,16cell/tile,256step |41,943,040,000update/tick |
| Q74 | Q73에서96B/update 가정 |4,026.53184GB/tick; 가정을 함께 표시 |
| Q75 |1,024²cell shard,h=2,6f64 halo | 단순 payload393,216B; 추가항 제외 명시 |
| Q76 |20ms QM job,8worker,10TPS 가정 | 이상적40job/tick; 실측값으로 소개 안 함 |
| Q77 |10% phase를10배 가속 | 전체 이론상 약1.10배,10배 아님 |
| Q78 | 시험용 손작성 에너지 이용 구조 성공 | R1/R2 witness; R3 발명으로 올리지 않음 |
| Q79 | 학습·신경을 포함한 CT3 장기 run | R9 germline/somatic/culture·유료 계산 계약 유지 |
| Q80 | 문서/PDF 검사 통과,simulator 미구현 | 물리·성능 시험405개 통과라고 보고하지 않음 |

**경계:** Engineering Contract는 expected test와 증거 기록이다. P는 시험 때문에 늘어나지 않는다. E의 연구 성공은 실행 표본으로만 판정한다. U는 미실행·통과·실패·미지원 상태를 구별한다.

## 138 구현 전 동결 항목·최종 판단·근거

### 이번에 확정한 설계 결정

핵종과 원소를 분리하고 핵 반응 에너지를 endpoint ledger에서 계산한다. 양자화학은 국소 근사 solver와 보존형 embedding으로 도입한다. 물리법칙 진화는 고정 meta-law 아래 LawField라는 별도 가상 실험으로 제한한다. 세계는 희소한 확장 좌표와 실제 시간 진행 계약을 가진다. 신경망·생애학습·유전 operator·물리 기억은 유지한다. 성능은 reference 의미를 기준으로 CPU/GPU/분산/저장과 A/B/C 최적화를 구분한다.

새로운 자유도가 장기적으로 반드시 유용한 기술을 낳는다는 보장은 없다. 단순한 발명도 생존 이득을 주는 중간 단계, 희귀 기능을 잃지 않는 적절한 근사, 충분한 시간·지역·표본이 필요하다. 복잡한 물리를 추가할수록 한 seed를 오래 보는 대신 여러 seed와 축약 profile을 비교해야 할 수도 있다. 세계를 흥미롭게 보이게 하려고 자동으로 발명·직업·장치를 심지 않는다.

### 아직 닫히지 않은 gate

| gate | 반드시 납품할 구체 산출물 | 없을 때 허용되지 않는 주장 |
| --- | --- | --- |
| D-Unit | tile/tick/mass/energy/charge 환산·rest convention·overflow 범위 | 현실 유사 핵/EM/역학이 이미 일관되게 결합됨 |
| D-Nuclear | registry·도달 생성물 닫힘·channel 데이터·수송 유효 범위 | 모든 핵반응 지원 |
| D-Quantum | method/parameter hash·element/charge/spin domain·reference fixtures | 일반적인 모든 양자화학 지원 |
| D-Coupling | QM-low zero point·force·charge·열·kernels·mesh 전환 식 | moving patch·반응·파동 결합 구현만 하면 무수정 완료 |
| D-LawField | H·χ 경계·안정성·반작용·무입력 energy 시험 | 법칙 진화가 보존적으로 완성됨 |
| D-World | frontier proof·boundary/catch-up 데이터·dormancy 판정 | 무한 세계의 모든 지역이 현재 tick까지 진행됨 |
| D-Engine | strict reference·layout/ABI·rollback·CPU/GPU kernels·profile 실측 | 대규모 통합10TPS 또는500x 달성 |
| D-Science | 자연 seed run·R1..R4 증거·근사 수렴·실패 보고 | 문명·기술 축적·개방형 진화 보장 |

이 표는 ‘나중에 알아서 구현’ 목록으로 덮어두기 위한 것이 아니다. 특히 UnitSpec과 보존형 coupling 식은 실제 solver 개발의 선행 연구다. 현재 문서만으로 모든 물성·경계 연산까지 기계적으로 옮기는 완전 동결 구현 명세라고 하면 부정확하다. 원래 핵심 구조보다 이번에 넓힌 과학적 범위의 미검증 항목이 더 많다. 설계 분량이 늘어났다고 완성도가 같은 비율로 올라가지는 않는다.

### 개발과 연구의 진행 기준

먼저 CT3의 작은 물리 reference와 data bundle을 고정하고 Q01..Q44의 핵·양자·세계 의미를 검증한다. 이어 Q45..Q72의 최적화·저장·결정론을 구현한다. 실측으로 가장 큰 병목 하나씩 제거하며 B 근사는 reference 대조 후 넣는다. Q73..Q80으로 수치 예산과 연구 증거를 감사한다. 소규모 기능 검증 없이 모든 물리와10만 개체를 한 번에 결합하지 않는다.

v0/Grid-5는 원래 생태 행동 baseline으로 남는다. v1/Matter·Life는 지속 구조·형태·문화의 baseline이다. v2-Continuum/Base-CT3는 이번 핵·국소 양자·확장 환경과 신경/학습을 포함하는 목표 제품 profile이다. v3/LawField와 더 넓은 비평형 전자·고에너지·공간 자유도는 별도 검증 연구다. 구현 순서가 세계의 기술 시대 순서는 아니다.

### 주요 근거와 사용 범위

| 1차 자료 | 이 문서에서 참고한 내용 | 이 자료가 보증하지 않는 것 |
| --- | --- | --- |
| IAEA LiveChart / API | 핵종·붕괴 데이터 출처와 조회 구조 | CT3 축약 단위·결합 모델 |
| Geant4 Physics Reference Manual | 반응별 모델·방사성 붕괴·수송 참조 | CT3 자체 solver의 정확성/성능 |
| xTB documentation / C API / ONIOM | 국소 근사 전자구조·interface·subtractive 결합 참고 | 모든 원소·반응·moving boundary의 정확성 |
| DFTB+ project | 대체 근사 전자구조 라이브러리 경로 | CT3 내 GPU speedup |
| CUDA Best Practices | profiling·data locality·transfer batching | 본 세계의 특정 배수 가속 |
| AMReX documentation | AMR·MPI·GPU 계층 병렬 설계 | 본 세계의 보존·결정론 |
| Roofline 원 논문 | 대역폭·연산량 기반 성능 상한 사고방식 | 실측 없는 처리율 |

[IAEA LiveChart](https://www-nds.iaea.org/relnsd/vcharthtml/VChartHTML.html), [Geant4 Physics Reference](https://geant4.web.cern.ch/documentation/dev/prm_html/PhysicsReferenceManual/index.html), [xTB](https://xtb-docs.readthedocs.io/en/latest/), [DFTB+](https://dftbplus.org/), [CUDA](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html), [AMReX](https://amrex-codes.github.io/amrex/docs_html/Introduction.html), [Roofline](https://escholarship.org/uc/item/5tz795vq). 2026-09-29 확인. 실행 시에는 이 web 문서의 최신판이 아니라 승인한 데이터·API·backend 버전을 pin한다.

**경계:** W의 지원 범위와 공학적 완료를 분리한다. P는 끝까지 낮은 수준으로 남는다. E는 가능성·효용·자연 발견·누적의 증거 단계로 확인한다. U는 설계·구현·실측·연구 결과를 혼동하지 않게 표시한다.
