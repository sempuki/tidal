template <class T> struct Slot { static int code() { return 0; } };
int code_elsewhere() { return Slot<char>::code(); }
