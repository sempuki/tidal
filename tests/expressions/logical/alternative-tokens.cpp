// tidal-rules: LOG-13
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int main() {
  bool a = true, b = false;
  int x = 6, y = 3;
  bool r = (a and not b) or b;
  int z = (x bitand y) bitor (x xor y);
  x and_eq 2; x or_eq 1; x xor_eq 4; (void)compl x;
  return r and z == 7 and (a not_eq b) and x == 7 ? 0 : 1;
}
