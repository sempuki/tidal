"""Read-only counts on a tree made by git archive. Usage: count.py TREE"""
import os, re, sys, json, collections

def pages(tree):
    out = []
    for d, _, fs in os.walk(os.path.join(tree, 'specification')):
        for f in fs:
            if f.endswith('.md') and f not in ('index.md', 'glossary.md', 'rules.md'):
                out.append(os.path.join(d, f))
    return sorted(out)

BLOCK = re.compile(r'^:::\{(rule|principle)\} (\S+)\n(.*?)^:::$', re.M | re.S)

def blocks(tree):
    res = {}
    for p in pages(tree):
        t = open(p).read()
        rel = os.path.relpath(p, os.path.join(tree, 'specification'))[:-3]
        for m in BLOCK.finditer(t):
            kind, ident, body = m.groups()
            fields = dict(re.findall(r'^:([a-z]+): ?(.*)$', body, re.M))
            text = '\n'.join(l for l in body.splitlines() if not re.match(r'^:(tests|rules|exceptions):', l))
            res[ident] = dict(type=kind, page=rel, kind=fields.get('kind'), tests=fields.get('tests', '').split(), rules=fields.get('rules', '').split(), text=text.strip())
    return res

def tests(tree):
    res = {}
    root = os.path.join(tree, 'tests')
    for d, _, fs in os.walk(root):
        for f in fs:
            p = os.path.join(d, f)
            if not (f.endswith('.cpp') or f.endswith('.h')):
                continue
            try:
                head = open(p, errors='replace').read(6000)
            except Exception:
                continue
            if '// tidal-rules' not in head and '// tidal-expect' not in head:
                continue
            hdr = collections.defaultdict(list)
            for l in head.splitlines():
                m = re.match(r'^// (tidal-[a-z0-9-]+):\s?(.*)$', l)
                if m:
                    hdr[m.group(1)].append(m.group(2))
                elif l.strip() and not l.startswith('//'):
                    break
            if 'tidal-expect' not in hdr:
                continue
            res[os.path.relpath(p, root)] = dict(hdr)
    return res

NEWEST = {'clang': ['clang', 'clang-22', 'clang-host'], 'gcc': ['gcc', 'gcc-16', 'gcc-host'], 'edg': ['edg', 'edg-7']}

def questions(tree):
    res = {}
    qd = os.path.join(tree, 'questions')
    for f in sorted(os.listdir(qd)):
        m = re.match(r'(Q-\d{4})', f)
        if not m:
            continue
        t = open(os.path.join(qd, f)).read()
        st = re.search(r'^\*\*Status:\*\* (.*)$', t, re.M)
        wf = re.search(r'^\*\*Waits for:\*\* (.*)$', t, re.M)
        af = re.search(r'^\*\*Affects:\*\* (.*)$', t, re.M)
        res[m.group(1)] = dict(file=f, status=st.group(1).strip() if st else None, waits=wf.group(1) if wf else None,
                               affects=af.group(1) if af else '', text=t)
    return res

def evidence(tree):
    tot = ev = 0
    per = {}
    for p in pages(tree):
        L = open(p).read().splitlines()
        rel = os.path.relpath(p, os.path.join(tree, 'specification'))[:-3]
        e = 0
        start = None
        for i, l in enumerate(L):
            if l.strip() == '## Evidence':
                start = i
        if start is not None:
            end = len(L)
            for j in range(start + 1, len(L)):
                if L[j].startswith('## '):
                    end = j
                    break
            e = end - start
        per[rel] = (e, len(L))
        tot += len(L); ev += e
    return ev, tot, per

def main(tree):
    B = blocks(tree)
    rules = {k: v for k, v in B.items() if v['type'] == 'rule'}
    live = {k: v for k, v in rules.items() if v['kind'] != 'withdrawn'}
    princ = {k: v for k, v in B.items() if v['type'] == 'principle'}
    T = tests(tree)
    cited = set()
    for v in live.values():
        cited.update(v['tests'])
    prog = [t for t in T if t not in cited]
    ov = {t: [k[len('tidal-expect-'):] for k in h if k.startswith('tidal-expect-')] for t, h in T.items()}
    ov = {t: l for t, l in ov.items() if l}
    labels = collections.Counter(x for l in ov.values() for x in l)
    one = collections.Counter(); allthree = []
    for t, l in ov.items():
        fams = [f for f, labs in NEWEST.items() if any(x in labs for x in l)]
        if len(fams) == 1:
            one[fams[0]] += 1
        if len(fams) == 3:
            allthree.append(t)
    errs = sum(1 for h in T.values() if any(k.startswith('tidal-error') for k in h))
    Q = questions(tree)
    st = collections.Counter(q['status'] for q in Q.values())
    ev, tot, _ = evidence(tree)
    res = os.path.join(tree, 'tests', 'RESULTS.md')
    rows = sum(1 for l in open(res) if l.startswith('| ') and '.cpp' in l) if os.path.exists(res) else None
    single = [k for k, v in live.items() if len(v['tests']) == 1]
    print(json.dumps(dict(live=len(live), withdrawn=len(rules) - len(live), principles=len(princ), tests=len(T), cited=len(cited & set(T)),
        program=len(prog), single=single, override_tests=len(ov), labels=labels.most_common(), one_family=dict(one), all_three=allthree,
        error_tests=errs, questions=len(Q), status=dict(st), evidence=[ev, tot], results_rows=rows,
        kinds=collections.Counter(v['kind'] for v in live.values()).most_common()), indent=1))

if __name__ == '__main__':
    main(sys.argv[1])
