// tidal-rules: SW-7
// tidal-expect: accept
// tidal-flags: -Wall -Werror
enum Level { low = 0, minimum = 0, high = 1 };
int code(Level l) {
  switch (l) {            // the case for low also covers minimum
    case low: return 1;
    case high: return 2;
  }
  return 0;
}
int main() { return code(minimum) == 1 ? 0 : 1; }
