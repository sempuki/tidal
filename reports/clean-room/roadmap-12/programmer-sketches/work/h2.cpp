class Ring {
  int v[2] = {1, 0};
 public:
  struct Iter {
    friend class Ring;
    const Ring* r;
    int get() const { return r->v[1]; }
  };
};
int main() {
  Ring r;
  Ring::Iter it{&r};
  return it.get();
}
