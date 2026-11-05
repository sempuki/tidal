#include <cstdio>
struct Note { char c; Note(char x) : c(x) { std::printf("make %c\n", c); std::fflush(stdout); }
  ~Note() { std::printf("end %c\n", c); std::fflush(stdout); } };
volatile char cb = 'b';
Note b(cb);
