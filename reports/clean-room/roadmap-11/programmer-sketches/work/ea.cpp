inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct alignas(32) Wide { double d[4]; };
struct Arena {
  alignas(32) unsigned char buf[256];
};
int main() {
  Arena a;
  Wide* w = ::new (a.buf + 32) Wide();
  return w->d[0] == 0 ? 0 : 1;
}
