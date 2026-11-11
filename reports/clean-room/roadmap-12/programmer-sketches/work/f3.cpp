struct Ring {
  int v[2];
  template <bool C> struct Iter {
    const Ring* r;
    int get() const { return r->v[1]; }
  };
};
int main() {
  Ring r{{1, 0}};
  Ring::Iter<true> it{&r};
  return it.get();
}
