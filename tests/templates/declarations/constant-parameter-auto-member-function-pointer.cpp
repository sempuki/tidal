// tidal-rules: TPL-11 TARG-12 MPTR-3
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction member_pointer_as_auto_template_argument.cpp. A constant parameter of type auto takes its type from the argument &Meter::bump (TPL-11), so Op is a pointer to member function of Meter. The call names the argument explicitly (TARG-12), and the body calls the member through .* (MPTR-3).
struct Meter {
  int total = 0;
  int bump(int by) { return total += by; }
};
template <auto Op> int apply_twice(Meter& m, int by) {
  (m.*Op)(by);
  return (m.*Op)(by);
}
int main() {
  Meter m;
  return apply_twice<&Meter::bump>(m, 4) == 8 ? 0 : 1;
}
