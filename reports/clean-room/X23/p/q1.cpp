// A test of validity sees the selection of the allocation function.
// Plain has ordinary alignment and only an aligned operator new (NEW-30 rejects new Plain).
// Wide is over-aligned and has only an unaligned operator new (NEW-29 accepts new Wide today).
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
struct Plain {
  int v;
  static void* operator new(std::size_t, std::align_val_t) { return pool; }
  static void operator delete(void*, std::align_val_t) noexcept {}
};
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t) { return pool; }
  static void operator delete(void*) noexcept {}
};
template <class T> constexpr bool can_new = requires { new T; };
template <class T> int pick() { if constexpr (can_new<T>) return 1; else return 2; }
int main() { return pick<Plain>() * 10 + pick<Wide>(); }   // today: 21
