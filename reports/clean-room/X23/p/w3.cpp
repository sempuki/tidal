// An over-aligned class with an aligned operator new and only an unaligned operator delete.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
int freed = 0;
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t, std::align_val_t) { return pool; }
  static void operator delete(void*) noexcept { freed = 1; }
};
int main() {
  Wide* w = new Wide;
  delete w;
  return freed == 1 ? 0 : 1;
}
