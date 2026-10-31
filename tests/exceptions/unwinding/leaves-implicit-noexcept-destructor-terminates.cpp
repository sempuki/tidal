// tidal-rules: UNW-5
// tidal-expect: run-fail
struct D { ~D() { throw 0; } };
int main() {
  try { D d; }
  catch (int) { return 0; }
  return 0;
}
