// tidal-rules: REF-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int destroyed = 0;
struct Note { int v; ~Note() { ++destroyed; } };
Note make(int v) { return Note{v}; }
int main() {
  {
    const Note& a = (make(1));
    const Note& b = true ? make(2) : make(3);
    const Note& c = (0, make(4));
    if (destroyed != 0) return 1;
    if (a.v != 1 || b.v != 2 || c.v != 4) return 2;
  }
  return destroyed == 3 ? 0 : 3;
}
