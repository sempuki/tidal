// The member is defined in the class, which comes after a use in a later inline variable? Class first, then n reads m.
#include <cstdio>
int seven();
struct S { static inline int m = seven(); };
inline int n = S::m + 1;
int seven() { return 7; }
int main() { std::printf("%d\n", n); std::fflush(stdout); return n == 8 ? 0 : 1; }
