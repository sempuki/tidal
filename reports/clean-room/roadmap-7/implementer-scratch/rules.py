import re,sys,glob,os,collections
kinds={}
for f in glob.glob('specification/**/*.md',recursive=True):
    t=open(f).read()
    for m in re.finditer(r':::\{rule\} ([A-Z]+-\d+)\n:kind: (\S+)',t):
        kinds[m.group(1)]=(m.group(2),f)
rej=[r for r,(k,f) in kinds.items() if k=='reject']
src=''
for f in glob.glob('frontend/src/**/*.rs',recursive=True): src+=open(f).read()
absent=[r for r in rej if not re.search(r'"'+re.escape(r)+r'"|\b'+re.escape(r)+r'\b',src)]
# first-named rules on passing rejects: from conform -v RULE-ID lines (other-rule) and cited counts are not listed; use check on tests instead
print('reject rules',len(rej),'absent from frontend src',len(absent))
bypage=collections.Counter(kinds[r][1].replace('specification/','').replace('.md','') for r in absent)
for p,c in bypage.most_common(15): print(c,p)
open(os.path.join(sys.argv[1],'absent.txt'),'w').write('\n'.join(sorted(absent)))
