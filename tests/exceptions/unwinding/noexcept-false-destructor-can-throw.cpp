// tidal-rules: UNW-6
// tidal-expect: run
struct D { ~D() noexcept(false) { throw 3; } };
int main() {
  try { D d; }
  catch (int v) { return v == 3 ? 0 : 1; }
  return 1;
}
