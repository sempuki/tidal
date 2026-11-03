// tidal-rules: FN-7
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: control reaches end of non-void function
// tidal-flags: -Wall -Werror
// tidal-note: Clang gives no warning when a switch over a scoped enumeration returns in a case for every enumerator. GCC warns, because the condition can hold a value with no enumerator (ENUM-22).
enum class Light { red, green };
int code(Light l) {
  switch (l) {
    case Light::red: return 1;
    case Light::green: return 2;
  }
}
int main() { return code(Light::red) == 1 ? 0 : 1; }
