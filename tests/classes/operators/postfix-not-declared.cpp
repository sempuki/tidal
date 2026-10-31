// tidal-rules: OPR-4
// tidal-expect: reject
// tidal-error-clang: cannot increment value of type .C
// tidal-error-gcc: no .operator\+\+\(int\). declared for postfix
// tidal-error-edg: no operator .\+\+. matches
struct C { C& operator++(); };
void f(C c) { c++; }
