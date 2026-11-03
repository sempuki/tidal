// tidal-rules: CE-11
// tidal-expect: reject
// tidal-error-clang: not allowed in a converted constant expression
// tidal-error-gcc: in a converted constant expression
// tidal-error-edg: has type .double. instead of the required
int pick(int v) {
  switch (v) {
  case 1.5:
    return 1;
  }
  return 0;
}
