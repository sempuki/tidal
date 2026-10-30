// tidal-rules: REF-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int destroyed = 0;
struct Note { int v; ~Note() { ++destroyed; } };
struct Holder { const Note& n; };
int main() {
  Holder h{Note{5}};
  int before = destroyed;
  return h.n.v == 5 && before == 0 ? 0 : 1;
}
