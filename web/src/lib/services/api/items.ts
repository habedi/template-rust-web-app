import { apiFetch, withJson } from '.';

export type Item = {
	id: string;
	name: string;
	description: string | null;
	created_at: string;
	updated_at: string;
};

export type ItemDraft = { name: string; description?: string | null };

export const listItems = async (fetcher?: typeof fetch) =>
	await apiFetch<Item[]>('/items', { method: 'GET' }, fetcher);

export const getItem = async (id: Item['id'], fetcher?: typeof fetch) =>
	await apiFetch<Item>(`/items/${id}`, { method: 'GET' }, fetcher);

export const createItem = async (item: ItemDraft) =>
	await apiFetch<Item>('/items', withJson({ method: 'POST' }, item));

export const updateItem = async (id: Item['id'], item: ItemDraft) =>
	await apiFetch<Item>(`/items/${id}`, withJson({ method: 'PUT' }, item));

export const deleteItem = async (id: Item['id']) =>
	await apiFetch(`/items/${id}`, { method: 'DELETE' });
