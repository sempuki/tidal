// tidal-rules: INST-7 INST-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-note: G157, Q-0428. The call p->get() names the virtual function, so the call instantiates it (INST-7), and the body is not valid for int. GCC and Clang reject. EDG accepts and does not instantiate the function.
template <class T> struct Box { virtual int get() { return T::nothing; } };
int read(Box<int>* p) { return p->get(); }
