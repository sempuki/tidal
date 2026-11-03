// tidal-rules: DER-16
// tidal-expect: run
// tidal-note: The cast goes down two levels, from a pointer to const.
struct B { int b = 1; };
struct M : B { int m = 2; };
struct D : M { int d = 3; };
int main() {
  D d;
  const B* p = &d;
  const D* back = static_cast<const D*>(p);
  return back == &d && back->d == 3 ? 0 : 1;
}
