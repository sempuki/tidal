// tidal-rules: SW-8
// tidal-expect: reject
// tidal-error-clang: case value not in enumerated type
// tidal-error-gcc: case value .5. not in enumerated type
// tidal-flags: -Wall -Werror
enum Light { red, amber };
int code(Light l) {
  switch (l) {
    case red: return 1;
    case amber: return 2;
    case 5: return 3;     // no enumerator has the value 5
  }
  return 0;
}
int main() { return code(red) == 1 ? 0 : 1; }
