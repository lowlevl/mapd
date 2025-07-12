use leptos::prelude::*;

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
        <span>404: this page does not exist.</span>
      </div>
    }
}
