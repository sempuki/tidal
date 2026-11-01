// tidal-rules: LSUP-5 NEW-28
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(64) unsigned char pool[256];
std::size_t requested = 0;
std::size_t alignment = 0;
void* operator new(std::size_t n, std::align_val_t a) {
  requested = n;
  alignment = static_cast<std::size_t>(a);
  return pool;
}
void operator delete(void*, std::align_val_t) noexcept {}
void operator delete(void*, std::size_t, std::align_val_t) noexcept {}
struct alignas(64) Block { int value; };
Block* volatile kept = nullptr;
int main() {
  Block* b = new Block{5};
  kept = b;
  if (alignment != 64) return 1;
  if (requested != sizeof(Block)) return 2;
  if (b->value != 5) return 3;
  delete b;
  return 0;
}
