// stats.cpp: the package body. The helpers below are private to this unit.
#include "stats.h"

namespace stats {

// An unnamed namespace gives its members internal linkage (LNK-5, NS-4).
// Code in this unit names them without a qualifier.
namespace {
int helper_calls = 0;

int add(int a, int b) {
  ++helper_calls;
  return a + b;
}
}  // namespace

// `static` at namespace scope also gives internal linkage (LNK-2).
static int divide_down(int sum, int count) { return count == 0 ? 0 : sum / count; }

int total(const int* values, int count) {
  int sum = 0;
  for (int i = 0; i < count; ++i) sum = add(sum, values[i]);
  return sum;
}

int mean(const int* values, int count) { return divide_down(total(values, count), count); }

int calls() { return helper_calls; }

}  // namespace stats
