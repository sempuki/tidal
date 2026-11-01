// tidal-rules: INST-28
// tidal-expect: link-fail
// tidal-error: undefined reference to
// tidal-sources: explicit-declaration-no-definition-member.tu.cpp
// tidal-note: get is inline and put is not. No unit has an explicit instantiation definition. INST-28 allows any result. Every compiler that links reports the missing definition.
template <class T> struct Box { int get() { return sizeof(T); } int put(); };
template <class T> int Box<T>::put() { return 1; }
extern template struct Box<int>;
int main() { Box<int> b; return b.get() + b.put() == sizeof(int) + 1 ? 0 : 1; }
