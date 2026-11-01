// tidal-rules: NEW-31 LSUP-6
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
struct nothrow_t { explicit nothrow_t() = default; };
inline constexpr nothrow_t nothrow{};
}
int built = 0, freed = 0, evaluated = 0;
void* operator new(std::size_t, const std::nothrow_t&) noexcept { return nullptr; }
void operator delete(void*, const std::nothrow_t&) noexcept { freed = 1; }
struct Gadget { int value; Gadget(int v) : value(v) { built = 1; } };
int argument() { evaluated = 1; return 3; }
int main() {
  Gadget* g = new (std::nothrow) Gadget(argument());
  if (g != nullptr) return 1;
  if (built != 0) return 2;
  if (evaluated != 0) return 3;
  if (freed != 0) return 4;
  return 0;
}
