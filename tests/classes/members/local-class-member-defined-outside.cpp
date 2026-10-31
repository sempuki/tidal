// tidal-rules: CLS-28
// tidal-expect: reject
// tidal-error-clang: function definition is not allowed here
// tidal-error-gcc: qualified-id in declaration
// tidal-error-edg: qualified name is not allowed
// tidal-note: Gap review 2026-10-30.
void f() {
  struct L { void g(); };
  void L::g() {}
}
