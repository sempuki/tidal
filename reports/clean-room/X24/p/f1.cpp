// X24 probe f1: FN-7 and a loop whose condition is constant true.
int a(int k) {            // the break leaves only the switch
  while (true) {
    switch (k) { case 0: break; }
    if (k > 3) return k;
  }
}
int b(int k) {            // a goto leaves the loop
  while (true) {
    if (k) goto out;
  }
out:;
}
int c(int k) {            // a break inside an inner loop
  while (true) {
    for (;;) { if (k) break; }
    if (k > 3) return k;
  }
}
int main() { return a(4) + c(4) - 8; }
