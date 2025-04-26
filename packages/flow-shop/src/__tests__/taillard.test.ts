import { expect, it } from 'vitest';
import { readTaillardFile } from '../utils/taillard.js';

it('reads the problems file properly', () => {
  const problems = readTaillardFile('tai20_5.txt');
  const problem = problems[0];

  expect(problems).toHaveLength(10);
  expect(problem.jobs).toBe(20);
  expect(problem.machines).toBe(5);
  expect(problem.seed).toBe(873654221);
  expect(problem.upperBound).toBe(1278);
  expect(problem.lowerBound).toBe(1232);
  expect(problem.processingTimes).toHaveLength(20);
  expect(problem.processingTimes[0]).toHaveLength(5);
});
