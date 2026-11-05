// Unit 1:  int a after(d) = d + 1;   int b = 10;
#include "cyc.h"
extern int d; void u2_upto(int);
int a, b;
constinit Unit u1;
static void run1(int i) { if (i == 0) { u2_upto(2); a = d + 1; } else b = 10; }
void u1_upto(int k) { step(u1, "unit 1", k, run1); }
static int s1 = (u1_upto(2), 0);
int main() { std::printf("a=%d b=%d\n", a, b); std::fflush(stdout); return 0; }
