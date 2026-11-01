// tidal-rules: NEW-30
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .operator new
// tidal-error-gcc: no matching function for call to .Plain::operator new
// tidal-error-edg: function "Plain::operator new" cannot be called
// tidal-note: Q-0300.
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(64) unsigned char pool[64];
struct Plain {
  int value;
  static void* operator new(std::size_t, std::align_val_t) { return pool; }
  static void operator delete(void*, std::align_val_t) noexcept {}
};
Plain* make() { return new Plain; }
