// tidal-rules: NEW-40
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address -fno-sanitize-recover=all
// tidal-error: new-delete-type-mismatch
// tidal-note: Run-time check with the sanitizer (decision 0028). The storage comes from the aligned form and goes back through the form without an alignment.
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
int main() {
  void* raw = ::operator new(64, std::align_val_t(64));
  ::operator delete(raw);              // undefined: the partner takes the alignment
  return 0;
}
