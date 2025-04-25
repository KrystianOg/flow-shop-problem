import fs from 'fs';
import path from 'path';
import type { Job } from '../neh.js';

export function loadData(filename: string): number[][] {
  const filePath = path.resolve(process.cwd(), 'converted_files', filename);

  const fileData = fs.readFileSync(filePath, 'utf-8');

  const data = fileData
    .split('\n')
    .map((m) => m.split(',').map((s) => parseFloat(s)));

  // remove last element if last \n was included in split
  if (data.at(-1)?.length === 1) {
    data.pop();
  }

  return data;
}

export function toJobs(matrix: number[][]): Job[] {
  const jobs = matrix.map<Job>((processingTimes, id) => ({
    id,
    processingTimes,
  }));

  return jobs;
}
