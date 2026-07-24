import { describe, expect, it } from 'vitest';
import { truncate } from '../src/lib/services/utils';

describe('truncate', () => {
	it('leaves a short string unchanged', () => {
		expect(truncate('short', 10)).toBe('short');
	});

	it('shortens a long string and marks the cut', () => {
		expect(truncate('abcdefghij', 3)).toBe('abc...');
	});
});
