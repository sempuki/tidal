// tidal-rules: ACC-14
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
// tidal-note: Gap review 2026-10-31.
struct A {};
struct B : private A {};
struct C : B { void f() { A* p = nullptr; (void)p; } };
