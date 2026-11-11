// X24 probe c1: constant evaluation and the three ways to leave two loops.
constexpr int grid[2][3] = {{1, 2, 3}, {4, 5, 6}};
constexpr int by_flag(int t) {
  int at = -1;
  bool found = false;
  for (int r = 0; r < 2 && !found; ++r)
    for (int c = 0; c < 3 && !found; ++c)
      if (grid[r][c] == t) { at = r * 3 + c; found = true; }
  return at;
}
constexpr int by_goto(int t) {
  int at = -1;
  for (int r = 0; r < 2; ++r)
    for (int c = 0; c < 3; ++c)
      if (grid[r][c] == t) { at = r * 3 + c; goto done; }
done:
  return at;
}
static_assert(by_flag(5) == 4);
#ifdef GOTO
static_assert(by_goto(5) == 4);   // CE-17: the evaluation reaches a goto
#endif
int main() { return by_goto(5) == 4 ? 0 : 1; }
