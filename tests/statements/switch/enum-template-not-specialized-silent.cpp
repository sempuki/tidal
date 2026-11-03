// tidal-rules: SW-7
// tidal-expect: accept
// tidal-flags: -Wall -Werror
enum Light { red, amber, green };
template <class T> int code(T v) {
  switch (v) {            // no specialization, so no enumeration type yet
    case red: return 1;
  }
  return 0;
}
int main() { return 0; }
