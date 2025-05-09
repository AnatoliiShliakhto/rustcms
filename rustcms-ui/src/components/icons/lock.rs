use super::*;

pub fn LockIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "stroke-width": "0",
            "stroke-linejoin": "miter",
            "fill": "currentColor",
            "viewBox": "0 0 24 24",
            "stroke": "none",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke-linecap": "butt",
            path {
                "d": "M12,17c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,17,12,17z \
                M18,8h-1V6c0-2.76-2.24-5-5-5S7,3.24,7,6v2H6 c-1.1,0-2,0.9-2,2v10c0,\
                1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V10C20,8.9,19.1,8,18,8z M8.9,6c0-1.71,\
                1.39-3.1,3.1-3.1 s3.1,1.39,3.1,3.1v2H8.9V6z M18,20H6V10h12V20z"
            }
        }
    }
}    