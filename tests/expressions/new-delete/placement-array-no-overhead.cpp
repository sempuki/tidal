// tidal-rules: NEW-33 LSUP-7
// tidal-expect: run
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new[](std::size_t, void* where) noexcept { return where; }
inline void operator delete[](void*, void*) noexcept {}
struct Tracked { int value = 7; ~Tracked() { value = 0; } };
alignas(16) unsigned char buffer[256];
int main() {
  Tracked* objects = new (buffer) Tracked[3];
  if (static_cast<void*>(objects) != buffer) return 1;
  if (objects[2].value != 7) return 2;
  for (int i = 0; i != 3; ++i) objects[i].~Tracked();
  int* numbers = new (buffer) int[4]{1, 2, 3, 4};
  if (static_cast<void*>(numbers) != buffer || numbers[3] != 4) return 3;
  return 0;
}
