type Job = number[];
type Order = number[];

// function makespan(perm: Order, times: Job[]): number {
//   const jobCount = perm.length;
//   const machineCount = times[0]?.length;

//   console.log(jobCount, machineCount);

//   const makespanMatrix: number[][] = Array.from({ length: jobCount + 1 }, () =>
//     new Array(machineCount + 1).fill(0),
//   );

//   // Loop through each job in the permutation order
//   for (let i = 0; i < jobCount; i++) {
//     const jobId = perm[i]; // Get the job ID from the permutation

//     if (jobId < 1 || jobId > times.length) {
//       console.log(1, jobId, times.length);
//     }

//     // Loop through each machine
//     for (let machine = 0; machine < machineCount; machine++) {
//       if (!times[jobId - 1])
//         // Calculate the makespan at this position (job, machine)
//         makespanMatrix[i + 1][machine + 1] =
//           Math.max(
//             makespanMatrix[i][machine + 1],
//             makespanMatrix[i + 1][machine],
//           ) + times[jobId - 1][machine];
//     }
//   }

//   // The final makespan is in the bottom-right corner of the matrix
//   return makespanMatrix[jobCount][machineCount];
// }

function makespan(perm: Order, times: Job[]): number {
  const jobCount = perm.length;
  const machineCount = times[0]?.length;

  const makespanMatrix: number[][] = Array.from({ length: jobCount + 1 }, () =>
    new Array(machineCount + 1).fill(0),
  );

  // Loop through each job in the permutation order
  for (let i = 0; i < jobCount; i++) {
    const jobId = perm[i]; // Get the job ID from the permutation

    // Loop through each machine
    for (let machine = 0; machine < machineCount; machine++) {
      if (!times[jobId - 1]) {
        continue;
      }

      // Calculate the makespan at this position (job, machine)
      makespanMatrix[i + 1][machine + 1] =
        Math.max(
          makespanMatrix[i][machine + 1],
          makespanMatrix[i + 1][machine],
        ) + times[jobId - 1][machine];
    }
  }

  // The final makespan is in the bottom-right corner of the matrix
  return makespanMatrix[jobCount][machineCount];
}

export function neh(times: Job[]): Order {
  const jobsWithTotalTimes: [number, number][] = times.map((job, jobId) => [
    jobId,
    job.reduce((a, b) => a + b, 0),
  ]);
  let order: Order = [];

  jobsWithTotalTimes.sort((a, b) => b[1] - a[1]);

  for (const job of jobsWithTotalTimes) {
    const jobId = job[0];
    let bestMakespan = Infinity;
    let bestPosition = 0;

    for (let i = 0; i <= order.length; i++) {
      order.splice(i, 0, jobId);
      const currentMakespan = makespan(order, times);

      if (currentMakespan < bestMakespan) {
        bestMakespan = currentMakespan;
        bestPosition = i;
      }

      order.splice(i, 1);
    }

    order.splice(bestPosition, 0, jobId);
  }

  return order;
}

export function calculateCmax(
  optimalOrder: number[],
  times: number[][],
): number {
  const jobCount = optimalOrder.length;
  const machineCount = times[0].length;

  // Initialize an array to store the completion time for each machine
  let completionTimes: number[] = new Array(machineCount).fill(0);

  // Simulate the flow of jobs through the machines
  for (let i = 0; i < jobCount; i++) {
    const jobId = optimalOrder[i] - 1; // Convert to 0-based index

    for (let machine = 0; machine < machineCount; machine++) {
      if (!times[jobId - 1]) {
        continue;
      }
      if (machine === 0) {
        // For the first machine, start time is simply the previous job's finish time
        completionTimes[machine] =
          Math.max(
            completionTimes[machine],
            machine > 0 ? completionTimes[machine - 1] : 0,
          ) + times[jobId][machine];
      } else {
        // For subsequent machines, the start time is the maximum of either
        // the finish time of the previous machine or the completion time on the current machine
        completionTimes[machine] =
          Math.max(completionTimes[machine], completionTimes[machine - 1]) +
          times[jobId][machine];
      }
    }
  }

  // The final Cmax is the completion time of the last job on the last machine
  return completionTimes[machineCount - 1];
}
