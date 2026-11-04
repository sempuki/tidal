// tidal-rules: TR-3
// tidal-expect: reject
// tidal-error-clang: not allowed to be declared consteval
// tidal-error-gcc: to be .consteval.
// tidal-error-edg: may not be declared consteval
consteval int main() { return 0; }
