// tidal-rules: SB-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct F { int bits : 3; int other; };
int main() {
  F f{1, 2};
  auto& [b, o] = f;
  b = 2;
  return f.bits == 2 ? 0 : 1;
}
