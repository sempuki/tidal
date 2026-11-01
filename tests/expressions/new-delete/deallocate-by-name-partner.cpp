// tidal-rules: NEW-40 NEW-19 NEW-26
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
int main() {
  void* one = ::operator new(24);
  ::operator delete(one);
  void* many = ::operator new[](40);
  ::operator delete[](many);
  void* wide = ::operator new(128, std::align_val_t(64));
  ::operator delete(wide, std::align_val_t(64));
  return 0;
}
