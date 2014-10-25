extern crate libc;

use libc::{c_char, c_int, c_void};
use std::ptr;

static HTML_PARSE_NOERROR: c_int = 32;
static HTML_PARSE_NOWARNING: c_int = 64;

#[link(name = "xml2")]
#[allow(non_camel_case_types)]
extern {
    fn htmlReadMemory(buffer: *const c_char, size: c_int, url: *const c_char, encoding: *const c_char, options: c_int) -> *mut c_void;
    fn xmlFreeDoc(cur: *mut c_void);
}

struct XmlDoc {
    doc_ptr: *mut c_void,
}

impl Drop for XmlDoc {
    fn drop(&mut self) {
        unsafe { xmlFreeDoc(self.doc_ptr); }
    }
}

impl XmlDoc {
    fn from_html_str(html: &str) -> Option<XmlDoc> {
        unsafe {
            let c_html = html.to_c_str();
            let c_enc = "".to_c_str();
            let doc_ptr = htmlReadMemory(c_html.as_ptr() as *const c_char, html.len() as c_int, c_enc.as_ptr(), ptr::null(), HTML_PARSE_NOWARNING | HTML_PARSE_NOERROR);
            if doc_ptr.is_null() {
                None
            } else {
                Some(XmlDoc { doc_ptr: doc_ptr })
            }
        }
    }
}

fn main() {
    let doc = XmlDoc::from_html_str("<html></html>");
    assert!(doc.is_some());
}
