// tidal-rules: INST-18
// tidal-expect: run
// tidal-note: The member get of Box<int> and the variable template code<int> have explicit specializations. The explicit instantiations after them have no effect.
template <class T> struct Box { int get() { return T::nothing; } };
template <> int Box<int>::get() { return 2; }
template int Box<int>::get();
template <class T> int code = 1;
template <> int code<int> = 3;
template int code<int>;
int main() { return Box<int>{}.get() == 2 && code<int> == 3 ? 0 : 1; }
