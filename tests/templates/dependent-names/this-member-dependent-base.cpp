// tidal-rules: DEP-17 DEP-13
// tidal-expect: run
// tidal-note: Gauge has a dependent base, so this->missing waits for the instantiation. The specialization Base<int> declares it.
template <class T> struct Base { };
template <class T> struct Gauge : Base<T> {
  int read() { return this->missing(); }
};
template <> struct Base<int> { int missing() { return 3; } };
int main() { return Gauge<int>{}.read() == 3 ? 0 : 1; }
