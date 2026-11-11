// tidal-rules: DEP-24
// tidal-expect: run
// tidal-note: Gap review RV6-B-9. Q-0427. Lookup at the definition also searches the bases that are not dependent. A member template that such a base declares is found after . with an object of the current instantiation, and after :: with the current instantiation as the qualifier, so neither needs template. Every compiler accepts both forms.
struct Base {
  template <class U> U get() const { return U(7); }
  template <class U> static U make() { return U(5); }
};
template <class T> struct Wrap : Base {
  static int read(const Wrap& w) { return w.get<int>(); }
  static int build() { return Wrap::make<int>(); }
};
int main() { return Wrap<int>::read(Wrap<int>{}) == 7 && Wrap<int>::build() == 5 ? 0 : 1; }
