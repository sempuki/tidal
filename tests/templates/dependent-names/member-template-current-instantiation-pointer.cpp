// tidal-rules: DEP-24 DEP-2
// tidal-expect: run
// tidal-note: Q-0427, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-dep-24-object-forms.json. Inside Wrap<T>, the pointer p and the local l have the type of the current instantiation, and lookup at the definition finds the member template get in Wrap. So p->get<int>() and l.get<int>() need no template keyword (DEP-24). Every compiler runs the program.
template <class T> struct Wrap {
  T held{};
  template <class U> U get() const { return static_cast<U>(held); }
  static int sum(const Wrap* p) {
    Wrap l;
    l.held = p->held;
    return p->get<int>() + l.get<int>();
  }
};
int main() {
  Wrap<double> w{2.5};
  return Wrap<double>::sum(&w) == 4 ? 0 : 1;
}
