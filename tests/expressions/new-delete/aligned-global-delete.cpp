// tidal-rules: NEW-28 NEW-22
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(256) unsigned char pool[1024];
int freed = 0;
void* operator new(std::size_t, std::align_val_t) { return pool; }
void operator delete(void*, std::align_val_t a) noexcept { freed = int(a); }
void operator delete(void*, std::size_t, std::align_val_t a) noexcept { freed = 1000 + int(a); }
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Wide { int value; };
Wide* volatile kept = nullptr;
int main() {
  kept = new Wide;
  delete kept;
  const int a = alignof(Wide);
  return freed == a || freed == 1000 + a ? 0 : 1;
}
