import { networkInterfaces } from 'systeminformation';
import { bench, describe } from 'vitest';
import { getNetworkInterfaces } from '../index.js';

describe('get networks', () => {
  bench('rust sysinfo', async () => {
    await getNetworkInterfaces();
  });

  bench('systeminformation', async () => {
    await networkInterfaces();
  });
});
