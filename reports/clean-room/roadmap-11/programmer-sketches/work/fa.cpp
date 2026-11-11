inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct alignas(32) Wide { double d[4]; };
struct Arena {
  alignas(32) unsigned char buf_[256];
  decltype(sizeof 0) used_ = 0;
  void* allocate(decltype(sizeof 0) size, decltype(sizeof 0) align) {
    decltype(sizeof 0) start = (used_ + align - 1) & ~(align - 1);
    used_ = start + size;
    return buf_ + start;
  }
};
int main() {
  Arena a;
  a.allocate(4, 4);
  void* p = a.allocate(sizeof(Wide), alignof(Wide));
  Wide* w = ::new (p) Wide();
  return w->d[0] == 0 ? 0 : 1;
}
