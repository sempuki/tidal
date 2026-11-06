// A callback table filled at static initialization by two units.
// This unit owns the table. plugins.cpp adds three more callbacks.
using Callback = int (*)(int);

struct Table {
  Callback fns[8];
  int count;
};

// Constant-initialized, so the table is empty and ready before any
// dynamic initializer of any unit runs (SINIT-1, SINIT-P2).
constinit Table table{};

bool add_callback(Callback f) {
  if (table.count == 8) return false;
  table.fns[table.count++] = f;
  return true;
}

static int twice(int x) { return 2 * x; }

// Dynamic initialization: runs at start-up, before main on Clang and
// GCC (SINIT-5). The order against plugins.cpp is unspecified (SINIT-3).
bool main_registered =
    add_callback(&twice) &&                         // a plain function
    add_callback([](int x) { return x + 1; });      // a lambda, converted (LAM-10)

extern bool plugins_registered;

// A captureless lambda as a constant function pointer (LAM-10, LAM-12).
constexpr Callback triple = [](int x) { return 3 * x; };
static_assert(triple(4) == 12);

int main() {
  if (!main_registered || !plugins_registered) return 1;
  if (table.count != 5) return 2;
  // The order of the entries depends on the unit order, so check a sum.
  int sum = 0;
  for (int i = 0; i < table.count; ++i) sum += table.fns[i](10);
  // twice 20, +1 11, negate -10, square 100, minus3 7
  if (sum != 20 + 11 - 10 + 100 + 7) return 3;
  return 0;
}
