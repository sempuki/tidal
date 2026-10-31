// tidal-rules: UNW-5
// tidal-expect: run-fail
void f() { throw 0; }
int main() {
  try { f(); }
  catch (long) { return 0; }
  return 0;
}
