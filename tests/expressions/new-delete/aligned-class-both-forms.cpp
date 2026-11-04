// tidal-rules: NEW-22 NEW-28
// tidal-expect: run
// tidal-note: The class declares both aligned usual forms. A delete expression calls the form without the size, and so does the cleanup after a constructor throws.
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
alignas(256) unsigned char pool[1024];
int freed = 0;
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Wide {
  int value;
  Wide(int v) : value(v) { if (v < 0) throw 0; }
  static void* operator new(std::size_t, std::align_val_t) { return pool; }
  static void operator delete(void*, std::align_val_t) { freed = 1; }
  static void operator delete(void*, std::size_t, std::align_val_t) { freed = 2; }
};
Wide* volatile kept = nullptr;
int main() {
  kept = new Wide(1);
  delete kept;
  int after_delete = freed;
  freed = 0;
  try { new Wide(-1); } catch (int) {}
  return after_delete * 10 + freed == 11 ? 0 : after_delete * 10 + freed;
}
