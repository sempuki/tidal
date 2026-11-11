// DEP-9, INST-13: a hidden friend of a class template found by argument-dependent lookup.
template <class T> struct Meter {
  T v;
  friend int level(const Meter& m) { return static_cast<int>(m.v); }
};
template <class M> int probe(const M& m) { return level(m); }
int main() { return probe(Meter<long>{6}) == 6 ? 0 : 1; }
