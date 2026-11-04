// stats.h: the public face of the stats package (like an Ada package spec).
#ifndef W14_P2_STATS_H
#define W14_P2_STATS_H

namespace stats {

// External linkage (LNK-1): other units call these.
int total(const int* values, int count);
int mean(const int* values, int count);
int calls();   // how many times the private helper ran

}  // namespace stats

#endif
