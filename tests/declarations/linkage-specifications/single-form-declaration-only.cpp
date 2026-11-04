// tidal-rules: LSP-3 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .never_defined
// tidal-error-gcc: undefined reference to .never_defined
extern "C" int never_defined;
int main() { return never_defined; }
