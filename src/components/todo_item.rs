use leptos::*;
use leptos::ev::{Event, MouseEvent};

use crate::pages::app::TodoItemStruct;

pub struct TodoItemProp {
    pub(crate) value: TodoItemStruct,
    pub(crate) on_text: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_complete: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_time: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_remove: Box<dyn Fn(usize)>,
}

#[component]
pub fn TodoItem(prop: TodoItemProp) -> impl IntoView {
    let (edit, set_edit) = create_signal(false);
    let (todo, set_todo) = create_signal(prop.value.text.clone());
    let (completed, set_completed) = create_signal(prop.value.completed.clone());
    let (time, set_time) = create_signal(prop.value.created_at.clone());

    let on_change_text = move |event: Event| {
        set_todo.set(event_target_value(&event));
        if time.get().len() > 0 && todo.get().len() > 0 {
            (prop.on_text)(TodoItemStruct {
                id: prop.value.id.clone(),
                text: todo.get(),
                completed: completed.get(),
                created_at: time.get(),
            });
            set_edit.set(false);
        }
    };

    let on_click_complete = move |_: MouseEvent| {
        set_completed.update(|cmp| (*cmp) = !*cmp);
        (prop.on_complete)(TodoItemStruct {
            id: prop.value.id.clone(),
            text: todo.get(),
            completed: completed.get(),
            created_at: time.get(),
        });
    };

    let on_click_remove = move |_: MouseEvent| {
        (prop.on_remove)(prop.value.id.clone());
    };

    let on_change_time = move |event: Event| {
        set_time.set(event_target_value(&event));
        if time.get().len() > 0 && todo.get().len() > 0 {
            (prop.on_time)(TodoItemStruct {
                id: prop.value.id.clone(),
                text: todo.get(),
                completed: completed.get(),
                created_at: time.get(),
            });
            set_edit.set(false);
        }
    };

    view! {
        <div class="todo-item">
            <div style="flex-grow:1;display:flex;gap:1rem;">
                <textarea
                    style="flex-grow:1;"
                    readonly=move || !edit.get()
                    disabled=move || completed.get()
                    on:dblclick=move |_| if !completed.get() { set_edit.set(true)}
                    type="text"
                    prop:value=move || todo.get()
                    on:change={on_change_text}/>
                <input
                    type="datetime-local"
                    prop:value=move || time.get()
                    on:change={on_change_time}
                    readonly=move || !edit.get()
                    disabled=move || completed.get()
                    on:dblclick=move |_| if !completed.get() { set_edit.set(true)}
                    />
            </div>
            <div class="todo-item-buttons">
                <button class:unchecked=move ||!completed.get() class:checked=move ||completed.get() data-primary on:click={on_click_complete}></button>
                <button data-danger class="remove-icon" on:click={on_click_remove}></button>
            </div>
        </div>
    }
}