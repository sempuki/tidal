// tidal-rules: CE-25
// tidal-expect: reject
// tidal-error-clang: variable length arrays in C\+\+
// tidal-error-gcc: forbids variable length array
// tidal-error-edg: expression must have a constant value
void fill(int count) {
  int table[count];
  table[0] = 0;
}
