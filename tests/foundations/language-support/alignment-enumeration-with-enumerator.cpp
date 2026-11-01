// tidal-rules: LSUP-5 NEW-26
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t { page = 4096 };
}
struct alignas(64) Block { int value; };
int main() {
  void* p = ::operator new(8, std::align_val_t::page);
  bool ok = reinterpret_cast<std::size_t>(p) % 4096 == 0;
  ::operator delete(p, std::align_val_t::page);
  delete new Block;
  return ok ? 0 : 1;
}
