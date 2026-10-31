// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: static and non-static member functions with the same parameter types cannot be overloaded
// tidal-error-gcc: cannot be overloaded with
// tidal-error-edg: invalid redeclaration of member function
// tidal-note: Gap review 2026-10-31.
struct S { void f(); static void f(); };
