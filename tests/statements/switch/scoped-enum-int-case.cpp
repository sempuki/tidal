// tidal-rules: SW-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
enum class E { A, B };
void f(E e) {
  switch (e) { case 0: break; }
}
