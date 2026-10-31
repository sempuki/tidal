// tidal-rules: UNW-5
// tidal-expect: run-fail
void inner() { throw 0; }
void outer() noexcept { inner(); }
int main() {
  try { outer(); }
  catch (int) { return 0; }
  return 0;
}
