// tidal-rules: ODR-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .count
// tidal-error-gcc: redefinition of .int Box::count
// tidal-error-edg: has already been defined
struct Box { static int count; };
int Box::count = 0;
int Box::count = 1;
int main() { return 0; }
