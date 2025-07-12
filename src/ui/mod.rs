use leptos::prelude::*;
use leptos_meta::{Html, Meta, Style, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod gjson;

mod map;
use map::Map;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::MetaTags;

    view! {
      <!DOCTYPE html>
      <html>
        <head>
          <MetaTags />

          <HydrationScripts options=options.clone() />
          <AutoReload options />
        </head>
        <body>
          <Ui />
        </body>
      </html>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    const PIN: &str = r#"
#         #####          
  #   ###       ###      
    ##             ##    
   ####   #####      #   
  #######       #     #  
  ##### ###      #    #  
  ##### ####     #    #  
  ###### #####  #     #  
   ####### #####     #   
    ##############  #    
     ###############     
      ############# #    
        #########     #  
         #######        #
           ###           
            #            
"#;

    view! {
      <div class="not-found">
        <pre class="pin">{PIN}</pre>
        <span>404: this page does not exist.</span>
      </div>
    }
}

#[component]
pub fn Ui() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Html {..} lang="en" />
        <Title text="mapd :: an untitled map" />

        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />

        <Style>
        r#"
          html, body {
            margin: 0;

            color: #a6a6a6;
            background: #262626;

            font-family: 'Trebuchet MS', sans-serif;
          }

          .not-found {
            position: absolute;

            top: 50%;
            transform: translateY(-50%);

            width: 100vw;
            text-align: center;
          }

          .pin {
            line-height: 1.15em;
          }
        "#
        </Style>

        <Router>
          <main>
            <Routes fallback=NotFound>
              <Route path=path!("/") view=Map />
            </Routes>
          </main>
        </Router>
    }
}
