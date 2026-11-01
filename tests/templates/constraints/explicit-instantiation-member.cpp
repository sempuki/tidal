// tidal-rules: CNS-13
// tidal-expect: accept
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Box { void f() requires Wide<T> {} };
template void Box<int>::f();
