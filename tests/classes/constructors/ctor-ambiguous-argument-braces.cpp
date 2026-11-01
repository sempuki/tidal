// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: call to constructor of .Split. is ambiguous
// tidal-error-gcc: conversion from .<brace-enclosed initializer list>. to .Split. is ambiguous
// tidal-error-edg: more than one instance of constructor .Split::Split. matches the argument list
// tidal-note: Holder(Split) is the only viable constructor. No constructor of Split is best for its argument {1, 1}.
struct Split { Split(int, long); Split(long, int); };
struct Holder { Holder(Split); };
Holder h{{1, 1}};
