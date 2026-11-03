// tidal-rules: FN-9
// tidal-expect: reject
// tidal-error-clang: function definition is not allowed here
// tidal-error-gcc: function-definition is not allowed here
// tidal-error-edg: expected a ";"
// tidal-note: The function definition is inside the body of a member function.
struct S {
  void m() {
    int inner() { return 1; }
  }
};
