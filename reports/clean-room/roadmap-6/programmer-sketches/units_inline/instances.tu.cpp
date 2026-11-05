// instances.cpp: the explicit instantiation definitions (INST-16, INST-17).
// The header's declarations come first, as INST-26 allows.
#include "fixed.h"

namespace fx {
template class Fixed<16>;
template struct Stats<Q16>;
template Q16 weighted_mean<Q16>(const Q16*, const int*, int);
template Stats<Q16> summarize<Q16>(const Q16* values, int count);
}
