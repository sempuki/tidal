// tidal-rules: UNW-3
// tidal-expect: run
// tidal-expect-clang-18: run-fail
// tidal-note: The second element of an array whose element type has a destructor throws in a new expression. The sized global array deallocation function releases the storage. Clang 18 calls the unsized form, and the program returns 1.
using Size = decltype(sizeof 0);
extern "C" void* malloc(Size);
extern "C" void free(void*);
int which = 0;
int k = 0;
void* operator new[](Size n) { return malloc(n); }
void operator delete[](void* p) noexcept { which = 1; free(p); }
void operator delete[](void* p, Size) noexcept { which = 2; free(p); }
struct D { char c; D() { if (k++ == 1) throw 0; } ~D() {} };
int main() { try { new D[3]; } catch (int) {} return which == 2 ? 0 : 1; }
