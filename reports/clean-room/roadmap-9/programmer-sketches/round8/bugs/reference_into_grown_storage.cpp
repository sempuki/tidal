namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct Vec {
  int* d = nullptr; std::size_t n = 0, cap = 0;
  int& push(int x) {
    if (n == cap) {
      std::size_t c = cap ? cap * 2 : 1;
      int* f = static_cast<int*>(::operator new(c * sizeof(int)));
      for (std::size_t i = 0; i < n; ++i) ::new (f + i) int(d[i]);
      ::operator delete(d);
      d = f; cap = c;
    }
    return *::new (d + n++) int(x);
  }
  ~Vec() { ::operator delete(d); }
};
int main() {
  Vec v;
  int& first = v.push(1);
  v.push(2);  // grows; first now refers into freed storage
  return first;
}
