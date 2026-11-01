// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: call to constructor of .Split. is ambiguous
// tidal-error-gcc: call of overloaded .Split\(<brace-enclosed initializer list>\). is ambiguous
// tidal-error-edg: more than one instance of constructor .Split::Split. matches the argument list
struct Split { Split(int, long); Split(long, int); };
Split s{1, 1};
