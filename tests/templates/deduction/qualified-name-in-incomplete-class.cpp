// tidal-rules: DED-24
// tidal-expect: run
// tidal-note: I5-9, I5 reduction r1-incomplete-class-qualified-name.cpp. Opaque is incomplete, so it has no member named type, and the substitution of T::type fails (DED-24). The template adds no candidate, and the call selects pick(...).
struct Opaque;
template <class T> int pick(T*, typename T::type = 0) { return 1; }
int pick(...) { return 2; }
int main() {
  Opaque* p = nullptr;
  return pick(p) == 2 ? 0 : 1;
}
