import type { PageLoad } from "./$types";
const apiEndpoint = import.meta.env.VITE_API_ENDPOINT || "https://rust-svelte-api.devstroop.com"
export const load: PageLoad = async () => {
  return {
    todos: (await fetch(`${apiEndpoint}`).then((data) => data.json())) as Todo[]
  };
};
