// tidal-rules: CLS-19 CLS-30
// tidal-expect: run
// tidal-note: Only non-static data members count for the access condition. A private static data member and a private member function leave Meter standard-layout.
template <class A, class B> bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Meter {
  int reading;
  int limit;
private:
  static int instances;
  int twice() const { return 2 * reading; }
};
int main() {
  Meter meter{1, 2};
  return same_address(meter, meter.reading) ? 0 : 1;
}
