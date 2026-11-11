template <class T> struct Ring {
  T v[2];
  template <bool C> struct Iter {
    const Ring* r;
    auto& get() const {
      if constexpr (C) return r->v[1];
      else return const_cast<Ring*>(r)->v[1];
    }
  };
};
int main() {
  Ring<int> r{{1, 0}};
  Ring<int>::Iter<true> it{&r};
  return it.get();
}
