// tidal-rules: NEW-2
// tidal-expect: accept
constexpr bool check() {
  double half = 0.5;
  int* cells = new int[3](half, 2);
  bool ok = cells[0] == 0 && cells[1] == 2 && cells[2] == 0;
  delete[] cells;
  return ok;
}
static_assert(check());
