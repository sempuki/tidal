// tidal-rules: NEW-2
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-error-gcc: must have integral or enumeration type
// tidal-note: Q-0036. The size 2.0 converts to 2 with no loss. Clang and EDG convert it, with no diagnostic under -pedantic-errors. GCC rejects every floating-point size.
int main() {
  int* p = new int[2.0];
  p[1] = 0;
  delete[] p;
  return 0;
}
