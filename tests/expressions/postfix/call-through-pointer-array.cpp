// tidal-rules: PFX-7
// tidal-expect: run
// tidal-note: The pointers are elements of an array, and a pointer is dereferenced twice.
int inc(int v) { return v + 1; }
int neg(int v) { return -v; }
int main() {
  int (*table[2])(int) = {inc, neg};
  int (*p)(int) = table[1];
  return table[0](4) == 5 && (*table[1])(4) == -4 && (**p)(2) == -2 ? 0 : 1;
}
