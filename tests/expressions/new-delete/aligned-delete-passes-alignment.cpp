// tidal-rules: NEW-28 NEW-21
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(256) unsigned char pool[1024];
int freed = 0;
struct Both {
  static void* operator new(std::size_t) { return pool; }
  static void* operator new(std::size_t, std::align_val_t) { return pool; }
  static void operator delete(void*) noexcept { freed = 1; }
  static void operator delete(void*, std::align_val_t) noexcept { freed = 2; }
};
struct Narrow : Both { int value; };
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Wide : Both { int value; };
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) WideThrows : Both {
  WideThrows() { throw 0; }
};
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) WidePlacedThrows {
  static void* operator new(std::size_t, std::align_val_t, int) { return pool; }
  static void operator delete(void*, std::align_val_t, int) noexcept { freed = 3; }
  static void operator delete(void*, int) noexcept { freed = 4; }
  static void operator delete(void*, std::align_val_t) noexcept {}
  WidePlacedThrows() { throw 0; }
};
int main() {
  delete new Narrow;
  if (freed != 1) return 1;
  delete new Wide;
  if (freed != 2) return 2;
  freed = 0;
  try { new WideThrows; } catch (int) {}
  if (freed != 2) return 3;
  freed = 0;
  try { new (9) WidePlacedThrows; } catch (int) {}
  if (freed != 3) return 4;
  return 0;
}
