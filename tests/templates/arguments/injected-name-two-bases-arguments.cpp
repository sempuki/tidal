// tidal-rules: TARG-11
// tidal-expect: accept
// tidal-note: With an argument list, the injected class name from two bases names the template.
template <class T> struct Base {};
struct Both : Base<int>, Base<char> { Base<long>* p = nullptr; };
