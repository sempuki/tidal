// tidal-rules: PFX-13
// tidal-expect: reject
// tidal-error-clang: cannot increment expression of enum type
// tidal-error-gcc: no .operator\+\+\(int\). declared for postfix
// tidal-error-edg: operation on an enumerated type
// tidal-note: Gap review 2026-10-31.
enum E { a, b };
void f(E e) { e++; }
