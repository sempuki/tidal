// tidal-rules: FN-4
// tidal-expect: reject
// tidal-error-clang: main. is not allowed to be declared constexpr
// tidal-error-gcc: cannot declare .::main. to be .constexpr
// tidal-error-edg: main. may not be declared constexpr
constexpr int main() { return 0; }
