// tidal-rules: UNW-3 NEW-22
// tidal-expect: run
// tidal-expect-clang-18: run-fail
// tidal-note: The constructor of a single object of complete type throws in a new expression. The sized global deallocation function releases the storage. Clang 18 calls the unsized form, and the program returns 1.
using Size = decltype(sizeof 0);
extern "C" void free(void*);
int which = 0;
void operator delete(void* p) noexcept { which = 1; free(p); }
void operator delete(void* p, Size) noexcept { which = 2; free(p); }
struct S { int v; S() { throw 0; } };
int main() { try { new S; } catch (int) {} return which == 2 ? 0 : 1; }
