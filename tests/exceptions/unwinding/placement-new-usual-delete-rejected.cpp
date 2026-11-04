// tidal-rules: UNW-11
// tidal-expect: reject
// tidal-error-clang: refers to non-placement .operator delete.
// tidal-error-gcc: selects non-placement .operator delete.
// tidal-error-edg: operator delete
// tidal-note: The placement allocation function takes one extra size argument. The usual sized deallocation function of the class has the same parameters, so the cleanup after a throw would select it.
using Size = decltype(sizeof 0);
struct S { static void* operator new(Size n, Size extra); static void operator delete(void* p, Size n); S(); };
S* f() { return new (5) S; }
