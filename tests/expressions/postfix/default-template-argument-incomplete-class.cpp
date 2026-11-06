// tidal-rules: PFX-11 CLS-2
// tidal-expect: reject
// tidal-error-clang: incomplete type
// tidal-error-gcc: incomplete type
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md). Q-0406. The default template argument of a member template uses sizeof(S). Clang and GCC treat S as incomplete there. The review reads the class as complete in a default template argument.
struct S {
  template <int N = sizeof(S)> int f() { return N; }
  int pad[3];
};
int main() { return S{}.f() == sizeof(S) ? 0 : 1; }
