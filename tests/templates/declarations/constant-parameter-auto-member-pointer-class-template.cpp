// tidal-rules: TPL-11 TARG-12 MPTR-3
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe member_pointer_as_class_template_argument.cpp. A class template with a constant parameter of type auto takes the pointer to member function &Meter::bump (TPL-11, TARG-12). Its static member function calls the member through .* (MPTR-3).
struct Meter {
  int total = 0;
  int bump(int by) { return total += by; }
};
template <auto Op> struct Bound {
  static int run(Meter& m) { return (m.*Op)(5); }
};
int main() {
  Meter m;
  return Bound<&Meter::bump>::run(m) == 5 ? 0 : 1;
}
