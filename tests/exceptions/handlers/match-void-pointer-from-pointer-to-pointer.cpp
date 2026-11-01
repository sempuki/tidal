// tidal-rules: TRY-2
// tidal-expect: run
int value;
int* slot = &value;
int main() {
  try { throw &slot; }
  catch (const volatile void* p) { return p == &slot ? 0 : 1; }
  return 2;
}
