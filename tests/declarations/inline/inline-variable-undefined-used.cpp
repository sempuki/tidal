// tidal-rules: INL-6
// tidal-expect: reject
// tidal-error-clang: inline variable .shared. is not defined
// tidal-error-gcc: inline variable .shared. is not defined
// tidal-error-edg: inline variable .shared. was referenced but not defined
extern inline int shared;
int main() { return shared; }
