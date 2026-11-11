template <class T> class List {
  int head_ = 7;
public:
  template <bool Const> class Iter {
    int* n_ = nullptr;
    friend class List;
    explicit Iter(int* n) : n_(n) {}
  public:
    template <bool C = Const> requires C
    Iter(const Iter<false>& o) : n_(o.n_) {}
  };
  Iter<false> end() { return Iter<false>(&head_); }
  template <class U> int emplace(Iter<true> pos, U) { return *pos.n_; }
  template <class U> int push_back(U u) { return emplace(end(), u); }
};
int main() {
  List<int> a;
  return a.push_back(1) == 7 ? 0 : 1;
}
