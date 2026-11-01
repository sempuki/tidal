// tidal-rules: LSUP-7 NEW-33
// tidal-expect: run
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* where) noexcept { return where; }
inline void* operator new[](std::size_t, void* where) noexcept { return where; }
inline void operator delete(void*, void*) noexcept {}
inline void operator delete[](void*, void*) noexcept {}
struct Tracked { int value = 7; ~Tracked() { value = 0; } };
alignas(16) unsigned char buffer[256];
int main() {
  int* one = new (buffer) int(5);
  if (static_cast<void*>(one) != buffer || *one != 5) return 1;
  Tracked* object = new (buffer) Tracked;
  if (static_cast<void*>(object) != buffer || object->value != 7) return 2;
  object->~Tracked();
  return 0;
}
