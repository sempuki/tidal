// tidal-rules: UN-19
// tidal-expect: accept
// tidal-flags: -Werror -Wsizeof-array-argument
unsigned long f(int (&a)[10]) {
  return sizeof(a);
}
