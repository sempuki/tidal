// tidal-rules: PFX-22
// tidal-expect: run
// tidal-flags: -Wall
// tidal-note: Propose exercise X20. The test reads each argument back with va_arg from <cstdarg>, as the type that PFX-22 gives it. EDG's strict mode has no va_list, so the test uses -Wall, which EDG skips and which changes no outcome on Clang and GCC.
#include <cstdarg>
enum class Scoped : short { a = 7 };
enum Plain : char { b = 9 };
int ten() { return 10; }
int check(int count, ...) {
  va_list list;
  va_start(list, count);
  double real = va_arg(list, double);         // from float
  int small = va_arg(list, int);              // from short
  int letter = va_arg(list, int);             // from char
  int flag = va_arg(list, int);               // from bool
  int plain = va_arg(list, int);              // from an unscoped enumeration
  Scoped scoped = va_arg(list, Scoped);       // a scoped enumeration keeps its type
  const char* text = va_arg(list, const char*); // from an array
  int (*call)() = va_arg(list, int (*)());    // from a function
  va_end(list);
  return real == 1.5 && small == 3 && letter == 'x' && flag == 1 && plain == 9 &&
         scoped == Scoped::a && text[0] == 'h' && call() == 10 ? 0 : 1;
}
int main() {
  float real = 1.5f;
  short small = 3;
  char letter = 'x';
  bool flag = true;
  return check(8, real, small, letter, flag, b, Scoped::a, "hi", ten);
}
