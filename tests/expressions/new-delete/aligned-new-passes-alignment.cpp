// tidal-rules: NEW-28 NEW-27
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(256) unsigned char pool[1024];
int form = 0;
std::size_t alignment = 0;
void* operator new(std::size_t) { form = 1; return pool; }
void* operator new(std::size_t, std::align_val_t a) { form = 2; alignment = std::size_t(a); return pool; }
void* operator new[](std::size_t) { form = 3; return pool; }
void* operator new[](std::size_t, std::align_val_t a) { form = 4; alignment = std::size_t(a); return pool; }
void* operator new(std::size_t, std::align_val_t a, int) { form = 5; alignment = std::size_t(a); return pool; }
void* operator new(std::size_t, int) { form = 6; return pool; }
void operator delete(void*) noexcept {}
void operator delete(void*, std::align_val_t) noexcept {}
void operator delete[](void*) noexcept {}
void operator delete[](void*, std::align_val_t) noexcept {}
struct alignas(__STDCPP_DEFAULT_NEW_ALIGNMENT__) AtDefault { int value; };
struct alignas(2 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Above { int value; };
void* volatile kept = nullptr;
int main() {
  kept = new AtDefault;
  if (form != 1) return 1;
  kept = new Above;
  if (form != 2 || alignment != alignof(Above)) return 2;
  kept = new AtDefault[2];
  if (form != 3) return 3;
  kept = new Above[2];
  if (form != 4 || alignment != alignof(Above)) return 4;
  kept = new (7) Above;
  if (form != 5 || alignment != alignof(Above)) return 5;
  kept = new (7) AtDefault;
  if (form != 6) return 6;
  return 0;
}
