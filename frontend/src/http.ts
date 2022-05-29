export const globals: Partial<RequestInit> = { headers: {} };

async function request(url: string, data: RequestInit): Promise<Response> {
	const response = await fetch(url, { ...globals, ... data, headers: {...globals.headers, ...data.headers} });
	if(!response.ok) throw new Error("Server returned error response");
	return response;
}

export async function get<T>(url: string): Promise<T> {
	const response = await request(url, {});
	return await response.json() as T;
}

export async function post<T>(url: string, body: unknown): Promise<T> {
	const response = await request(url, { method: "POST", body: JSON.stringify(body), headers: { "Content-Type": "application/json" } });
	return await response.json() as T;
}

export async function put(url: string, body: unknown): Promise<void> {
	await request(url, { method: "PUT", body: JSON.stringify(body), headers: { "Content-Type": "application/json" } });
}

