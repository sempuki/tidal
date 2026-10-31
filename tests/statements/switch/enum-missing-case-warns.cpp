// tidal-rules: SW-7
// tidal-expect: reject
// tidal-error-clang: not handled in switch
// tidal-error-gcc: not handled in switch
// tidal-flags: -Wall -Werror
enum Light { red, amber, green };
int code(Light l) {
  switch (l) {
    case red: return 1;
    case amber: return 2;
  }
  return 0;
}
int main() { return code(red) == 1 ? 0 : 1; }
