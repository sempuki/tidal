// tidal-rules: CAST-17
// tidal-expect: run
// tidal-note: The cast goes from the inaccessible base back to the derived class, for a pointer and for a reference.
struct B { int b = 4; };
struct D : private B { int d = 5; };
int main() {
  D obj;
  B* p = (B*)&obj;
  D* back = (D*)p;
  D& ref = (D&)*p;
  return back == &obj && ref.d == 5 ? 0 : 1;
}
