namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
// Probe: with the T* cast from the void* result, tidal-fe runs.
struct T { int id; T(int i) : id(i) {} };
int main() {
  void* s = ::operator new(16);
  T* d = static_cast<T*>(s);
  ::new (d + 1) T(2);
  int r = d[1].id - 2;
  ::operator delete(s);
  return r;
}
