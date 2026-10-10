# Gene: 작업 규칙

사용자가 정한 고정 요구사항이다. 매 작업에 적용한다.

- 큰 작업을 끝낼 때마다 전체 완성률을 보고한다. 기준과 표는 `docs/NEXT.md`의 "완성률"을 갱신해 쓴다.
- 화면(`gene serve` viewer)은 항상 60 FPS 이상을 목표로 한다. 화면을 바꾸면 FPS를 측정해 결과를 함께 보고한다.
  - 측정: `node` + Playwright로 페이지의 `window.__fps()`를 읽는다. 이 컨테이너에는 GPU가 없어 SwiftShader(소프트웨어)로 잰다. 실험 batch가 CPU를 쓰고 있으면 잠시 SIGSTOP/SIGCONT로 멈추고 잰다.
  - 매 frame 무거운 일을 두지 않는다. 정적 지형은 bake하고, 개체는 point sprite 한 번의 draw로 그린다. 주기 작업은 frame들에 나눈다.
- 그래픽 텍스처는 직접 만들어 보기 좋게 유지한다(절차적 생성, `assets/viewer.html`).
- 응답은 한국어로 한다.
