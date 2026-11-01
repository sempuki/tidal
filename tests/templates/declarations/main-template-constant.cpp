// tidal-rules: TPL-29
// tidal-expect: reject
// tidal-error-clang: .main. cannot be a template
// tidal-error-gcc: cannot declare .::main. to be a template
// tidal-error-edg: is not a valid name for a function template
// tidal-note: main is declared as a function template with a constant parameter.
template <int N> int main() { return N; }
