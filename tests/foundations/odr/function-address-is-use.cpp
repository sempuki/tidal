// tidal-rules: ODR-3 ODR-8
// tidal-expect: link-fail
// tidal-expect-gcc-16-o2: run
// tidal-expect-clang-22-o2: run
// tidal-error-clang: undefined reference to .declared_only\(\)
// tidal-error-gcc: undefined reference to .declared_only\(\)
// tidal-note: At -O2, GCC and Clang drop the unused pointer and emit no reference to declared_only, so the program links (ODR-8 needs no diagnostic).
void declared_only();
int main() { void (*handler)() = declared_only; (void)handler; return 0; }
