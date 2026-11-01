// tidal-rules: TSP-4
// tidal-expect: reject
// tidal-error-clang: no function template matches function template
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: no instance of function template
// tidal-note: No function template f has a specialization with a parameter of type char*.
template <class T> int f(T*, int);
template <> int f(char*) { return 0; }
