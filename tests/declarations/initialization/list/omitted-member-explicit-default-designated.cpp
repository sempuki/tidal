// tidal-rules: LIST-16
// tidal-expect: reject
// tidal-error-clang: chosen constructor is explicit in copy-initialization
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: no default constructor exists for class .E
// tidal-note: A designated initializer gives a value to the first member only. The omitted member's class has an explicit default constructor.
struct E { explicit E() {} };
struct A { int v; E e; };
A a{.v = 1};
