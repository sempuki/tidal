// tidal-rules: NEW-1 ATTR-19
// tidal-expect: run
// tidal-note: RV10-5. The type of a new expression is a pointer with an attribute after the star, and a member pointer. Every compiler accepts both.
struct Spot { int m; };
int main() {
  int** first = new int* [[]](nullptr);
  int Spot::** second = new int Spot::*(&Spot::m);
  bool ok = *first == nullptr && *second == &Spot::m;
  delete first;
  delete second;
  return ok ? 0 : 1;
}
