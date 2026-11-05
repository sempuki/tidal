#include <cstdio>
#include <cstdlib>
struct Unit { int done = 0; int running = -1; };   // zero before any code (constant-initialized)
inline void step(Unit& u, const char* name, int k, void (*run)(int)) {
  while (u.done < k) {
    if (u.running == u.done) { std::fprintf(stderr, "initialization cycle at %s position %d\n", name, u.done); std::fflush(stderr); std::abort(); }
    u.running = u.done; run(u.done); u.running = -1; ++u.done;
  }
}
