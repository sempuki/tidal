// tidal-rules: SW-7
// tidal-expect: reject
// tidal-error-clang: not handled in switch
// tidal-error-gcc: not handled in switch
// tidal-flags: -Wall -Werror
enum Light { red, amber, green };
struct Signal { operator Light() const { return red; } };
int code(Signal s) {
  switch (s) {            // the condition converts to Light (SW-2)
    case red: return 1;
  }
  return 0;
}
int main() { return code(Signal{}) == 1 ? 0 : 1; }
