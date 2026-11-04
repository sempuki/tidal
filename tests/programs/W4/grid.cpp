// tidal-rules: NEW-2 NEW-9 NEW-5 PFX-1 PFX-4 ADD-6 ADD-10 DCL-5 DCL-19 DCL-1 ADD-7
// tidal-expect: run
// tidal-note: Write suite W4, program grid.cpp (reports/clean-room/W4/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W4: a 2D grid stored in one array.
// Rules: NEW-2, NEW-9, NEW-5, PFX-1, PFX-4, ADD-6, ADD-10, DCL-5, DCL-19, DCL-1.

class Grid {
  int rows;
  int cols;
  int* cells;                              // rows * cols ints, row after row

public:
  Grid(int r, int c) : rows(r), cols(c), cells(new int[r * c]()) {}  // NEW-2, zeroed
  Grid(const Grid&) = delete;
  Grid& operator=(const Grid&) = delete;
  ~Grid() { delete[] cells; }              // NEW-9, array form (NEW-5)

  int height() const { return rows; }
  int width() const { return cols; }

  int& at(int r, int c) { return cells[r * cols + c]; }       // PFX-1
  int at(int r, int c) const { return cells[r * cols + c]; }

  // A pointer to the first cell of a row. row(r) + cols is one past the row.
  int* row(int r) { return cells + r * cols; }                // ADD-6

  bool in_range(int r, int c) const {
    return r >= 0 && r < rows && c >= 0 && c < cols;
  }

  // Count the live neighbours of a cell, as in the game of life.
  int neighbours(int r, int c) const {
    int n = 0;
    for (int dr = -1; dr <= 1; ++dr)
      for (int dc = -1; dc <= 1; ++dc) {
        if (dr == 0 && dc == 0) continue;
        if (in_range(r + dr, c + dc) && at(r + dr, c + dc) != 0) ++n;
      }
    return n;
  }
};

// A fixed grid as an array of arrays. Each row is its own array (ADD-10),
// so the program walks it with two indices.
constexpr int R = 3;
constexpr int C = 4;

int sum_fixed(const int (&g)[R][C]) {      // DCL-1: reference to array
  int s = 0;
  for (int r = 0; r < R; ++r)
    for (int c = 0; c < C; ++c) s += g[r][c];
  return s;
}

int main() {
  Grid g(4, 5);
  if (g.at(3, 4) != 0) return 1;           // value-initialized with ()

  for (int r = 0; r < g.height(); ++r)
    for (int c = 0; c < g.width(); ++c) g.at(r, c) = r * 10 + c;
  if (g.at(2, 3) != 23) return 2;

  // Walk one row with a pointer, from its first cell to one past its last.
  int sum = 0;
  for (int* p = g.row(1); p != g.row(1) + g.width(); ++p) sum += *p;
  if (sum != 10 + 11 + 12 + 13 + 14) return 3;

  // The rows sit back to back in one array of int.
  if (g.row(2) - g.row(1) != g.width()) return 4;              // ADD-7
  if (&g.at(1, 4) + 1 != &g.at(2, 0)) return 5;

  Grid life(3, 3);
  life.at(0, 1) = 1;
  life.at(1, 1) = 1;
  life.at(2, 1) = 1;
  if (life.neighbours(1, 0) != 3) return 6;
  if (life.neighbours(1, 1) != 2) return 7;
  if (life.neighbours(0, 0) != 2) return 8;

  int fixed[R][C] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}};
  if (sum_fixed(fixed) != 78) return 9;

  return 0;
}
