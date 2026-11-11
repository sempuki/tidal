// tidal-rules: NEW-22 NEW-28
// tidal-expect: run
// tidal-expect-clang-18: run-fail
// tidal-note: Gap review RV6-B-13. After delete of a single object of complete type whose alignment is more than the default new alignment, the global sized aligned form is called. Clang 22 and GCC call it. Clang 18 calls the aligned form without the size.
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(256) unsigned char pool[1024];
int freed = 0;
void* operator new(std::size_t, std::align_val_t) { return pool; }
void operator delete(void*, std::align_val_t) noexcept { freed = 1; }
void operator delete(void*, std::size_t, std::align_val_t) noexcept { freed = 2; }
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Wide { int value; };
Wide* volatile kept = nullptr;
int main() {
  kept = new Wide;
  delete kept;
  return freed == 2 ? 0 : 1;
}
