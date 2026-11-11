inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct T { int id; };
struct Arena { alignas(16) unsigned char buf[64]; };
int main() {
  Arena a;
  T* d = static_cast<T*>(static_cast<void*>(a.buf));
  ::new (&d[0]) T{1};
  ::new (&d[1]) T{2};
  return d[0].id + d[1].id == 3 ? 0 : 1;
}
