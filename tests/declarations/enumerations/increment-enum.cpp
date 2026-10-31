// tidal-rules: ENUM-16
// tidal-expect: reject
// tidal-error-clang: cannot increment expression of enum type
// tidal-error-gcc: no match for .operator\+\+
// tidal-error-edg: operation on an enumerated type requires
enum E { A, B };
void f() {
  E e = A;
  ++e;
}
