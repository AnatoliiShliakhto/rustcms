use super::*;

pub fn ManIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "stroke-linejoin": "miter",
            "stroke": "none",
            "stroke-width": "0",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 0 24 24",
            "fill": "currentColor",
            "stroke-linecap": "butt",
            path { 
                "d": "M12 2c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm9 \
                7h-6v13h-2v-6h-2v6H9V9H3V7h18v2z" 
            }
        }
    }
}