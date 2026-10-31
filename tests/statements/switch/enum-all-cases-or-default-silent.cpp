// tidal-rules: SW-7
// tidal-expect: accept
// tidal-flags: -Wall -Werror
enum class Light { red, amber, green };
int with_default(Light l) {
  switch (l) {
    case Light::red: return 1;
    default: return 0;
  }
}
int every_case(Light l) {
  switch (l) {
    case Light::red: return 1;
    case Light::amber: return 2;
    case Light::green: return 3;
  }
  return 0;
}
int main() { return with_default(Light::red) + every_case(Light::green) == 4 ? 0 : 1; }
