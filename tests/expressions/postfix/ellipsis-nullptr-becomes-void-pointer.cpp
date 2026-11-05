// tidal-rules: PFX-22
// tidal-expect: run
// tidal-flags: -Wall
// tidal-note: Gap review of the text phase 8 wrote on expressions. An argument of the type of nullptr becomes a pointer to void, so va_arg reads it back as void*. EDG's strict mode has no va_list, so the test uses -Wall, which EDG skips and which changes no outcome on Clang and GCC.
#include <cstdarg>
int check(int count, ...) {
  va_list list;
  va_start(list, count);
  void* p = va_arg(list, void*);
  va_end(list);
  return p == nullptr ? 0 : 1;
}
int main() { return check(1, nullptr); }
