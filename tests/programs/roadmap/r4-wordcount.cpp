// tidal-rules: CE-1 SA-1 LSUP-4 FN-4
// tidal-expect: run
// tidal-note: Roadmap round 4, programmer sketch wordcount.cpp (reports/clean-room/roadmap-4/programmer.md, section 1.5). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Count words and digits in a text; parse numbers; uses character values.
using size_t = decltype(sizeof 0);
struct Counts { int words = 0; int digits = 0; long sum = 0; };
constexpr bool is_space(char c) { return c == ' ' || c == '\n' || c == '\t'; }
constexpr bool is_digit(char c) { return c >= '0' && c <= '9'; }
constexpr Counts scan(const char* s) {
  Counts r; bool in = false; long cur = 0; bool num = false;
  for (; *s; ++s) {
    if (is_space(*s)) { if (num) r.sum += cur; in = false; num = false; cur = 0; continue; }
    if (!in) { ++r.words; in = true; num = is_digit(*s); }
    if (is_digit(*s)) { ++r.digits; cur = cur * 10 + (*s - '0'); } else num = false;
  }
  if (num) r.sum += cur;
  return r;
}
constexpr Counts c = scan("12 apples and 30 pears\n7 x");
static_assert(c.words == 7 && c.digits == 5 && c.sum == 49);
int main() { Counts r = scan("a 1 b 22"); return (r.words == 4 && r.sum == 23) ? 0 : 1; }
