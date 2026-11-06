// A constrained class template in a return type: failure removes the candidate.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> Box<T> make(T t) { return {t}; }
int make(...) { return 7; }
int main() { return make('a') == 7 && make(5).v == 5 ? 0 : 1; }
