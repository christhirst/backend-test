use crate::error_template::{AppError, ErrorTemplate};
use crate::model::data::{Data, Datas};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (datas, set_datas) = create_signal(Datas::new());

    /* let send = create_action(|new_data: &String| async {
        todo!()

      //  set_datas.update(move |c| c.datas.push(user_data))
    }); */
    view! {
        <Stylesheet id="leptos" href="/pkg/backend-test.css"/>

        // sets the document title
        <Title text="Ux-ti.com"/>
        ///<DataArea send/>
    }
}
