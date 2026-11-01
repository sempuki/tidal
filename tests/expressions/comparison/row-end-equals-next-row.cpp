// tidal-rules: CMP-26 CMP-7
// tidal-expect: accept
int grid[2][3];
static_assert(&grid[0][3] == &grid[1][0]);
static_assert(grid[0] + 3 == grid[1]);
static_assert(&grid[0][2] < &grid[1][0]);
static_assert(grid[0] + 3 < grid[1] + 1);
constexpr bool local() {
  int cells[3][2]{};
  return cells[1] + 2 == cells[2] && cells[0] + 2 <= cells[1];
}
static_assert(local());
