struct Note { int id; };
const Note& pass(const Note& n) { return n; }
struct Keeper { const int& r; Keeper() : r(5) {} };            // CTOR-17
struct Keeper2 { const int& r = 6; Keeper2() {} };             // CTOR-17
int main() {
  const Note& w = pass(Note{7});                                // REF-5
  Keeper k; Keeper2 k2;
  return w.id + k.r + k2.r;
}
