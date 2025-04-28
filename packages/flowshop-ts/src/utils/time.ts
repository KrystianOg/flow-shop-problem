/**returns time in ms*/
export function time(cb: () => void): number {
  const start = new Date().valueOf();
  cb();
  const end = new Date().valueOf();

  return end - start;
}
