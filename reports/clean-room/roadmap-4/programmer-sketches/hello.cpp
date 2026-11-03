extern "C" int puts(const char*);
int main() { return puts("hello") < 0; }
