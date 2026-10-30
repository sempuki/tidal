// tidal-rules: ENUM-3
// tidal-expect: reject
enum E { A, B };
void f() {
  E e = A;
  ++e;
}
