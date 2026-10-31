// tidal-rules: JMP-13
// tidal-expect: reject
// tidal-error-clang: destructor .~S. should not return a value
// tidal-error-gcc: returning a value from a destructor
// tidal-error-edg: constructor or destructor may not return a value
// tidal-note: Gap review 2026-10-30.
struct S {
  ~S() { return 1; }
};
