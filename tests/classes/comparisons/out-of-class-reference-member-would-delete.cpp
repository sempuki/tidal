// tidal-rules: DCMP-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: would delete it after its first declaration
// tidal-note: Q-0062. The class has a reference member, so a defaulted == would be deleted. The == is defaulted after the class definition. Clang rejects it. GCC and EDG accept it.
struct S { int& r; bool operator==(const S&) const; };
bool S::operator==(const S&) const = default;
