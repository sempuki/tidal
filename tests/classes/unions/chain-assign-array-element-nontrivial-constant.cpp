// tidal-rules: UNI-6 UNI-13
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: member call on member .items. of union with active member .none.
// tidal-error-gcc: does not implicitly begin its lifetime
// tidal-note: W20-2, with the writer probe reports/clean-room/roadmap-10/writer-probes/w20-2-union-array-member.json. Q-0213. The element class Reading has a constructor and no default constructor, so its default constructor is not trivial. An assignment to an element of the array member items does not make items active (UNI-13), and the assignment uses an object that does not exist (UNI-6). Clang and GCC reject the constant; EDG accepts it, as for the member chain of Q-0213.
struct Reading {
  int v;
  constexpr Reading(int x) : v(x) {}
};
union Log {
  char none;
  Reading items[2];
  constexpr Log() : none() {}
};
constexpr int fill() {
  Log log;
  log.items[1] = Reading(7);
  return log.items[1].v;
}
static_assert(fill() == 7);
