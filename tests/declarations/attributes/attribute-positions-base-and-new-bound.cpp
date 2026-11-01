// tidal-rules: ATTR-19
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-error-gcc-14: expected class-name before
// tidal-note: Q-0177
struct Base {};
struct Derived : [[tidal::note]] Base {};
int make() {
  int* list = new int[3] [[tidal::note]];
  delete[] list;
  return 0;
}
