// Unit b:  int slots after(config) = config * 2;
#include <cstdio>
extern int config;
void a_init_upto(int k);
int slots = (a_init_upto(2), config * 2);
int main() { std::printf("slots=%d\n", slots); std::fflush(stdout); return slots == 28 ? 0 : 1; }
