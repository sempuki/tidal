// tidal-rules: UNW-11
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: refers to non-placement .operator delete.
// tidal-error-gcc: selects non-placement .operator delete.
// tidal-error-edg: refers to non-placement function "operator delete
// tidal-note: The program declares a global placement allocation function with one extra size parameter. The global usual sized deallocation function has the same parameters, so the cleanup after a throw from the constructor of S would select it. Clang 18 accepts; Clang 22, GCC, and EDG reject.
using Size = decltype(sizeof 0);
void* operator new(Size n, Size extra);
struct S { S(); };
S* f() { return new (5) S; }
