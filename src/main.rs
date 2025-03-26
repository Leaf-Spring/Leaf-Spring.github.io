use yew::prelude::*;
use yew_router::prelude::*;

/// Definición de rutas
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/proyectos")]
    Proyectos,
}

/// Página Home
#[function_component(Home)]
fn home() -> Html {
    html! {
        <section style="text-align: center; padding: 20px;">
            <h2>{"Home"}</h2>
            <p>{"Bienvenido a la página principal."}</p>
        </section>
    }
}

/// Página About
#[function_component(About)]
fn about() -> Html {
    html! {
        <section style="text-align: center; padding: 20px;">
            <h2>{"About"}</h2>
            <p>{"Esta es la sección 'About' de la página."}</p>
        </section>
    }
}

/// Página Proyectos
#[function_component(Proyectos)]
fn proyectos() -> Html {
    html! {
        <section style="text-align: center; padding: 20px;">
            <h2>{"Proyectos"}</h2>
            <p>{"Aquí se mostrarán los proyectos."}</p>
        </section>
    }
}

/// Función que asigna una ruta a un componente
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Proyectos => html! { <Proyectos /> },
    }
}

/// Componente principal que maneja la navegación y las rutas
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav style="display: flex; justify-content: center; gap: 20px; padding: 20px;">
                <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                <Link<Route> to={Route::About} classes="nav-link">{"About"}</Link<Route>>
                <Link<Route> to={Route::Proyectos} classes="nav-link">{"Proyectos"}</Link<Route>>
            </nav>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}

