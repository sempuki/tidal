namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct T { int id; T(int i) : id(i) {} };
int main() {
  void* v = ::operator new(16);
  T* d = static_cast<T*>(v);
  ::new (d + 1) T(2);
  int r = d[1].id - 2;
  return r;
}
