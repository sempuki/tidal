// tidal-rules: DCL-2 PFX-7 PFX-17 AUTO-2 DEFARG-5 FN-1 CE-1 INL-3 FN-5 FN-11 FN-8
// tidal-expect: run
// tidal-note: Write suite W5, program commands.cpp (reports/clean-room/W5/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A table of commands. Each entry has a name and a pointer to a handler.

using Handler = int(int, int);   // a function type

constexpr int add(int a, int b) { return a + b; }
constexpr int sub(int a, int b) { return a - b; }
constexpr int first(int a, int) { return a; }      // FN-1: unnamed parameter
constexpr int scale(int v, int k = 10) {            // a default argument
  static_assert(sizeof(__func__) == 6);             // FN-5, FN-11: "scale"
  return v * k;
}

struct Command {
  const char* name;
  Handler* run;
};

constexpr Command table[] = {
    {"add", add},
    {"sub", sub},
    {"first", first},
    {"scale", scale},
};

constexpr bool same(const char* a, const char* b) {
  while (*a != '\0' && *a == *b) {
    ++a;
    ++b;
  }
  return *a == *b;
}

constexpr int dispatch(const char* name, int a, int b) {
  for (const Command& c : table) {
    if (same(c.name, name)) return c.run(a, b);     // PFX-7
  }
  return -1;
}

static_assert(dispatch("add", 2, 3) == 5);
static_assert(dispatch("sub", 2, 3) == -1);
static_assert(dispatch("first", 7, 3) == 7);
static_assert(dispatch("scale", 4, 3) == 12);       // pointer: no default
static_assert(dispatch("nope", 1, 1) == -1);
static_assert(scale(4) == 40);                      // direct call: default
static_assert(sizeof(table) / sizeof(table[0]) == 4);

constexpr auto p = add;                             // AUTO-2: a pointer
static_assert(p(1, 1) == 2 && (*p)(1, 1) == 2);
static_assert(p == table[0].run);

int main(int argc, char** argv) {
  (void)argv;
  int x = argc + 1;                                 // 2 when run plainly
  if (dispatch("add", x, 3) != 5) return 1;
  if (dispatch("sub", x, 3) != -1) return 2;
  if (dispatch("first", x, 9) != 2) return 3;
  if (dispatch("scale", x, 5) != 10) return 4;
  int (*q)(int, int) = table[1].run;
  if ((*q)(x, x) != 0) return 5;
}
