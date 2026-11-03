// tidal-rules: SW-7
// tidal-expect: reject
// tidal-error-clang: not handled in switch
// tidal-error-gcc: not handled in switch
// tidal-flags: -Wall -Werror
enum Light { red, amber, green };
template <class T> int code(T v) {
  switch (v) {            // the warning comes with the specialization for Light
    case red: return 1;
  }
  return 0;
}
int main() { return code(red) == 1 ? 0 : 1; }
