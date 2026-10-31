// tidal-rules: COND-5
// tidal-expect: reject
// tidal-error-clang: conditional expression is ambiguous
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: ambiguous .\?. operation
struct B;
struct A { A(); A(const B&); };
struct B { B(); B(const A&); };
void f(bool c) { auto r = c ? A{} : B{}; }
