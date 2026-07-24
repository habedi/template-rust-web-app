const apiUrl = import.meta.env.VITE_API_URL.replace(/\/\s*$/, '');

export class ApiError extends Error {
	constructor(readonly status: number) {
		super(`request failed with status ${status}`);
		this.name = 'ApiError';
	}
}

/**
 * Wraps `fetch` with the API base URL and error handling. Inside a SvelteKit
 * `load` function, pass that function's own `fetch` so the request is tracked.
 */
export const apiFetch = async <T = void>(
	url: string,
	options?: RequestInit,
	fetcher: typeof fetch = fetch,
): Promise<T> => {
	const response = await fetcher(`${apiUrl}${url}`, { mode: 'cors', ...options });

	if (!response.ok) throw new ApiError(response.status);

	if (response.headers.get('content-type')?.includes('application/json')) {
		return (await response.json()) as T;
	}

	return undefined as T;
};

export const withJson = (options: RequestInit | undefined, data?: unknown): RequestInit => ({
	...options,
	headers: { ...options?.headers, 'content-type': 'application/json' },
	body: JSON.stringify(data),
});

export * from './items';
