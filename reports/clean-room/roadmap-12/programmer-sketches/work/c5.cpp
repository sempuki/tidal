namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct T { int id; T(int i) : id(i) {} };
int main() {
  T* d = static_cast<T*>(::operator new(16));
  T* q = ::new (d + 1) T(2);
  int r = q->id - 2;
  ::operator delete(d);
  return r;
}
