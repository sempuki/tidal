template <class T> struct List {
  template <bool Const> struct Iter {
    int* p = nullptr;
    Iter() = default;
    explicit Iter(int* q) : p(q) {}
    template <bool C = Const> requires C
    Iter(const Iter<false>& o) : p(o.p) {}
  };
  int x = 0;
  Iter<false> end() { return Iter<false>(&x); }
  int at(Iter<true> pos) { return *pos.p; }
  template <class U> int put(U) { return at(end()); }
  int put2() { return at(end()); }
};
int main() {
  List<int> a;
  return a.put(1) + a.put2();
}
