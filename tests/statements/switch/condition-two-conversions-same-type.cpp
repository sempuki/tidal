// tidal-rules: SW-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30. Overload resolution picks the conversion function by the const of the object (CVF-4).
struct S {
  operator int() const { return 1; }
  operator int() { return 2; }
};
int pick(S s) {
  switch (s) { case 2: return 0; default: return 1; }
}
int pick_const(const S s) {
  switch (s) { case 1: return 0; default: return 1; }
}
int main() { return pick(S{}) + pick_const(S{}); }
