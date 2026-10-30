// tidal-rules: ATTR-16
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct Empty {};
struct Two { [[no_unique_address]] Empty a; [[no_unique_address]] Empty b; };
int main() {
  Two t;
  return static_cast<void*>(&t.a) != static_cast<void*>(&t.b) ? 0 : 1;
}
