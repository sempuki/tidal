// tidal-rules: IF-17
// tidal-expect: reject
void f(int k) {
  switch (k) {
    case 0:
      if constexpr (true) {
    case 1:;
      }
  }
}
