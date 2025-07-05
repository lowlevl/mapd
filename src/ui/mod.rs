use leptos::prelude::*;
use leptos_meta::{MetaTags, Title};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags />

                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
            <body>
                <Ui />
            </body>
        </html>
    }
}

#[component]
pub fn Ui() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Title text="mapd ::" />

        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos"</h2>
            <p class="px-10 pb-10 text-left">"This setup includes Tailwind and SASS"</p>
        </main>
    }
}
