// tidal-rules: INST-2
// tidal-expect: accept
template <class T> struct Box { static_assert(sizeof(T) == 0); int v; };
template <class X> struct Holder {};
Box<int>* p = nullptr;
void take(Box<int>& r);
Box<int> make();
using Name = Box<int>;
Holder<Box<int>> h;
bool same(Box<int>* a, Box<int>* b) { return a == b; }
Box<int>* back(void* v) { return static_cast<Box<int>*>(v); }
