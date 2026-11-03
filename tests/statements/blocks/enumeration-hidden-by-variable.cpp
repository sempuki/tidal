// tidal-rules: STMT-10
// tidal-expect: reject
// tidal-error-clang: must use .enum. tag to refer to type .Mode
// tidal-error-gcc: expected .;. before .m
// tidal-error-edg: expected a .;.
// tidal-note: The hidden name is an enumeration, and the declaration names it without enum.
int f() {
  enum Mode { on = 2 };
  int Mode = 5;
  Mode m = on;
  return Mode + m;
}
