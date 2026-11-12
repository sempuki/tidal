template <class D> struct Mixin { int tag = 1; };
template <class T> struct Box : Mixin<Box<T> > {};
int main() { return Box<int>{}.tag - 1; }
