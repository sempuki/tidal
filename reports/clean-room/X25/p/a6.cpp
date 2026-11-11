// A two-dimensional array member: build element [1][0] only.
#include "construct.h"
union Grid { int cells[2][2]; constexpr Grid() {} };
constexpr int f() {
  Grid g;
  std::construct_at(&g.cells[1][0], 4);
  return g.cells[1][0];
}
static_assert(f() == 4);
int main() {}
