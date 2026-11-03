// tidal-rules: CTOR-16
// tidal-expect: reject
// tidal-error-clang: delegating constructor must appear alone
// tidal-error-gcc: constructor delegation follows mem-initializer
// tidal-error-edg: delegating constructor cannot have other mem-initializers
// tidal-note: A base initializer comes before the delegation.
struct Base { Base(int) {} };
struct S : Base {
  S(int x) : Base(x) {}
  S() : Base(0), S(1) {}
};
