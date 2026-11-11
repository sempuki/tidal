template <class T> struct Ring {
  union Slot { T value; };
  Slot slots[2];
  template <bool C> struct Iter {
    const Ring* r;
    int get() const { return r->slots[1].value; }
  };
};
int main() {
  Ring<int> r{{{1}, {0}}};
  Ring<int>::Iter<true> it{&r};
  return it.get();
}
