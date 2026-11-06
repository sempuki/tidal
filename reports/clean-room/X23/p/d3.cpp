// A private aligned form in an over-aligned class, with a public unaligned form.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
int form = 0;
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t) { form = 1; return pool; }
  static void operator delete(void*) noexcept {}
private:
  static void* operator new(std::size_t, std::align_val_t) { form = 2; return pool; }
};
int main() { Wide* w = new Wide; delete w; return form; }
