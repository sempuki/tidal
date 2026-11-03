// tidal-rules: NEW-13
// tidal-expect: run
// tidal-note: The array forms. new[] and delete[] for the class use the class functions, and ::new[] and ::delete[] use the global ones.
using size = decltype(sizeof 0);
int class_new = 0, class_delete = 0;
alignas(16) unsigned char buf[128];
struct S {
  int v = 0;
  static void* operator new[](size) { ++class_new; return buf; }
  static void operator delete[](void*) noexcept { ++class_delete; }
};
int main() {
  S* a = new S[2];
  S* b = ::new S[2];
  if (class_new != 1 || a == b) return 1;
  ::delete[] b;
  if (class_delete != 0) return 2;
  delete[] a;
  return class_delete == 1 ? 0 : 3;
}
