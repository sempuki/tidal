// tidal-rules: IF-13
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
template <class T>
void f(T) {
  if constexpr (false) {
    never_declared();
  }
}
