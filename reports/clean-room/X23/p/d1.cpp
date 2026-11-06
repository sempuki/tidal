// A deleted aligned form in an over-aligned class: the first call selects it, and there is no retry.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t) { return pool; }
  static void* operator new(std::size_t, std::align_val_t) = delete;
  static void operator delete(void*) noexcept {}
};
Wide* make() { return new Wide; }
int main() {}
