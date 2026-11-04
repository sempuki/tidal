// tidal-rules: UNW-3
// tidal-expect: run
// tidal-note: The class declares an allocation function and no deallocation function. Lookup of the deallocation function finds the global one, which releases the storage after the constructor throws.
using Size = decltype(sizeof 0);
extern "C" void* malloc(Size);
extern "C" void free(void*);
int frees = 0;
void operator delete(void* p) noexcept { ++frees; free(p); }
void operator delete(void* p, Size) noexcept { ++frees; free(p); }
struct S { static void* operator new(Size n) { return malloc(n); } S() { throw 0; } };
int main() { try { new S; } catch (int) {} return frees == 1 ? 0 : 1; }
