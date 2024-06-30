use leptos::*;
use leptos::ev::MouseEvent;

use crate::pages::app::TodoItemStruct;

pub struct TodoItemProp {
    pub(crate) value: TodoItemStruct,
    pub(crate) on_text: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_complete: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_remove: Box<dyn Fn(usize)>,
}

#[component]
pub fn TodoItem(prop: TodoItemProp) -> impl IntoView {
    let (edit, set_edit) = create_signal(false);
    let (todo, set_todo) = create_signal(prop.value.text.clone());
    let (completed, set_completed) = create_signal(prop.value.completed.clone());


    let on_blur = move || {
        (prop.on_text)(TodoItemStruct {
            id: prop.value.id.clone(),
            text: todo.get(),
            completed: completed.get(),
        });
        set_edit.set(false);
    };


    let show_input = move || if edit.get() { "inline" } else { "none" };
    let show_text = move || if edit.get() { "none" } else { "block" };

    let on_click_complete = move |_: MouseEvent| {
        set_completed.update(|cmp| (*cmp) = !*cmp);
        (prop.on_complete)(TodoItemStruct {
            id: prop.value.id.clone(),
            text: todo.get(),
            completed: completed.get(),
        });
    };

    let on_click_remove = move |_: MouseEvent| {
        (prop.on_remove)(prop.value.id.clone());
    };

    view! {
        <div class="todo-item">
            <p  class:del=move || completed.get() style:cursor="pointer" style:display=show_text on:dblclick=move |_| if !completed.get() { set_edit.set(true)}>{move ||todo.get()}</p>
            <input
                style:display=show_input
                type="text"
                prop:value=move || todo.get()
                on:input=move |event| set_todo.set(event_target_value(&event))
                on:blur=move |_| {
                    set_edit.set(false);
                    on_blur();
                }/>

            <div class="todo-item-buttons">
                <button class:unchecked=move ||!completed.get() class:checked=move ||completed.get() data-primary on:click={on_click_complete}></button>
                <button data-danger class="remove-icon" on:click={on_click_remove}></button>
            </div>
        </div>
    }
}