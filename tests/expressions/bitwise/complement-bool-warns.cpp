// tidal-rules: BIT-7
// tidal-expect: reject
// tidal-flags: -Werror -Wbool-operation
int f(bool b) {
  return ~b;
}
