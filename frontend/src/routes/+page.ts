import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    todos: (await fetch("https://rust-svelte-api.devstroop.com").then((data) => data.json())) as Todo[]
  };
};
