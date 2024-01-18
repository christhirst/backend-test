use crate::error_template::{AppError, ErrorTemplate};
use crate::model::data::{Data, Datas};
use leptos::*;
use leptos_meta::*;
use leptos_oidc2::{Auth, AuthParameters};
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
        <Router>
            <AppWithRouter/>
            <MyComponent/>
        </Router>
    }
}

#[component]
pub fn AppWithRouter() -> impl IntoView {
    // Specify OIDC authentication parameters here.
    // Note: This is an example for keycloak, please change it to your needs
    let auth_parameters = AuthParameters {
        auth_endpoint: "https://samples.auth0.com/authorize".to_string(),
        token_endpoint: "https://samples.auth0.com/oauth/token".to_string(),
        logout_endpoint: "https://ENDPOINT/auth/realms/REALM/protocol/openid-connect/logout"
            .to_string(),
        client_id: "kbyuFDidLLm280LIwVFiazOqjO3ty8KH".to_string(),
        redirect_uri: "http://localhost:3000/profile".to_string(),
        post_logout_redirect_uri: "http://localhost:3000/bye".to_string(),
        scope: Some("openid profile email phone address".to_owned()),
    };
    let auth = Auth::init(auth_parameters);

    view! {
        <div></div>

        <Routes>
            <Route path="/" view=move || view! { <Home/> }/>

            // This is an example route for your profile, it will render
            // loading if it's still loading, render unauthenticated if it's
            // unauthenticated and it will render the children, if it's
            // authenticated
            <Route
                path="/profile"
                view=move || {
                    view! {}
                }
            />

        </Routes>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let auth = expect_context::<Auth>();

    view! {
        <Title text="Home"/>
        <h1>Home</h1>
    }
}

/// This will be rendered, if the authentication library is still loading
#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <Title text="Loading"/>
        <h1>Loading</h1>
    }
}

/// This will be rendered, if the user is unauthenticated
#[component]
pub fn Unauthenticated() -> impl IntoView {
    view! {
        <Title text="Unauthenticated"/>
        <h1>Unauthenticated</h1>
    }
}

/// This will be rendered, if the user is authentication
#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <Title text="Profile"/>
        <h1>Profile</h1>
    }
}

#[component]
fn MyComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let (count, set_count) = create_signal(String::new());
    let auth = expect_context::<Auth>();

    // Generate the login URL to initiate the authentication process.
    let login_url = auth.clone().login_url().clone();
    //let login_url = move || login_url;
    //println!("{:?}", login_url);

    // Generate the logout URL for logging out the user.
    let logout_url = auth.clone().logout_url().clone();
    let logout_url = move || logout_url;
    set_name(login_url);
    // set_count.update(string::new("test"));
    view! {
        <button>count()</button>
        <A href=name>Login</A>
    }
}
