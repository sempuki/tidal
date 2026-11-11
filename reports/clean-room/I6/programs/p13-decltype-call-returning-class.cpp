// INST-1, INST-7 (notes 19): decltype of a call that returns a class specialization.
template <class T> struct Box { static_assert(sizeof(T) == 0); };
Box<int> make();
using R = decltype(make());
int main() { return 0; }
