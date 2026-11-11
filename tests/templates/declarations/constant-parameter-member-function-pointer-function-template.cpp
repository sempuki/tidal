// tidal-rules: TPL-6 TARG-12 MPTR-3
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe member_pointer_as_typed_template_argument.cpp. A function template has a constant parameter of the pointer to member function type int (Meter::*)(int) (TPL-6). The call gives &Meter::bump explicitly (TARG-12), and the body calls the member through .* (MPTR-3).
struct Meter {
  int total = 0;
  int bump(int by) { return total += by; }
};
template <int (Meter::*Op)(int)> int apply(Meter& m, int by) { return (m.*Op)(by); }
int main() {
  Meter m;
  return apply<&Meter::bump>(m, 6) == 6 ? 0 : 1;
}
