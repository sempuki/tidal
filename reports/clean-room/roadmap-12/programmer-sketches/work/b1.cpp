namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct T { int id; T(int i) : id(i) {} };
int main() {
  T* d = static_cast<T*>(::operator new(8));
  ::new (d) T(1);
  int r = d->id - 1;
  ::operator delete(d);
  return r;
}
