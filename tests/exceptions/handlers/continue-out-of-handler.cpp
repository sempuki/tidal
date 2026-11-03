// tidal-rules: TRY-15
// tidal-expect: run
int count = 0;
struct T { ~T() { ++count; } };
int main() {
  int i = 0;
  for (; i < 3; ++i) {
    try { throw i; }
    catch (int) { T t; continue; }  // t ends at each continue
    return 1;
  }
  return count == 3 && i == 3 ? 0 : 2;
}
