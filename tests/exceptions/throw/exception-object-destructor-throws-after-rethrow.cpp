// tidal-rules: THR-12
// tidal-expect: run
// tidal-note: The inner handler rethrows, so the outer handler is the last one for the object. The destructor's exception leaves the end of the outer handler.
struct Fragile { ~Fragile() noexcept(false) { throw 7; } };
int main() {
  int r = 0;
  try {
    try {
      try { throw Fragile{}; }
      catch (Fragile&) { r = 1; throw; }
    } catch (Fragile&) { r += 10; }
    r += 100;
  } catch (int v) { r += v; }
  return r == 18 ? 0 : 1;
}
