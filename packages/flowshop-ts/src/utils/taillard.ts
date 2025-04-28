import fs from 'fs';
import path from 'path';

export interface TaillardProblem {
  jobs: number;
  machines: number;
  seed: number;
  upperBound: number;
  lowerBound: number;
  processingTimes: number[][]; // [jobs][machines]
}

export function readTaillardFile(filename: string): TaillardProblem[] {
  const filePath = path.resolve(process.cwd(), 'taillard-benchmark', filename);

  const fileContent = fs.readFileSync(filePath, 'utf-8');

  const lines = fileContent
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line.length > 0);

  const problems: TaillardProblem[] = [];

  let i = 0;

  while (i < lines.length) {
    if (lines[i].startsWith('number of jobs')) {
      // const metaLine = lines[++i].split(/s+/).map(Number);
      const metaLine = lines[++i].split(/\s+/).map((v) => parseInt(v));
      const [jobs, machines, seed, upperBound, lowerBound] = metaLine;

      // Skip the 'processing times : line
      i++;

      const processingTimes: number[][] = Array.from(
        { length: jobs },
        () => [],
      );

      for (let jobIdx = 0; jobIdx < machines; jobIdx++) {
        const processingLine = lines[++i].split(/\s+/).map(Number);
        for (let j = 0; j < jobs; j++) {
          processingTimes[j].push(processingLine[j]);
        }
      }

      problems.push({
        jobs,
        machines,
        seed,
        upperBound,
        lowerBound,
        processingTimes,
      });
    }
    i++;
  }

  return problems;
}
