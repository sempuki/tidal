// tidal-rules: FN-4
// tidal-expect: reject
// tidal-error-clang: main. cannot be a template
// tidal-error-gcc: cannot declare .::main. to be a template
// tidal-error-edg: main. is not a valid name for a function template
template <class T> int main() { return 0; }
