// tidal-rules: NAM-16
// tidal-expect: reject
// tidal-error-clang: number. .aka .int.. is not a class, namespace, or enumeration
// tidal-error-gcc: number. is not a class, namespace, or enumeration
// tidal-error-edg: name followed by .::. must be a class or namespace name
using number = int;
int read() { return number::value; }
