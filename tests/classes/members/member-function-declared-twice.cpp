// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: class member cannot be redeclared
// tidal-error-gcc: cannot be overloaded with
// tidal-error-edg: invalid redeclaration of member function
// tidal-note: Gap review 2026-10-30.
struct S {
  void f();
  void f();
};
