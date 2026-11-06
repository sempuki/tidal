// tidal-rules: COND-16 COND-3
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction cond_class_lvalue_vs_string_literal.cpp. A class lvalue meets a string literal that converts to the class through a converting constructor that takes a pointer to const char.
struct Label {
  const char* text;
  Label(const char* t) : text(t) {}
};
char first(bool quiet) {
  Label loud = "HIGH";
  Label l = quiet ? "low" : loud;
  return l.text[0];
}
int main() { return first(true) == 'l' && first(false) == 'H' ? 0 : 1; }
