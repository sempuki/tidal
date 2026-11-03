// Over-aligned elements in allocated storage (LSUP-5 shape).
using size_t = decltype(sizeof 0);
namespace std { enum class align_val_t : size_t {}; }
void* operator new(size_t, std::align_val_t);
void operator delete(void*, std::align_val_t) noexcept;
struct alignas(64) Line { int v; };
int main() {
  void* p = ::operator new(4 * sizeof(Line), std::align_val_t(alignof(Line)));
  Line* a = static_cast<Line*>(p);
  int bad = (reinterpret_cast<decltype(sizeof 0)>(p) % 64) != 0;
  ::operator delete(p, std::align_val_t(alignof(Line)));
  (void)a; return bad;
}
