// tidal-rules: ACC-7
// tidal-expect: reject
class Point {
  friend int make_zero() { return 0; }
};
int z = make_zero();
