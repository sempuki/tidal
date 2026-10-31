// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-note: Q-0057. Clang accepts an anonymous union without static in an unnamed namespace. GCC and EDG reject it.
namespace { union { int i; float f; }; }
int get() { return i; }
