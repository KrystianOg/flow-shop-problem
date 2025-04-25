/**
 * Interface defining the structure of a job in the flow shop problem
 */
export interface Job {
  id: number;
  processingTimes: number[];
}

/**
 * Interface for calculating makespan
 */
interface MakespanCalculator {
  calculate(jobs: Job[]): number;
}

/**
 * Default implementation of makespan calculation
 */
export class DefaultMakespanCalculator implements MakespanCalculator {
  /**
   * Calculates the makespan for a given sequence of jobs
   * @param jobs - Array of jobs in their current sequence
   * @returns The total makespan
   */
  calculate(jobs: Job[]): number {
    if (jobs.length === 0) return 0;

    const numMachines = jobs[0].processingTimes.length;
    const numJobs = jobs.length;

    // Initialize completion time matrix
    const completionTimes: number[][] = Array(numJobs)
      .fill(0)
      .map(() => Array(numMachines).fill(0));

    // Calculate completion times
    for (let i = 0; i < numJobs; i++) {
      for (let j = 0; j < numMachines; j++) {
        const prevJobTime = i > 0 ? completionTimes[i - 1][j] : 0;
        const prevMachineTime = j > 0 ? completionTimes[i][j - 1] : 0;
        completionTimes[i][j] =
          Math.max(prevJobTime, prevMachineTime) + jobs[i].processingTimes[j];
      }
    }

    return completionTimes[numJobs - 1][numMachines - 1];
  }
}

/**
 * NEH Algorithm implementation for permutation flow shop scheduling
 */
export class NEHAlgorithm {
  private makespanCalculator: MakespanCalculator;

  /**
   * Creates a new instance of NEH algorithm
   * @param makespanCalculator - Optional custom makespan calculator
   */
  constructor(makespanCalculator?: MakespanCalculator) {
    this.makespanCalculator =
      makespanCalculator || new DefaultMakespanCalculator();
  }

  /**
   * Calculates the total processing time for each job
   * @param job - The job to calculate total time for
   * @returns Total processing time across all machines
   */
  private calculateTotalProcessingTime(job: Job): number {
    return job.processingTimes.reduce((sum, time) => sum + time, 0);
  }

  /**
   * Sorts jobs by their total processing time in descending order
   * @param jobs - Array of jobs to sort
   * @returns Sorted array of jobs
   */
  private sortJobsByTotalTime(jobs: Job[]): Job[] {
    // Create a new array to avoid modifying the original
    return [...jobs].sort((a, b) => {
      const totalA = this.calculateTotalProcessingTime(a);
      const totalB = this.calculateTotalProcessingTime(b);
      return totalB - totalA; // This ensures descending order
    });
  }

  /**
   * Finds the best position to insert a job in the current sequence
   * @param currentSequence - Current sequence of jobs
   * @param jobToInsert - Job to be inserted
   * @returns Best position index and its makespan
   */
  private findBestInsertionPosition(
    currentSequence: Job[],
    jobToInsert: Job,
  ): { position: number; makespan: number } {
    let bestPosition = 0;
    let bestMakespan = Infinity;

    for (let i = 0; i <= currentSequence.length; i++) {
      const newSequence = [...currentSequence];
      newSequence.splice(i, 0, jobToInsert);
      const makespan = this.makespanCalculator.calculate(newSequence);

      if (makespan < bestMakespan) {
        bestMakespan = makespan;
        bestPosition = i;
      }
    }

    return { position: bestPosition, makespan: bestMakespan };
  }

  /**
   * Solves the permutation flow shop problem using NEH algorithm
   * @param jobs - Array of jobs to schedule
   * @returns Optimized sequence of jobs
   */
  public solve(jobs: Job[]): Job[] {
    if (jobs.length === 0) return [];

    if (!this.isEqualTasksCount(jobs)) {
      throw new Error('Jobs require different counts of machines.');
    }

    // Step 1: Sort jobs by total processing time
    const sortedJobs = this.sortJobsByTotalTime(jobs);

    // Step 2: Initialize sequence with first job
    let sequence: Job[] = [sortedJobs[0]];

    // Step 3: For each remaining job, find its best position
    for (let i = 1; i < sortedJobs.length; i++) {
      const { position } = this.findBestInsertionPosition(
        sequence,
        sortedJobs[i],
      );
      sequence.splice(position, 0, sortedJobs[i]);
    }

    return sequence;
  }

  private isEqualTasksCount(jobs: Job[]): boolean {
    if (jobs.length === 0) return true;

    const expectedLength = jobs[0].processingTimes.length;

    for (const job of jobs) {
      if (job.processingTimes.length !== expectedLength) {
        return false;
      }
    }

    return true;
  }
}
