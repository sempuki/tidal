// From ring_first_try.cpp. `friend class Ring;` in a class nested in
// Ring names the enclosing class (TYN-4, ACC-5). tidal-fe declares a
// new class, so r->v finds no member.
template <class T> class Ring {
  T v[2] = {1, 0};
 public:
  struct Iter {
    friend class Ring;
    const Ring* r;
    int get() const { return r->v[1]; }
  };
};
int main() {
  Ring<int> r;
  Ring<int>::Iter it{&r};
  return it.get();
}
