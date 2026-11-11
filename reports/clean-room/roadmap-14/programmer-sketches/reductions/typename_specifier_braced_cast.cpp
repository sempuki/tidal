template <class T> struct Box { using type = T; };
template <class T> constexpr int make() { return typename Box<T>::type{3} + typename Box<T>::type(4); }
static_assert(make<int>() == 7);
int main() { return make<long>() - 7; }
