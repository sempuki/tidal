// tidal-rules: TR-4 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .declared_only\(\)
// tidal-error-gcc: undefined reference to .declared_only\(\)
int declared_only();
int main() { return declared_only(); }
