// Today's fallback: an over-aligned class with only an unaligned operator new.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t) { return pool + 16; }   // misaligned on purpose
  static void operator delete(void*) noexcept {}
};
int main() {
  Wide* w = new Wide;
  w->v = 1;
  int bad = (reinterpret_cast<decltype(sizeof 0)>(w) % 64) != 0;
  delete w;
  return bad ? 7 : 0;
}
