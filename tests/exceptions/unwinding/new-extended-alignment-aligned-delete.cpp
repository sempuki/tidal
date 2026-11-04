// tidal-rules: UNW-3
// tidal-expect: run
// tidal-note: The type has extended alignment, so the global deallocation function that releases the storage takes the alignment. The test accepts the sized and the unsized form. Clang and GCC call the unsized form.
using Size = decltype(sizeof 0);
namespace std { enum class align_val_t : Size {}; }
int which = 0;
void operator delete(void*) noexcept { which = 1; }
void operator delete(void*, Size) noexcept { which = 2; }
void operator delete(void*, std::align_val_t) noexcept { which = 3; }
void operator delete(void*, Size, std::align_val_t) noexcept { which = 4; }
struct alignas(64) S { char c; S() { throw 0; } };
int main() { try { new S; } catch (int) {} return which == 3 || which == 4 ? 0 : 1; }
