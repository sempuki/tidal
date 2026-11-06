// A default template argument names Box<T>. The default is formed before the
// constraint of the template is checked.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T, class B = Box<T>> requires Wide<T> int pick(T) { return 1; }
int pick(...) { return 2; }
int main() { return pick('a') == 2 && pick(5) == 1 ? 0 : 1; }
