// tidal-rules: ODR-4 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Guard::~Guard\(\)
// tidal-error-gcc: undefined reference to .Guard::~Guard\(\)
struct Guard { Guard() {} ~Guard(); };
int main() { Guard guard; return 0; }
