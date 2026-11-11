// An overload on decltype(nullptr) beside one on a pointer, and 0 that picks int.
int which(int) { return 1; }
int which(const char*) { return 2; }
int which(decltype(nullptr)) { return 3; }
int main() { return which(0) == 1 && which("s") == 2 && which(nullptr) == 3 ? 0 : 1; }
