template <class T> concept small = sizeof(T) <= 2;
template <class T> requires small<T> int code(T v) { return static_cast<int>(v); }
int main() { return code(0LL); }
