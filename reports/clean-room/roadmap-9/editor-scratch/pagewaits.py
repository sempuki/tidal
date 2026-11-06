import sys,json,collections; sys.path.insert(0,'.')
from count import questions
Q=questions('t9'); P=json.load(open('pages_t9.json'))
def k(w):
    if 'the maintainer' in w: return 'maint'
    if 'newest' in w: return 'row1'
    for x in ['first review','against','silent','doubt','part','principle','child']:
        if x in w: return x
    return w
for p in ['overloading/conversion-sequences','foundations/storage','overloading/overload-resolution','classes/members','expressions/new-delete','foundations/constant-evaluation','templates/class-template-deduction','templates/ordering','expressions/conditional','foundations/language-support']:
    print(p, dict(collections.Counter(k(Q[q]['waits']) for q in P[p]['Q'])))
