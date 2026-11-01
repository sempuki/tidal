// tidal-rules: THR-12
// tidal-expect: run
struct Fragile { ~Fragile() noexcept(false) { throw 7; } };
int main() {
  int r = 0;
  try {
    try { throw Fragile{}; }
    catch (Fragile&) { r = 1; }
    r += 100;
  } catch (int v) { r += v; }
  return r == 8 ? 0 : 1;
}
