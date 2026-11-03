// tidal-rules: LOOP-5
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool
// tidal-error-gcc: could not convert .* to .bool
// tidal-error-edg: must have bool type
// tidal-note: The condition of a for statement is a scoped enumerator.
enum class State { idle };
void f() {
  for (; State::idle;) {}
}
