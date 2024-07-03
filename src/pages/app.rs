use leptos::*;
use leptos::ev::MouseEvent;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use serde::{Deserialize, Serialize};

use crate::components::todo_item::{TodoItem, TodoItemProp};

pub enum Status {
    Todo,
    Pending,
    Completed,
}
impl Status {
    pub(crate) fn value(&self) -> String {
        match self {
            Status::Todo => String::from("Todo"),
            Status::Pending => String::from("Pending"),
            Status::Completed => String::from("Completed"),
        }
    }
}


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItemStruct {
    pub(crate) id: usize,
    pub(crate) text: String,
    pub(crate) created_at: String,
    pub(crate) status: String,
}

impl Default for TodoItemStruct {
    fn default() -> Self {
        Self {
            id: 0,
            text: String::new(),
            created_at: String::new(),
            status: Status::Todo.value(),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (todo_list, set_todo_list, _) = use_local_storage::<Vec<TodoItemStruct>, JsonCodec>("TODO");
    let (new_todo, set_new_todo) = create_signal(String::new());
    let (order, set_order) = create_signal(0_usize);
    let (time, set_time) = create_signal(String::new());

    let on_create_todo = move |_: MouseEvent| {
        if new_todo.get().len() > 0 && time.get().len() > 0 {
            set_todo_list.update(|list: &mut Vec<TodoItemStruct>| {
                (*list).push(TodoItemStruct {
                    id: order.get(),
                    text: new_todo.get(),
                    created_at: time.get(),
                    status: String::from("Todo"),
                });
            });
            set_order.update(|n| {
                *n += 1;
            });
            set_new_todo.set(String::new());
            set_time.set(String::new());
        };
    };


    let edit_callback = Box::from(move |new_value: TodoItemStruct| {
        set_todo_list.update(|list: &mut Vec<TodoItemStruct>| {
            if let Some(idx) = (*list).iter().position(|o| o.id == new_value.id) {
                (*list)[idx] = new_value.clone();
            }
        });
    });

    let remove_todo = Box::from(move |id: usize| {
        set_todo_list.set(
            todo_list.get().iter().filter(|todo| todo.id != id).cloned().collect()
        );
    });

    view! {
        <div class="todo-container">
            <h1>TODO List</h1>

            <div class="input-container">
                <input type="text" placeholder="What would you like to do?" on:change=move |ev| set_new_todo.set(event_target_value(&ev)) prop:value={move || new_todo.get()} />
                <input type="datetime-local" prop:value=move||time.get() on:change=move |ev|set_time.set(event_target_value(&ev)) />
                <button on:click={on_create_todo}>Add</button>
            </div>

            <div class="todo-list">
                    <h2>"Todo List"</h2>
                <div class="todo-in-container">
                        <For
                            each=move || todo_list.get()
                            key=|todo| todo.id.clone()
                            children=move |todo| {
                              view! {
                                    <TodoItem prop={TodoItemProp {
                                            value:todo.clone(),
                                            on_complete:edit_callback.clone(),
                                            on_text:edit_callback.clone(),
                                            on_remove:remove_todo.clone(),
                                            on_time:edit_callback.clone(),
                                    }} />
                              }
                            }
                          />
                </div>
            </div>
        </div>
    }
}