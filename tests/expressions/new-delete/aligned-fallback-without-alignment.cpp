// tidal-rules: NEW-29
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(256) unsigned char pool[1024];
int form = 0, freed = 0;
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Wide {
  int value;
  static void* operator new(std::size_t) { form = 1; return pool; }
  static void operator delete(void*) noexcept { freed = 1; }
};
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) WidePlaced {
  int value;
  static void* operator new(std::size_t, int) { form = 2; return pool; }
  static void operator delete(void*) noexcept { freed = 2; }
};
int main() {
  Wide* w = new Wide;
  if (form != 1) return 1;
  delete w;
  if (freed != 1) return 2;
  WidePlaced* p = new (5) WidePlaced;
  if (form != 2) return 3;
  delete p;
  if (freed != 2) return 4;
  return 0;
}
