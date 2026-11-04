// tidal-rules: LSUP-4 LSUP-1 LIST-3 LIST-8 LIST-5 LIST-14 LIST-2 DCL-5 DCL-19 CX-1 CX-2 CE-23 CINIT-1 CINIT-2 SINIT-1 SINIT-2 STO-2 SA-1 SA-2
// tidal-expect: run
// tidal-note: Write suite W6, program config_table.cpp (reports/clean-room/W6/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A configuration table: rows of key, value, and an enabled flag.
// Built from Tidal: LIST-3, LIST-5, LIST-8, LIST-14, DCL-5, CX-1,
// CINIT-1, SINIT-1, SINIT-2, STO-2, LSUP-1, LSUP-4.

namespace std {
using size_t = decltype(sizeof 0);

template <class E> class initializer_list {
  const E* first;
  size_t count;
  constexpr initializer_list(const E* f, size_t n) : first(f), count(n) {}
public:
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
  constexpr size_t size() const { return count; }
};
}

struct Setting {
  const char* key;
  int value;
  bool enabled = true;
};

// The bound comes from the number of rows (DCL-5).
constexpr Setting table[] = {
    {"width", 80},                       // enabled from its default
    {"height", 24, false},
    {.key = "depth", .value = 3},        // designated, in order
    {.key = "mode", .value = 0, .enabled = false},
};

constexpr std::size_t table_size = sizeof table / sizeof table[0];
static_assert(table_size == 4);
static_assert(table[0].enabled && table[0].value == 80);
static_assert(!table[1].enabled);
static_assert(table[2].enabled && table[2].value == 3);
static_assert(table[3].value == 0 && !table[3].enabled);

constexpr bool same(const char* a, const char* b) {
  while (*a != '\0' && *a == *b) {
    ++a;
    ++b;
  }
  return *a == *b;
}

constexpr int lookup(const char* key, int fallback) {
  for (const Setting& s : table) {
    if (same(s.key, key)) {
      return s.enabled ? s.value : fallback;
    }
  }
  return fallback;
}
static_assert(lookup("width", -1) == 80);
static_assert(lookup("height", -1) == -1);
static_assert(lookup("missing", 7) == 7);

constexpr int total(std::initializer_list<int> values) {
  int sum = 0;
  for (int v : values) sum += v;
  return sum;
}
static_assert(total({1, 2, 3}) == 6);

// A table that main can change: constinit checks that it is set
// before any code runs (CINIT-1), and it stays modifiable.
constinit int limits[3] = {10, 20};   // the third element is 0
constinit Setting current = table[0];

// Dynamic initialization reads a constinit variable defined later.
// The constinit variable already has its value (SINIT-1).
int read_later();
int early = read_later();
constinit int later = 5;
int read_later() { return later; }

// A block-scope static runs its initializer at the first call (STO-2).
int calls = 0;
int& counter() {
  static int value = ++calls * 100;
  return value;
}

int main() {
  int result = 0;
  if (lookup("depth", 0) != 3) result |= 1;
  if (limits[2] != 0) result |= 2;
  limits[2] = 30;
  if (limits[0] + limits[1] + limits[2] != 60) result |= 4;
  ++current.value;
  if (current.value != 81 || table[0].value != 80) result |= 8;
  if (early != 5) result |= 16;
  counter() += 1;
  if (counter() != 101 || calls != 1) result |= 32;
  if (total({limits[0], limits[1]}) != 30) result |= 64;
  return result;
}
