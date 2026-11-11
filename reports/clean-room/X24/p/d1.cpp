// X24 probe d1: the order in which a goto out of nested loops ends
// objects. The goto models `break outer;` (default) and, with -DCONT,
// `continue outer;` (a goto to the end of the outer body).
#include <cstdio>
struct T {
  const char* n;
  T(const char* s) : n(s) { std::printf("+%s ", n); }
  ~T() { std::printf("-%s ", n); }
  explicit operator bool() const { return true; }
};
struct Range {
  int v[2] = {1, 2};
  Range() { std::printf("+range "); }
  ~Range() { std::printf("-range "); }
  const int* begin() const { return v; }
  const int* end() const { return v + 2; }
};
int main() {
  int pass = 0;
  for (T a("outerinit"); pass < 2; ++pass) {
    T b("outerbody");
    for (int x : Range()) {
      T c("rangebody");
      for (T d("innerinit"); T e = T("innercond");) {
        T f("innerbody");
        (void)x;
#ifdef CONT
        goto next;
#else
        goto done;
#endif
      }
    }
#ifdef CONT
  next:;
    std::printf("| ");
#endif
  }
done:
  std::printf("end\n");
}
