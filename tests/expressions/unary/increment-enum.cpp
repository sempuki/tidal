// tidal-rules: UN-11
// tidal-expect: reject
enum E { A, B };
void f() {
  E e = A;
  ++e;
}
