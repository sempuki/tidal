// tidal-rules: TPL-11 TARG-12 MPTR-3
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe data_member_pointer_as_auto_template_argument.cpp. A constant parameter of type auto takes the type int Pair::* from the argument &Pair::second (TPL-11, TARG-12). The function template reads the member through .* (MPTR-3).
struct Pair { int first; int second; };
template <auto Field> int read(const Pair& p) { return p.*Field; }
int main() {
  Pair p{3, 11};
  return (read<&Pair::second>(p) == 11 && read<&Pair::first>(p) == 3) ? 0 : 1;
}
