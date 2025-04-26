import { afterAll, describe, it } from 'vitest';
// import { NEHAlgorithm } from '../neh.js';
import { loadData } from '../utils/load-data.js';
import { neh, calculateCmax } from '../my-neh.js';
import { time } from '../utils/time.js';
import { readTaillardFile } from '../utils/taillard.js';

describe('NEH benchmark', () => {
  // Array of filenames (or parameters) you want to run the tests for
  const filenames = [
    'problem1_TPO.csv',
    'problem2_TPO.csv',
    'problem3_TPO.csv',
    'problem55_TPO.csv',
    'problem69_TPO.csv',
  ];

  it.skip.each(filenames)('problems', (filename) => {
    const data = loadData(filename);

    let result: number[] = [];

    console.log(data);

    const timeTaken = time(() => {
      result = neh(data);
    });

    console.log(`size: ${data.length}x${data[0].length}`);
    console.log(`result: ${result}`);
    const cmax = calculateCmax(result, data);

    console.log('cmax', cmax, `\tfor case: ${filename}`);
  });
});

describe('Taillard benchmark', () => {
  describe('NEH', { timeout: 60000 }, () => {
    const files = [
      'tai20_5',
      'tai20_10',
      'tai20_20',
      'tai50_5',
      'tai50_10',
      'tai50_20',
      'tai100_5',
      'tai100_10',
      'tai100_20',
      'tai200_20',
      // 'tai500_20',
    ];

    const problems = files
      .map((file) => readTaillardFile(`${file}.txt`))
      .flat();

    let allResults: number[][] = [];

    it.each(problems)(`$jobs x $machines LB: $lowerBound`, (problem) => {
      let result: number[] = [];

      const timeTaken = time(() => {
        result = neh(problem.processingTimes);
      });

      const cmax = calculateCmax(result, problem.processingTimes);

      const diff = cmax - problem.lowerBound;
      const percentComplete = diff / problem.lowerBound;
      allResults.push([
        cmax,
        problem.lowerBound,
        timeTaken,
        percentComplete,
        diff,
      ]);
    });

    afterAll(() => {
      console.table(allResults);
    });
  });
});
