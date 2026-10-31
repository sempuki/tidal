// tidal-rules: REF-3
// tidal-expect: reject
// tidal-error-clang: rvalue reference to type .Base. cannot bind to lvalue
// tidal-error-gcc: cannot bind rvalue reference of type .Base&&. to lvalue
// tidal-error-edg: rvalue reference cannot be bound to an lvalue
// tidal-note: Gap review 2026-10-30.
struct Base {};
struct Derived : Base {};
Derived d;
Base&& r = d;
