// tidal-rules: OBJ-3 LOOP-3
// tidal-expect: run
// tidal-expect-gcc-16: reject
// tidal-expect-gcc-host: reject
// tidal-expect-edg: reject
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: X24-1. In constant evaluation, a return out of a for ends the body's object, then the condition variable, then the initializer variable, as at run time. Clang and GCC 14 agree. GCC 16 does not run the condition variable's destructor, and EDG ends the condition variable before the body's object (Q-0452, Q-0044).
struct Log {
  char text[8] = {};
  int count = 0;
  constexpr void add(char c) { text[count++] = c; }
};
struct Mark {
  Log* log;
  char id;
  constexpr Mark(Log* l, char c) : log(l), id(c) {}
  constexpr ~Mark() { log->add(id); }
  constexpr explicit operator bool() const { return true; }
};
constexpr void leave(Log* log) {
  for (Mark init(log, 'a'); Mark cond = Mark(log, 'b');) {
    Mark body(log, 'c');
    return;
  }
}
constexpr bool ends_in_order() {
  Log log;
  leave(&log);
  return log.count == 3 && log.text[0] == 'c' && log.text[1] == 'b' && log.text[2] == 'a';
}
static_assert(ends_in_order());
int main() { return ends_in_order() ? 0 : 1; }
