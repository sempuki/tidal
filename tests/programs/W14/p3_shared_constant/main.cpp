// tidal-rules: LNK-3 INL-1 LNK-4 ODR-7 STO-7 ODR-3 CE-3 ODR-8
// tidal-expect: run
// tidal-sources: a.tu.cpp b.tu.cpp config.tu.cpp
// tidal-note: Write suite W14, program p3_shared_constant, units main.cpp, a.cpp, b.cpp, config.cpp (reports/clean-room/W14/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
#include "limits.h"

static_assert(limits::max_users == 64);
static_assert(limits::page_size == 16);

int main() {
  // inline variable: one address in the program (LNK-4).
  if (limits::max_users_seen_by_a() != &limits::max_users) return 1;
  // constexpr variable: each unit has its own object (LNK-3, INL-1).
  if (limits::page_size_seen_by_a() == &limits::page_size) return 2;
  if (*limits::page_size_seen_by_a() != limits::page_size) return 3;
  if (limits::buffer_cells_in_b() != 4) return 4;
  if (limits::build_number != 2026) return 5;
  return 0;
}
