struct P { unsigned short a; unsigned short b; };
int main() { P p{0x0102, 0x0304}; const unsigned char* b = reinterpret_cast<const unsigned char*>(&p); int s = 0; for (unsigned i = 0; i < sizeof p; ++i) s += b[i]; return s == 1 + 2 + 3 + 4 ? 0 : 1; }
