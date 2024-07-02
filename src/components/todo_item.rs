use leptos::*;
use leptos::ev::{Event, MouseEvent};

use crate::pages::app::Status;
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
    let (status, set_status) = create_signal(prop.value.status.clone());
    let (time, set_time) = create_signal(prop.value.created_at.clone());

    let on_change_text = move |event: Event| {
        set_todo.set(event_target_value(&event));
        if time.get().len() > 0 && todo.get().len() > 0 {
            (prop.on_text)(TodoItemStruct {
                id: prop.value.id.clone(),
                text: todo.get(),
                status: status.get(),
                created_at: time.get(),
            });
            set_edit.set(false);
        }
    };

    let on_click_status = move |event: Event| {
        set_status.set(event_target_value(&event));
        (prop.on_complete)(TodoItemStruct {
            id: prop.value.id.clone(),
            text: todo.get(),
            status: status.get(),
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
                status: status.get(),
                created_at: time.get(),
            });
            set_edit.set(false);
        }
    };

    let is_completed = move || status.get() == Status::Completed.value();

    view! {
        <tr>
            <td>
                <input
                    type="text"
                    readonly=move || !edit.get()
                    disabled=move || is_completed()
                    on:dblclick=move |_| if !is_completed() { set_edit.set(true)}
                    prop:value=move || todo.get()
                    on:change={on_change_text}
                />
            </td>
            <td>
                <input
                    type="datetime-local"
                    prop:value=move || time.get()
                    on:change={on_change_time}
                    readonly=move || !edit.get()
                    disabled=move || is_completed()
                    on:dblclick=move |_| if !is_completed() { set_edit.set(true)}
                />
            </td>
            <td>
                <select prop:value=move || status.get() on:change={on_click_status}>
                    <option value={Status::Todo.value()}>{Status::Todo.value()}</option>
                    <option value={Status::Pending.value()}>{Status::Pending.value()}</option>
                    <option value={Status::Completed.value()}>{Status::Completed.value()}</option>
                </select>
        </td>
            <td><button class="delete-btn" on:click={on_click_remove}>'\u{fe0f}'</button></td>
        </tr>
    }
}