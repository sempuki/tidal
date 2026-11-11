namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct T { int id; T(int i) : id(i) {} };
int main() {
  void* s = ::operator new(8);
  ::new (s) T(1);
  T* d = static_cast<T*>(s);
  int r = d->id - 1;
  ::operator delete(s);
  return r;
}
