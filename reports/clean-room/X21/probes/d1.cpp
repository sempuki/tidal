// Destruction across units follows the reverse of completed initialization.
#include <cstdio>
#include <cstdlib>
extern char log_[4]; extern int pos;
struct Note { char c; Note(char x) : c(x) { std::printf("make %c\n", c); std::fflush(stdout); }
  ~Note() { std::printf("end %c\n", c); std::fflush(stdout); } };
volatile char ca = 'a';
Note a(ca);
int main() { return 0; }
