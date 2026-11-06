// Same as d1, but the deleted aligned form is the only aligned form, and the program runs.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
int form = 0;
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t) { form = 1; return pool; }
  static void* operator new(std::size_t, std::align_val_t) = delete;
  static void operator delete(void*) noexcept {}
};
int main() { Wide* w = new Wide; delete w; return form; }
