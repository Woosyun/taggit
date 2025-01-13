//use wasm_bindgen::prelude::*;
//use leptos::prelude::*;

//#[wasm_bindgen]
//pub fn run(input: &str) -> Result<(), JsValue> {
    //let window = web_sys::window().expect("cannot get window");
    //let document = window.document().expect("cannot get document");
    //let body = document.body().expect("cannot get body");

    //let p = document.create_element("p")?;
    //p.set_text_content(Some(input));
    //body.append_child(&p)?;

    //Ok(())
//}

// purpose of this trait is 
// implementing data-structure-independent 
// note-handling abilities
pub trait NoteAction {
    type Body;
    type Ptr;
    
    // frontend -> Body : scan frontend and reflect changes to Body
    fn synchronize(&mut self);

    fn get_first_child(&self) -> Option<Self::Ptr>;
    fn next_child(&self, current_ptr: Self::Ptr) -> Option<Self::Ptr>;
    fn get_focused_element(&self) -> Option<Self::Ptr>; 

    //fn add_element(&mut self) -> Self::Ptr {implementation required}
    //fn get_body(&self) -> Self::Body {implementation required}
}