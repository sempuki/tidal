// tidal-rules: UNW-3
// tidal-expect: run
// tidal-note: Both global forms are replaced, because an implementation can call the sized or the unsized one. Clang 18 calls the unsized form; the others call the sized form.
using Size = decltype(sizeof 0);
int frees = 0;
void operator delete(void* p) noexcept { ++frees; static_cast<void>(p); }
void operator delete(void* p, Size) noexcept { ++frees; static_cast<void>(p); }
struct S { S() { throw 0; } };
int main() {
  try { new S; }
  catch (int) {}
  return frees == 1 ? 0 : 1;
}
