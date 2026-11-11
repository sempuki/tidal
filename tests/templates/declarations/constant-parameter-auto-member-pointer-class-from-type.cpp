// tidal-rules: TPL-11 TARG-12 TSP-14
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe member_pointer_traits_parameter.cpp. The function parameter's type names the class of the pointer to member through a partial specialization that matches R (C::*)(int) (TSP-14). The constant parameter of type auto takes its type from &Meter::bump (TPL-11, TARG-12), so the parameter is Meter&.
struct Meter {
  int total = 0;
  int bump(int by) { return total += by; }
};
template <class P> struct owner_of;
template <class R, class C> struct owner_of<R (C::*)(int)> { using type = C; };
template <auto Op> int apply(typename owner_of<decltype(Op)>::type& m) { return (m.*Op)(7); }
int main() {
  Meter m;
  return apply<&Meter::bump>(m) == 7 ? 0 : 1;
}
