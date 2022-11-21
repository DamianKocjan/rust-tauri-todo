<script lang="ts">
  import { onMount } from "svelte";
  import type {
    PaginatedTodos,
    TodoChangeSet,
    Todo as ITodo,
    ErrorResponse,
  } from "../lib/types";
  import Todo from "./Todo.svelte";

  let todos: ITodo[] = [];
  let lastResponse: PaginatedTodos;
  let page = 0;
  let page_size = 4;
  let hasMore = false;

  let error = null;

  async function fetchTodos() {
    try {
      const res = await fetch(
        `http://localhost:8000/api/v1/todos?page=${page}&page_size=${page_size}`
      );

      if (res.status !== 200) {
        error = ((await res.json()) as ErrorResponse).message;
        return;
      }

      const data = (await res.json()) as PaginatedTodos;

      lastResponse = data;
      todos = [...todos, ...data.result];
    } catch (err) {
      console.error(err);
      error = err.message;
    }
  }

  async function updateTodos() {
    const till = page;
    page = 0;
    todos = [];
    lastResponse = null;

    for (let i = 0; i < till; i++) {
      await fetchTodos();

      if (lastResponse.total > todos.length) {
        page++;
        continue;
      }
      break;
    }
  }

  async function fetchNext() {
    page++;
    await fetchTodos();
  }

  let todo = {
    title: "",
    description: "",
  };

  async function createTodo(e: Event) {
    e.preventDefault();

    try {
      const res = await fetch(`http://localhost:8000/api/v1/todos`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          title: todo.title.trim(),
          description: todo.description.trim(),
        }),
      });

      if (res.status !== 201) {
        error = ((await res.json()) as ErrorResponse).message;
        return;
      }

      todo.title = "";
      todo.description = "";

      await updateTodos();
    } catch (err) {
      console.error(err);
      error = err.message;
    }
  }

  async function update(id: number, data: TodoChangeSet) {
    try {
      const res = await fetch(`http://localhost:8000/api/v1/todos/${id}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (res.status !== 200) {
        error = ((await res.json()) as ErrorResponse).message;
        return;
      }

      await updateTodos();
    } catch (err) {
      console.error(err);
      error = err.message;
    }
  }

  async function remove(id: number) {
    try {
      const res = await fetch(`http://localhost:8000/api/v1/todos/${id}`, {
        method: "DELETE",
      });

      if (res.status !== 200) {
        error = ((await res.json()) as ErrorResponse).message;
        return;
      }

      await updateTodos();
    } catch (err) {
      console.error(err);
      error = err.message;
    }
  }

  $: {
    hasMore = todos.length < lastResponse?.total;
  }

  onMount(fetchTodos);
</script>

<h1>Todo App</h1>

{#if error}
  <div class="error">
    <p>{error}</p>
  </div>
{/if}

<form class="todo__create">
  <div class="row">
    <input type="text" placeholder="Title" bind:value={todo.title} />
    <input
      type="text"
      placeholder="Description"
      bind:value={todo.description}
    />
  </div>
  <button type="submit" on:click={createTodo}>Create</button>
</form>

<div class="todos">
  {#each todos as todo}
    <Todo {todo} {update} {remove} />
  {/each}
</div>

{#if hasMore}
  <button class="btn__fetch-next" type="button" on:click={fetchNext}
    >Fetch more</button
  >
{/if}
