/* eslint-disable @typescript-eslint/no-empty-object-type, @typescript-eslint/no-explicit-any */
import 'vitest';
import type { TestingLibraryMatchers } from '@testing-library/jest-dom/matchers';

declare module 'vitest' {
	interface Assertion<R = void, T = any> extends TestingLibraryMatchers<T, R> {}
	interface AsymmetricMatchersContaining extends TestingLibraryMatchers<any, any> {}
}
