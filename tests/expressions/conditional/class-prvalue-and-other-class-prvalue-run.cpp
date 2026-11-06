// tidal-rules: COND-16 COND-3
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction cond_class_prvalue_vs_convertible_class.cpp. A class prvalue meets a prvalue of an empty class that converts to it through a converting constructor.
struct Unset {};
struct Field {
  int v = 0;
  Field(int x) : v(x) {}
  Field(Unset) {}
};
int main(int argc, char**) {
  Field f = argc > 0 ? Field(8) : Unset{};
  Field g = argc < 0 ? Field(8) : Unset{};
  return f.v == 8 && g.v == 0 ? 0 : 1;
}
