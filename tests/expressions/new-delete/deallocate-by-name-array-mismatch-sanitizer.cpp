// tidal-rules: NEW-40
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address -fno-sanitize-recover=all
// tidal-error: alloc-dealloc-mismatch
// tidal-note: Run-time check with the sanitizer (decision 0028). The storage comes from operator new[] and goes back through operator delete.
int main() {
  void* raw = ::operator new[](32);
  ::operator delete(raw);              // undefined: the partner is operator delete[]
  return 0;
}
