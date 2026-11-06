// A placement form with an extra int: today the alignment is dropped when only (size, int) exists.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
int form = 0;
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t, int) { form = 1; return pool; }
  static void operator delete(void*) noexcept {}
#ifdef ALIGNED
  static void* operator new(std::size_t, std::align_val_t, int) { form = 2; return pool; }
#endif
};
int main() {
  Wide* w = new (5) Wide;
  delete w;
  return form;
}
