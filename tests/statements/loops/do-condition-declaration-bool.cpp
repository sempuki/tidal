// tidal-rules: LOOP-6
// tidal-expect: reject
// tidal-error-clang: expected .\(. for function-style cast
// tidal-error-gcc: expected primary-expression before .bool
// tidal-error-edg: type name is not allowed
// tidal-note: The condition declares a bool with an initializer.
void f() {
  do ; while (bool done = true);
}
