// tidal-rules: THR-1
// tidal-expect: run
struct S { S() {} S(const S&) { throw 7; } };
S global;

int main() {
  try { throw global; }       // the copy into the exception object throws 7
  catch (int v) { return v == 7 ? 0 : 1; }
  catch (S&) { return 2; }
  return 3;
}
