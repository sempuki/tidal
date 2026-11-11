namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct T { int id; T(int i) : id(i) {} };
int main() {
  alignas(T) unsigned char buf[8];
  T* d = ::new (buf) T(1);
  int r = d->id - 1;
  return r;
}
