// X24 probe t1: when the label of a goto is checked.
#ifdef TEMPLATE
template <class T> void f() { goto nowhere; }   // never instantiated
#endif
#ifdef DISCARDED
void g() { if constexpr (false) { goto nowhere; } }
#endif
#ifdef TEMPLATE_DISCARDED
template <class T> void h() { if constexpr (sizeof(T) == 0) { goto nowhere; } }
void use() { h<int>(); }
#endif
#ifdef TEMPLATE_BREAK
template <class T> void k() { break; }          // never instantiated
#endif
int main() {}
