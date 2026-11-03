// tidal-rules: TRY-2
// tidal-expect: run
struct B { int m; };
int main() {
  try { throw nullptr; }
  catch (int B::* p) { return p == nullptr ? 0 : 1; }
  catch (...) { return 2; }
  return 3;
}
