// tidal-rules: NEW-41
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
int main() {
  void* none = nullptr;
  ::operator delete(none);             // does nothing
  ::operator delete[](none);
  ::operator delete(none, std::align_val_t(64));
  return 0;
}
