// tidal-rules: CLS-26
// tidal-expect: reject
// tidal-error-clang: class member cannot be redeclared
// tidal-error-gcc: is ambiguous|cannot be overloaded with
// tidal-error-edg: both have ref-qualifi
// tidal-note: Q-0027. The program also calls f on an lvalue. Clang, GCC 14, and EDG reject the declarations. GCC 16 rejects the call as ambiguous.
struct S {
  void f() &;
  void f();
};
void g(S& s) { s.f(); }
