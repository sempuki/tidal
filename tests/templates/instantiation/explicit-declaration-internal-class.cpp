// tidal-rules: INST-29
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: explicit instantiation declaration of 'Box' with internal linkage
// tidal-note: The class template is in an unnamed namespace. Clang rejects the explicit instantiation declaration, GCC and EDG accept it (Q-0234).
namespace {
template <class T> struct Box {};
extern template struct Box<int>;
}
