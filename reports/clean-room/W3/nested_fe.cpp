// Nested-loop search that leaves both loops early.
// C++ has no labeled break (JMP grammar: "break" ";"; JMP-1: break ends
// the innermost loop only). Three ways to leave both loops:
//   1. goto a label after the loops (JMP-8, JMP-5, JMP-6, JMP-9),
//   2. return from a function (JMP-4),
//   3. a flag tested by the outer loop condition (LOOP-3).

struct Pos {
  int row;
  int col;
};

constexpr int rows = 4;
constexpr int cols = 5;

constexpr int grid[rows][cols] = {
    {3, 8, 1, 9, 4},
    {7, 2, 6, 5, 0},
    {1, 1, 8, 2, 7},
    {9, 3, 6, 4, 2},
};

// 1. goto. Also counts the cells visited, to show the early exit.
// Not constexpr: CX-6 forbids evaluating a goto in constant evaluation.
Pos find_goto(int target, int& visited) {
  Pos p{-1, -1};
  visited = 0;
  for (int r = 0; r < rows; ++r) {
    for (int c = 0; c < cols; ++c) {
      ++visited;
      if (grid[r][c] == target) {
        p.row = r; p.col = c;
        goto done;  // JMP-9: leaves both loop scopes, r and c end
      }
    }
  }
  {  // JMP-6: the goto may skip a whole block, not a bare initialized declaration
    int misses = visited;  // only reached when nothing matched
    if (misses != rows * cols) p.row = -2;  // cannot happen
  }
done:
  return p;
}

// 2. return from the inner loop.
constexpr Pos find_return(int target) {
  for (int r = 0; r < rows; ++r) {
    for (int c = 0; c < cols; ++c) {
      if (grid[r][c] == target) return {r, c};
    }
  }
  return {-1, -1};
}

// 3. a flag in both conditions.
constexpr Pos find_flag(int target) {
  Pos p{-1, -1};
  bool found = false;
  for (int r = 0; r < rows && !found; ++r) {
    for (int c = 0; c < cols && !found; ++c) {
      if (grid[r][c] == target) {
        p.row = r; p.col = c;
        found = true;
      }
    }
  }
  return p;
}

constexpr bool same(Pos a, Pos b) { return a.row == b.row && a.col == b.col; }

constexpr bool check(int target, Pos want) {
  return same(find_return(target), want) && same(find_flag(target), want);
}

static_assert(check(3, {0, 0}));
static_assert(check(5, {1, 3}));
static_assert(check(8, {0, 1}));   // first 8 in row order, not {2, 2}
static_assert(check(2, {1, 1}));
static_assert(check(42, {-1, -1}));

int main() {
  int visited = 0;
  Pos p = find_goto(6, visited);
  if (!same(p, {1, 2})) return 1;
  if (visited != 8) return 2;  // 5 cells in row 0, then 3 in row 1
  if (!same(find_goto(42, visited), {-1, -1}) || visited != rows * cols) return 5;
  for (int t = 0; t < 10; ++t) {
    if (!same(find_goto(t, visited), find_return(t))) return 3;
    if (!same(find_flag(t), find_return(t))) return 4;
  }
  return 0;
}
