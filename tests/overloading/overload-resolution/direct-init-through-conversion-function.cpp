// tidal-rules: OVL-14 CVF-1
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction direct_init_through_conversion_function.cpp. Direct initialization of a class from an object of another class calls the copy or move constructor of the target, whose parameter binds to the result of the source's conversion function. A functional cast, a static_cast, and parentheses give the same result.
struct Celsius {
  int degrees;
  Celsius(int d) : degrees(d) {}
};
struct Reading {
  int raw = 0;
  operator Celsius() const { return Celsius(raw * 2); }
};
int main() {
  Reading r{5};
  Celsius a = Celsius(r);
  Celsius b = static_cast<Celsius>(r);
  Celsius c(r);
  return a.degrees + b.degrees + c.degrees == 30 ? 0 : 1;
}
