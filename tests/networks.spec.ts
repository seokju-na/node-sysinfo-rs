import { describe, expect, test } from 'vitest';
import { getNetworkInterfaces } from '../index.js';

describe('getNetworkInterfaces', () => {
  test('get networks', async () => {
    const networks = await getNetworkInterfaces();
    expect(networks).not.toBeNullable();
  });
});
