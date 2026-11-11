inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct Arena {
  alignas(32) unsigned char buf[256];
};
int main() {
  Arena a;
  double* w = ::new (a.buf + 8) double(1.0);
  return *w == 1.0 ? 0 : 1;
}
