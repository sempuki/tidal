// tidal-rules: UNI-14 STG-10
// tidal-expect: accept
// tidal-note: W20-2, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-w20-2-step-in-inactive-array.json. The union Store has no active member, or another member is active. In constant evaluation, the program forms pointers to elements of the array member cells, steps from element 0 to elements 1 and 3, subtracts, and compares them. No object is read. Every compiler accepts.
union Store {
  int cells[3];
  char flag;
  constexpr Store() {}
};
constexpr bool steps(bool other_active) {
  Store s;
  if (other_active) s.flag = 'x';
  int* first = &s.cells[0];
  int* end = first + 3;
  return first + 1 == &s.cells[1] && end - first == 3 && first < &s.cells[2];
}
static_assert(steps(false));
static_assert(steps(true));
