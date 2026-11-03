// tidal-rules: NEW-29
// tidal-expect: run
// tidal-note: The array forms. The class declares only the unaligned array functions, and new[] and delete[] use them.
using size = decltype(sizeof 0);
alignas(256) unsigned char pool[2048];
int allocated = 0, freed = 0;
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Wide {
  int value = 3;
  static void* operator new[](size) { ++allocated; return pool; }
  static void operator delete[](void*) noexcept { ++freed; }
};
int main() {
  Wide* w = new Wide[2];
  if (allocated != 1 || w[1].value != 3) return 1;
  delete[] w;
  return freed == 1 ? 0 : 2;
}
