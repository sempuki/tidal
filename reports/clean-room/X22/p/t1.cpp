// A template template parameter with its own constraint, used with a
// dependent argument. The argument template has no constraint.
template <class T> concept Wide = sizeof(T) > 1;
template <template <Wide> class TT, class T> struct Use { TT<T> m; };
template <class T> struct Box {};
Use<Box, char> u;
int main() {}
