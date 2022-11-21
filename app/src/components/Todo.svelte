<script lang="ts">
  import type { Todo, TodoChangeSet } from "../lib/types";

  export let todo: Todo;
  export let remove: (id: number) => Promise<void>;
  export let update: (id: number, data: TodoChangeSet) => Promise<void>;

  let isEditing = false;

  function toggleEditing() {
    isEditing = !isEditing;
  }
</script>

{#if isEditing}
  <div class="todo todo__editing-form">
    <div class="todo__title">
      <input
        class="todo__edit-input"
        type="text"
        placeholder="Title"
        bind:value={todo.title}
        on:keydown={async (e) => {
          if (e.key === "Enter") {
            await update(todo.id, { title: todo.title.trim() });
            toggleEditing();
          }
        }}
      />
    </div>
    <div class="todo__description">
      <input
        class="todo__edit-input"
        type="text"
        placeholder="Description"
        bind:value={todo.description}
        on:keydown={async (e) => {
          if (e.key === "Enter") {
            await update(todo.id, { description: todo.description.trim() });
            toggleEditing();
          }
        }}
      />
    </div>

    <div class="todo__actions">
      <button
        type="button"
        class="todo__action todo__action--isCompleted"
        on:click={async () =>
          await update(todo.id, { is_completed: !todo.is_completed })}
      >
        {#if todo.is_completed}
          Mark as incomplete
        {:else}
          Mark as complete
        {/if}
      </button>

      <button
        type="button"
        class="todo__action todo__action--cancel"
        on:click={toggleEditing}
      >
        Cancel
      </button>
      <button
        type="button"
        class="todo__action todo__action--save"
        on:click={async () => {
          await update(todo.id, {
            title: todo.title,
            description: todo.description,
          });
          toggleEditing();
        }}
      >
        Save
      </button>
      <button
        type="button"
        class="todo__action todo__action--delete"
        on:click={async () => await remove(todo.id)}
      >
        Delete
      </button>
    </div>
  </div>
{:else}
  <div class="todo {todo.is_completed ? 'completed' : ''}">
    <div class="todo__title">
      {todo.title}
    </div>
    <div class="todo__description">
      {todo.description}
    </div>

    <div class="todo__actions">
      <button
        type="button"
        class="todo__action todo__action--isCompleted"
        on:click={async () =>
          await update(todo.id, { is_completed: !todo.is_completed })}
      >
        {#if todo.is_completed}
          Mark as incomplete
        {:else}
          Mark as complete
        {/if}
      </button>
      <button
        type="button"
        class="todo__action todo__action--edit"
        on:click={toggleEditing}
      >
        Edit
      </button>
      <button
        type="button"
        class="todo__action todo__action--delete"
        on:click={async () => await remove(todo.id)}
      >
        Delete
      </button>
    </div>
  </div>
{/if}
