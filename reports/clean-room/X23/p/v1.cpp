// delete through a base pointer: the derived class's deallocation function runs,
// chosen where the derived destructor is defined. The derived type is over-aligned.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t {}; }
alignas(256) unsigned char pool[1024];
int freed = 0;
struct Base { virtual ~Base() {} int b = 0; };
struct alignas(64) Derived : Base {
  int v = 0;
  static void* operator new(std::size_t, std::align_val_t) { return pool; }
  static void operator delete(void*) noexcept { freed = 1; }
#ifdef BOTH
  static void operator delete(void*, std::align_val_t) noexcept { freed = 2; }
#endif
};
int main() {
  Base* p = new Derived;
  delete p;
  return freed;
}
