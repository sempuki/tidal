inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct alignas(32) Wide { double d[4]; };
int main() {
  alignas(32) unsigned char buf[256];
  Wide* w = ::new (buf + 32) Wide();
  return w->d[0] == 0 ? 0 : 1;
}
