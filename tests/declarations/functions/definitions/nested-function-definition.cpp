// tidal-rules: FN-9
// tidal-expect: reject
// tidal-error-clang: function definition is not allowed here
// tidal-error-gcc: function-definition is not allowed here
// tidal-note: Gap review 2026-10-30.
void outer() {
  void inner() {}
}
