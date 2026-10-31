// tidal-rules: UNW-5
// tidal-expect: run-fail
struct D { ~D() noexcept(false) { throw 1; } };
int main() {
  try { D d; throw 0; }
  catch (int) { return 0; }
  return 0;
}
