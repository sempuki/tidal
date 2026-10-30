// tidal-rules: CTOR-5
// tidal-expect: accept
struct Base { Base(int) {} };
using Alias = Base;
struct S : Base { S() : Alias(1) {} };
