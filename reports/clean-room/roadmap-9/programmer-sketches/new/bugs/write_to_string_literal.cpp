int main() { char* p = const_cast<char*>("abc"); p[0] = 120; return p[0] == 120 ? 0 : 1; }
