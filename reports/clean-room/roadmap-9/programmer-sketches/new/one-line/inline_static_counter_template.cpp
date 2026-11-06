template <class T> struct Counted { inline static int made = 0; Counted() { ++made; } };
struct A : Counted<A> {}; struct B : Counted<B> {};
int main() { A a1, a2; B b; return Counted<A>::made == 2 && Counted<B>::made == 1 ? 0 : 1; }
