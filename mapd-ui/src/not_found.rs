use leptos::prelude::*;
use leptos_router::components::A;

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

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
      <div class="not-found">
        <pre class="pin">{PIN}</pre>
        <div>this page does not exist,</div>
        <A href="/">take me back "\u{21a9}"</A>
      </div>
    }
}
