namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct T { int id; T(int i) : id(i) {} };
int main() {
  unsigned char* base = static_cast<unsigned char*>(::operator new(64));
  T* d = static_cast<T*>(static_cast<void*>(base));
  ::new (&d[0]) T(1);
  ::new (&d[1]) T(2);
  int r = d[1].id - 2;
  ::operator delete(base);
  return r;
}
