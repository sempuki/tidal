template <class T> struct Shape { static int code() { return 0; } };
int code_elsewhere() { return Shape<int*>::code(); }
