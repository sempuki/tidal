// tidal-rules: FN-4
// tidal-expect: reject
// tidal-error-clang: main. is not allowed to be declared consteval
// tidal-error-gcc: cannot declare .::main. to be .consteval
// tidal-error-edg: main. may not be declared consteval
consteval int main() { return 0; }
