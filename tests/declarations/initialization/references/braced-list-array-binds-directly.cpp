// tidal-rules: REF-6
// tidal-expect: accept
constexpr int pair[2] = {1, 2};
constexpr const int (&same_pair)[2]{pair};
static_assert(&same_pair == &pair);

struct Cell { int v; };
constexpr Cell cells[2] = {{3}, {4}};
constexpr const Cell (&same_cells)[2]{cells};
static_assert(&same_cells == &cells);

constexpr int grid[2][2] = {{1, 2}, {3, 4}};
constexpr const int (&same_grid)[2][2]{grid};
static_assert(&same_grid == &grid);

int store[3];
int (&plain)[3]{store};
int (&&moved)[3]{static_cast<int (&&)[3]>(store)};
