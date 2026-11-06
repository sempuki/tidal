// tidal-rules: NEW-21 ODR-5
// tidal-expect: run
// tidal-note: Propose X23 (reports/clean-room/X23/report.md), finding X23-4. A delete through a pointer to a base with a virtual destructor calls the deallocation function of the dynamic type, chosen where that class's destructor is defined. The over-aligned Derived has an aligned form, which runs.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
int freed = 0;
struct Base { virtual ~Base() {} int b = 0; };
struct alignas(64) Derived : Base {
  int v = 0;
  static void* operator new(std::size_t, std::align_val_t) { return pool; }
  static void operator delete(void*) noexcept { freed = 1; }
  static void operator delete(void*, std::align_val_t) noexcept { freed = 2; }
};
int main() {
  Base* p = new Derived;
  delete p;
  return freed == 2 ? 0 : 1;
}
