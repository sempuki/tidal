// tidal-rules: SW-3
// tidal-expect: reject
// tidal-error-clang: value of type .int. is not implicitly convertible to .E
// tidal-error-gcc: could not convert .0. from .int. to .E
// tidal-error-edg: has type .int. instead of the required .E. type
// tidal-note: Gap review 2026-10-30.
enum class E { A, B };
void f(E e) {
  switch (e) { case 0: break; }
}
