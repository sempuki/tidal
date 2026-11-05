// Where does an inline static data member defined in its class sit among inline variables?
#include <cstdio>
int trace = 0;
int mark(int d) { trace = trace * 10 + d; return d; }
inline int before = mark(1);
struct S { static inline int m = mark(2); };
inline int after_v = mark(3);
int main() { std::printf("%d\n", trace); std::fflush(stdout); return 0; }
