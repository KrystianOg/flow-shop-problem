# Some data

https://www.sciencedirect.com/science/article/pii/S2352340921002699
https://data.mendeley.com/datasets/58x5fxx67y/1

from this:
https://arxiv.org/html/2210.17178v2/#S5

Taillard Benchmark
http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/ordonnancement.html

https://www.sciencedirect.com/science/article/abs/pii/S0377221714005992

different variants: NEH, NEH-D (Dong, Huang and Chen 2008) 11.9% gain on average to NEH but O(m x n^2)

more complex heuristic FRB3 48.0% gain on average but at cost of O(m x n^4)

## RPD - Relative Percentage Deviation

(Method_sol - Best_sol) / Best_sol * 100

https://www.researchgate.net/publication/266748671_New_hard_benchmark_for_flowshop_scheduling_problems_minimising_makespan

https://people.brunel.ac.uk/~mastjjb/jeb/orlib/flowshopinfo.html

## Downloading taillard-benchmark files

I've created quich bash script downloading all files from Taillard's benchmark.

Usage is simple:

```bash
chmod +x taillard.sh
./taillard.sh
```

this saves files in taillard-benchmark directory (Linux / MacOS).


