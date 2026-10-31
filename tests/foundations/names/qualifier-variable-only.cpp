// tidal-rules: NAM-16
// tidal-expect: reject
// tidal-error-clang: box. is not a class, namespace, or enumeration
// tidal-error-gcc: box. is not a class, namespace, or enumeration
// tidal-error-edg: name followed by .::. must be a class or namespace name
int box = 0;
int read() { return box::value; }
