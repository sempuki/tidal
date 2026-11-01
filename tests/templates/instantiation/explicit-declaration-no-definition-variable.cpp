// tidal-rules: INST-28
// tidal-expect: link-fail
// tidal-error: undefined reference to
// tidal-sources: explicit-declaration-no-definition-variable.tu.cpp
// tidal-note: A variable template and a static data member. No unit has an explicit instantiation definition. INST-28 allows any result. Every compiler that links reports the missing definition.
template <class T> int zero = 1;
template <class T> struct Box { static int count; };
template <class T> int Box<T>::count = 1;
extern template int zero<int>;
extern template struct Box<int>;
int main() { return zero<int> + Box<int>::count == 2 ? 0 : 1; }
