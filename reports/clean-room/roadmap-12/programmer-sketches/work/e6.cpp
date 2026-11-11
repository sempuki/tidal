namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct T { int id; T(int i) : id(i) {} };
int main() {
  unsigned char* b = static_cast<unsigned char*>(::operator new(16));
  T* d = reinterpret_cast<T*>(b);
  ::new (d) T(2);
  int r = d->id - 2;
  return r;
}
