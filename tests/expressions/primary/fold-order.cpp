// tidal-rules: PRIM-28
// tidal-expect: run
int order[4];
int n = 0;
bool mark(int v) { order[n++] = v; return v != 2; }
template <int... V> void right_comma() { (mark(V), ...); }
template <int... V> bool left_and() { return (... && mark(V)); }
template <int... V> bool right_or() { return (!mark(V) || ...); }
int main() {
  right_comma<1, 2, 3>();
  if (n != 3 || order[0] != 1 || order[1] != 2 || order[2] != 3) return 1;
  n = 0;
  if (left_and<1, 2, 3>()) return 2;
  if (n != 2 || order[0] != 1 || order[1] != 2) return 3;   // stops at 2
  n = 0;
  if (!right_or<1, 2, 3>()) return 4;
  if (n != 2 || order[0] != 1 || order[1] != 2) return 5;   // stops at 2
  return 0;
}
