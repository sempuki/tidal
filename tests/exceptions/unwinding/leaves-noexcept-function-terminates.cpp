// tidal-rules: UNW-5
// tidal-expect: run-fail
void risky() noexcept { throw 0; }
int main() {
  try { risky(); }
  catch (int) { return 0; }
  return 0;
}
