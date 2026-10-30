// tidal-rules: CONV-1
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct S { unsigned v : 3; };
int main() {
  S s{1};
  return s.v - 2 < 0 ? 0 : 1;
}
