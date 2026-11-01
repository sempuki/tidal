// tidal-rules: LSUP-7 NEW-20
// tidal-expect: reject
// tidal-error-clang: no matching .*operator new\[\]
// tidal-error-gcc: no matching function for call to .operator new ?\[\]
// tidal-error-edg: no instance of overloaded .operator new\[\]. matches
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* where) noexcept { return where; }
alignas(int) unsigned char cells[4 * sizeof(int)];
int main() {
  int* values = new (cells) int[4]();  // needs the operator new[] form
  return values[0];
}
