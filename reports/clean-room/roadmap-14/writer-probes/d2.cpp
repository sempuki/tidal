// Round 14 writer probe d2 (X24-1): the order in which other exits end
// the variables of a statement's own scope. CASE selects the exit.
// 1: a return out of nested loops (the structure of X24's d1.cpp)
// 2: a throw out of nested loops, caught in main
// 3: a goto out of an if with an init-statement and a condition variable
// 4: a goto out of a while with a condition variable
// 5: a goto out of a switch with an init-statement
// 6: a break out of a for whose initializer and condition declare variables
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
void nested() {
  for (T a("outerinit"); T g = T("outercond");) {
    T b("outerbody");
    for (int x : Range()) {
      T c("rangebody");
      for (T d("innerinit"); T e = T("innercond");) {
        T f("innerbody");
        (void)x;
#if CASE == 1
        return;
#else
        throw 0;
#endif
      }
    }
  }
}
int main() {
#if CASE == 1 || CASE == 2
  try { nested(); } catch (int) { std::printf("caught "); }
#elif CASE == 3
  if (T a("ifinit"); T b = T("ifcond")) {
    T c("ifbody");
    goto done;
  }
#elif CASE == 4
  while (T e = T("whilecond")) {
    T f("whilebody");
    goto done;
  }
#elif CASE == 5
  switch (T a("swinit"); 0) {
    default: {
      T b("swbody");
      goto done;
    }
  }
#elif CASE == 6
  for (T d("forinit"); T e = T("forcond");) {
    T f("forbody");
    break;
  }
#endif
#if CASE >= 3
done:
#endif
  std::printf("end\n");
}
