// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-error-clang: cannot capture any entity
// tidal-error-gcc: default argument
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md). A lambda in a default argument of a function declared in a block is not inside a block. Such a lambda has no capture default and no named capture, so the capture of n is rejected.
int main() {
  int n = 1;
  void g(int = [n] { return n; }());
  return 0;
}
