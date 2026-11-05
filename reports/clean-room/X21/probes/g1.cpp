int source();
inline int shared_inline = source();          // inline, dynamic
template <class T> struct Holder { static int v; };
template <class T> int Holder<T>::v = source();
int plain = source();                         // ordinary, dynamic
extern thread_local int tls;
int use_all() { return shared_inline + Holder<int>::v + plain + tls; }
