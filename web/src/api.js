const BASE = "/api/v1";
async function request(path, options = {}) {
  const response = await fetch(BASE + path, { ...options, headers: { "Content-Type": "application/json", ...(options.headers || {}) } });
  const isJson = (response.headers.get("content-type") || "").includes("application/json");
  const payload = isJson ? await response.json() : await response.text();
  if (!response.ok) { const error = new Error(payload?.error || `HTTP ${response.status}`); error.status = response.status; error.payload = payload; throw error; }
  return payload;
}
export const api = {
  health: () => request("/health"),
  getState: () => request("/state"),
  createResource: resource => request("/resources", { method: "POST", body: JSON.stringify(resource) }),
  updateResource: (id, resource) => request("/resources/" + encodeURIComponent(id), { method: "PUT", body: JSON.stringify(resource) }),
  importToml: content => request("/resources/import-toml", { method: "POST", body: JSON.stringify({ content }) }),
  saveWorkspace: (workspace, expectedRevision) => request("/workspace", { method: "PUT", body: JSON.stringify({ workspace, expectedRevision }) }),
};