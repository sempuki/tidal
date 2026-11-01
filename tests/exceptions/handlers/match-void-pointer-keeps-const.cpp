// tidal-rules: TRY-2
// tidal-expect: run
const int limit = 3;
int main() {
  try { throw &limit; }
  catch (void*) { return 1; }
  catch (const void* p) { return p == &limit ? 0 : 2; }
  return 3;
}
