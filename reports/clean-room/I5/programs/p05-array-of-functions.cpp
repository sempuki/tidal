// notes 15: an array of functions or of void is not a valid type, so the template drops out.
template <class T> int f(T (*)[1]) { return 1; }
template <class T> int f(...) { return 2; }
int main() { return f<int(int)>(0) == 2 && f<void>(0) == 2 && f<int>(0) == 1 ? 0 : 1; }
