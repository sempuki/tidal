// tidal-rules: DEP-24 DEP-2
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction member_template_through_current_instantiation_object.cpp. Q-0427. The parameter w has the type of the current instantiation, and lookup at the definition finds the member template get in Wrap. GCC, Clang, and EDG accept w.get<int>() with no template keyword. The test follows the compilers while the question is open.
template <class T> struct Wrap {
  T held{};
  template <class U> U get() const { return static_cast<U>(held); }
  static int whole(const Wrap& w) { return w.get<int>(); }
};
int main() {
  Wrap<double> w{3.75};
  return Wrap<double>::whole(w) == 3 ? 0 : 1;
}
