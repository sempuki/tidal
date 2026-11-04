// tidal-rules: FN-7
// tidal-expect: accept
// tidal-flags: -Werror -Wreturn-type
// tidal-note: Reaching the end of main returns zero, so FN-7 asks for no warning. Clang and GCC give none.
int main(int argc, char**) {
  if (argc > 5) return 1;
}
