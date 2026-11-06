// tidal-rules: CPY-4 CPY-5 OVL-14
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction assign_through_converting_constructor.cpp. Assignment of an int to a class object calls an implicit assignment operator, whose parameter takes a temporary that the converting constructor makes.
struct Meter {
  int value = 0;
  Meter(int v) : value(v) {}
};
int main() {
  Meter m(3);
  m = 11;
  return m.value == 11 ? 0 : 1;
}
