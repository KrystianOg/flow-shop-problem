import { describe, it, expect, beforeEach } from 'vitest';
import { NEHAlgorithm, DefaultMakespanCalculator } from '../neh.js';

describe('NEH heuristic', () => {
  let neh: NEHAlgorithm;

  beforeEach(() => {
    neh = new NEHAlgorithm();
  });

  it('should handle empty job list', () => {
    const result = neh.solve([]);
    expect(result).toEqual([]);
  });

  it('should handle single job', () => {
    const jobs = [{ id: 1, processingTimes: [3, 4, 2] }];
    const result = neh.solve(jobs);
    expect(result).toEqual(jobs);
  });

  it('should correctly sort jobs by total processing time', () => {
    const jobs = [
      { id: 1, processingTimes: [3, 4, 2] }, // total: 9
      { id: 2, processingTimes: [2, 3, 4] }, // total: 9
      { id: 3, processingTimes: [4, 5, 6] }, // total: 15
    ];
    const result = neh.solve(jobs);

    expect(result[0].id).toBe(3); // Should be first as it has highest total time
  });

  it('should find optimal sequence for two jobs', () => {
    const jobs = [
      { id: 1, processingTimes: [3, 4, 2] }, // 9
      { id: 2, processingTimes: [2, 3, 4] }, // 9
    ];
    const result = neh.solve(jobs);
    const makespan = new DefaultMakespanCalculator().calculate(result);
    expect(makespan).toBeLessThanOrEqual(9); // Expected makespan for optimal sequence
  });

  it('should handle jobs with different number of machines', () => {
    const jobs = [
      { id: 1, processingTimes: [3, 4] },
      { id: 2, processingTimes: [2, 3, 4] },
    ];
    expect(() => neh.solve(jobs)).toThrow();
  });

  it('should find optimal sequence for multiple jobs', () => {
    const jobs = [
      { id: 1, processingTimes: [3, 4, 2] },
      { id: 2, processingTimes: [2, 3, 4] },
      { id: 3, processingTimes: [4, 5, 6] },
      { id: 4, processingTimes: [1, 2, 3] },
    ];
    const result = neh.solve(jobs);
    const makespan = new DefaultMakespanCalculator().calculate(result);
    expect(makespan).toBeLessThanOrEqual(20); // Expected makespan for optimal sequence
  });

  it('should maintain job properties after sorting', () => {
    const jobs = [
      { id: 1, processingTimes: [3, 4, 2] },
      { id: 2, processingTimes: [2, 3, 4] },
      { id: 3, processingTimes: [4, 5, 6] },
    ];
    const result = neh.solve(jobs);
    result.forEach((job) => {
      expect(job).toHaveProperty('id');
      expect(job).toHaveProperty('processingTimes');
      expect(job.processingTimes).toHaveLength(3);
    });
  });

  it('should handle jobs with zero processing times', () => {
    const jobs = [
      { id: 1, processingTimes: [0, 0, 0] },
      { id: 2, processingTimes: [0, 0, 0] },
    ];
    const result = neh.solve(jobs);
    const makespan = new DefaultMakespanCalculator().calculate(result);
    expect(makespan).toBe(0);
  });

  it('should handle jobs with large processing times', () => {
    const jobs = [
      { id: 1, processingTimes: [1000, 2000, 3000] },
      { id: 2, processingTimes: [2000, 3000, 4000] },
    ];
    const result = neh.solve(jobs);
    const makespan = new DefaultMakespanCalculator().calculate(result);
    expect(makespan).toBeGreaterThan(0);
  });
});
