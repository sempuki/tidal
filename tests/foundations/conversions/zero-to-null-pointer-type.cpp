// tidal-rules: CONV-8
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction zero_to_nullptr_t_operator.cpp and zero_to_nullptr_t_call.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). A zero literal converts to the null pointer type, in a call and in an operator call.
struct Ord {
  signed char v;
  friend bool operator<(Ord a, decltype(nullptr)) { return a.v < 0; }
};
int is_null(decltype(nullptr)) { return 1; }
int main() {
  Ord o{-1};
  return o < 0 && is_null(0) == 1 ? 0 : 1;
}
