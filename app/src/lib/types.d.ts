export interface Todo {
  id: number;
  title: string;
  description: string;
  is_completed: boolean;
}

export type TodoChangeSet = Partial<Omit<Todo, "id">>;

export interface PaginatedTodos {
  page: number;
  page_size: number;
  result: Todo[];
  success: number;
  total: number;
}

export interface ErrorResponse {
  error: number;
  message: string;
}
