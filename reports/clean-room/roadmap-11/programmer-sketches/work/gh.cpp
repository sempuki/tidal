namespace std { enum class align_val_t : decltype(sizeof 0) {}; }
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct alignas(32) Wide { double d[4]; };
int main() {
  void* base = ::operator new(64, std::align_val_t(32));
  Wide* w = ::new (base) Wide();
  int r = w->d[0] == 0 ? 0 : 1;
  ::operator delete(base, std::align_val_t(32));
  return r;
}
