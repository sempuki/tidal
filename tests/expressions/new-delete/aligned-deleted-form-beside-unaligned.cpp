// tidal-rules: NEW-29 NEW-25
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-note: Propose X23 (reports/clean-room/X23/report.md), finding X23-2. Q-0417. The over-aligned class's aligned form is deleted, and it has a usable unaligned form. Overload resolution selects the deleted aligned form, so the expression is not valid.  Clang and EDG reject. GCC calls the unaligned form.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
int form = 0;
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t) { form = 1; return pool; }
  static void* operator new(std::size_t, std::align_val_t) = delete;
  static void operator delete(void*) noexcept {}
};
int main() { Wide* w = new Wide; delete w; return form == 1 ? 0 : 1; }
