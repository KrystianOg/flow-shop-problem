import { describe, it, beforeEach } from 'vitest';
// import { NEHAlgorithm } from '../neh.js';
import { loadData, toJobs } from '../utils/load-data.js';
import { neh, calculateCmax } from '../my-neh.js';

/**returns time in seconds */
function time(cb: () => void): number {
  const start = new Date().valueOf();
  cb();
  const end = new Date().valueOf();

  return (end - start) / 1000;
}

describe('NEH benchmark', () => {
  // Array of filenames (or parameters) you want to run the tests for
  const filenames = [
    'problem1_TPO.csv',
    'problem2_TPO.csv',
    'problem3_TPO.csv',
    'problem55_TPO.csv',
    'problem69_TPO.csv',
  ];

  it.each(filenames)('problems', (filename) => {
    const data = loadData(filename);

    let result: number[] = [];

    const timeTaken = time(() => {
      result = neh(data);
    });

    console.log(`result: ${result}`);
    const cmax = calculateCmax(result, data);

    console.log('cmax', cmax, `\tfor case: ${filename}`);

    // console.log(`calculation for ${filename} took: ${timeTaken.toFixed(3)}s`);
  });
});
