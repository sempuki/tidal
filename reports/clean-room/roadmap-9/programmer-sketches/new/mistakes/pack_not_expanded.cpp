template <class... A> int count(A... a) { return sizeof...(a); }
template <class... A> int wrap(A... a) { return count(a); }
int main() { return wrap(1, 2); }
