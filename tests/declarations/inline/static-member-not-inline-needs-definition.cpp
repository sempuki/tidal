// tidal-rules: INL-4 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .S::count
// tidal-error-gcc: undefined reference to .S::count
struct S { static int count; };
int main() { return S::count; }
