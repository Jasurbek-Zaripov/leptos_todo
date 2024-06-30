use leptos::*;
use leptos::ev::{Event, KeyboardEvent};
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use serde::{Deserialize, Serialize};

use crate::components::todo_item::{TodoItem, TodoItemProp};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItemStruct {
    pub(crate) id: usize,
    pub(crate) text: String,
    pub(crate) completed: bool,
}

impl Default for TodoItemStruct {
    fn default() -> Self {
        Self {
            id: 0,
            text: String::from("empty"),
            completed: false,
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (todo_list, set_todo_list, _) = use_local_storage::<Vec<TodoItemStruct>, JsonCodec>("TODO");
    let (new_todo, set_new_todo) = create_signal(String::new());
    let (order, set_order) = create_signal(0_usize);

    let on_enter = move |event: KeyboardEvent| {
        if event.key() == String::from("Enter") {
            set_todo_list.update(|list: &mut Vec<TodoItemStruct>| {
                (*list).push(TodoItemStruct {
                    id: order.get(),
                    text: new_todo.get(),
                    completed: false,
                });
            });
            set_order.update(|n| {
                *n += 1;
            });
            set_new_todo.set(String::new());
        };
    };

    let on_input = move |event: Event| {
        set_new_todo.set(event_target_value(&event));
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
        <div>
            <div class="input-parent">
                <label>"TODO"</label>
                <input type="text" on:keyup={on_enter}  placeholder="typing" on:input={on_input} prop:value={move || new_todo.get()}/>
            </div>
            <ul class="list">
                <For
                each=move || todo_list.get()
                key=|todo| todo.id.clone()
                children=move |todo| {
                  view! {
                    <li class="list-item">
                        <TodoItem prop={TodoItemProp {
                                value:todo.clone(),
                                on_complete:edit_callback.clone(),
                                on_text:edit_callback.clone(),
                                on_remove:remove_todo.clone()
                        }} />
                    </li>
                  }
                }
              />
            </ul>
        </div>
    }
}