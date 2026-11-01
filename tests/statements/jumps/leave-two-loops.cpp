// tidal-rules: JMP-1 JMP-8
// tidal-expect: run
constexpr int grid[3][3] = {{4, 7, 1}, {9, 2, 7}, {5, 3, 8}};

int find_goto(int target, int& visited) {
  visited = 0;
  for (int r = 0; r < 3; ++r) {
    for (int c = 0; c < 3; ++c) {
      ++visited;
      if (grid[r][c] == target) goto found;
    }
  }
  return 0;
found:
  return 1;
}

int find_return(int target, int& visited) {
  visited = 0;
  for (int r = 0; r < 3; ++r) {
    for (int c = 0; c < 3; ++c) {
      ++visited;
      if (grid[r][c] == target) return 1;
    }
  }
  return 0;
}

int find_flag(int target, int& visited) {
  visited = 0;
  bool found = false;
  for (int r = 0; r < 3 && !found; ++r) {
    for (int c = 0; c < 3 && !found; ++c) {
      ++visited;
      found = grid[r][c] == target;
    }
  }
  return found;
}

int find_break(int target, int& visited) {
  visited = 0;
  int found = 0;
  for (int r = 0; r < 3; ++r) {
    for (int c = 0; c < 3; ++c) {
      ++visited;
      if (grid[r][c] == target) {
        found = 1;
        break;  // leaves the inner loop only
      }
    }
  }
  return found;
}

int main() {
  int v = 0;
  // The first 7 is the second cell. Each way stops after two cells.
  if (find_goto(7, v) != 1 || v != 2) return 1;
  if (find_return(7, v) != 1 || v != 2) return 2;
  if (find_flag(7, v) != 1 || v != 2) return 3;
  // A plain break: the outer loop goes on through rows 1 and 2.
  if (find_break(7, v) != 1 || v != 2 + 3 + 3) return 4;
  if (find_goto(6, v) != 0 || v != 9) return 5;
  return 0;
}
