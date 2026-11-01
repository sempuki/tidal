// tidal-rules: NEW-26 LSUP-5
// tidal-expect: run
// tidal-expect-clang-18: reject
// tidal-error-clang-18: no matching function for call to .operator delete
// tidal-note: Q-0125. Clang 18 does not declare the sized deallocation functions; Clang 22 does.
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
bool aligned(void* p, std::size_t a) { return reinterpret_cast<std::size_t>(p) % a == 0; }
int main() {
  void* one = ::operator new(40, std::align_val_t(64));
  if (!aligned(one, 64)) return 1;
  ::operator delete(one, std::align_val_t(64));
  void* many = ::operator new[](40, std::align_val_t{128});
  if (!aligned(many, 128)) return 2;
  ::operator delete[](many, 40, std::align_val_t{128});
  void* sized = operator new(8, std::align_val_t(32));
  operator delete(sized, 8, std::align_val_t(32));
  void* array = operator new[](8, std::align_val_t(32));
  operator delete[](array, std::align_val_t(32));
  return 0;
}
