// Unit 2:  int c after(b) = b + 1;   int d = 20;
#include "cyc.h"
extern int b; void u1_upto(int);
int c, d;
constinit Unit u2;
static void run2(int i) { if (i == 0) { u1_upto(2); c = b + 1; } else d = 20; }
void u2_upto(int k) { step(u2, "unit 2", k, run2); }
static int s2 = (u2_upto(2), 0);
