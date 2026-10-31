// tidal-rules: LIST-16
// tidal-expect: reject
// tidal-error-clang: chosen constructor is explicit in copy-initialization
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: no default constructor exists for class .E
// tidal-note: Gap review 2026-10-31.
struct E { explicit E() {} };
struct A { int v; E e; };
A a{1};
