// tidal-rules: ATTR-7
// tidal-expect: accept
// tidal-flags: -Werror -Wimplicit-fallthrough
[[noreturn]] void stop();
int pick(int k) {
  switch (k) {
    case 0:
      stop();
    case 1:
      return 1;
  }
  return 0;
}
