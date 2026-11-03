// tidal-rules: IF-23
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .int \*. with an rvalue of type .double
// tidal-error-gcc: cannot convert .double. to .int\*.
// tidal-error-edg: cannot be used to initialize an entity of type .int \*
// tidal-note: The discarded statement initializes a pointer from a double.
void f() {
  if constexpr (false) {
    int* p = 1.5;
  }
}
