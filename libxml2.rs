#![feature(unsafe_destructor)]

extern crate core;
extern crate libc;

use core::raw::Slice;
use libc::{c_char, c_uchar, c_ushort, c_int, c_void};
use std::ffi::CString;
use std::mem;
use std::marker::ContravariantLifetime;
use std::ptr;

static HTML_PARSE_NOERROR: c_int = 32;
static HTML_PARSE_NOWARNING: c_int = 64;

#[allow(non_camel_case_types)]
type xmlChar = c_uchar;

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct xmlDoc {
    _private: *mut c_void,
    type_: c_int, // xmlElementType
    name: *const xmlChar,
    children: *mut xmlNode,
    last: *mut xmlNode,
    parent: *mut xmlNode,
    next: *mut xmlNode,
    prev: *mut xmlNode,
    doc: *mut xmlDoc,
    compression: c_int,
    standalone: c_int,
    intSubset: *mut c_void, // *xmlDtd
    extSubset: *mut c_void,
    oldNs: *mut c_void, // *xmlNs
    version: *const xmlChar,
    encoding: *const xmlChar,
    ids: *mut c_void,
    refs: *mut c_void,
    URL: *const xmlChar,
    charest: c_int,
    dict: *mut c_void, // *xmlDict
    psvi: *mut c_void,
    parseFlags: c_int,
    properties: c_int
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct xmlNode {
    _private: *mut c_void,
    type_: c_int, // xmlElementType
    name: *const xmlChar,
    children: *mut xmlNode,
    last: *mut xmlNode,
    parent: *mut xmlNode,
    next: *mut xmlNode,
    prev: *mut xmlNode,
    doc: *mut xmlDoc,
    ns: *mut c_void, // *xmlNs
    content: *mut xmlChar, // *xmlChar
    properties: *mut c_void, // *xmlAttr
    nsDef: *mut c_void, // *xmlNs
    psvi: *mut c_void,
    line: c_ushort,
    extra: c_ushort
}

#[link(name = "xml2")]
#[allow(non_camel_case_types)]
extern {
    fn htmlReadMemory(buffer: *const c_char, size: c_int, url: *const c_char, encoding: *const c_char, options: c_int) -> *mut xmlDoc;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> *mut xmlNode;
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlFreeDoc(cur: *mut xmlDoc);
    fn xmlFreeNode(cur: *mut xmlNode);
    fn xmlStrlen(str: *const xmlChar) -> c_int;
}

#[allow(non_snake_case)]
unsafe fn xmlFree(ptr: *mut c_void) {
    libc::free(ptr);
}

unsafe fn xml_str_to_slice<'a>(xml_str: *const xmlChar) -> &'a str {
    mem::transmute(Slice { data: xml_str, len: xmlStrlen(xml_str) as usize })
}

struct XmlDoc {
    ptr: *mut xmlDoc
}

impl Drop for XmlDoc {
    fn drop(&mut self) {
        unsafe { xmlFreeDoc(self.ptr); }
    }
}

impl XmlDoc {
    fn from_html_str(html: &str) -> Option<XmlDoc> {
        unsafe {
            let c_html = CString::from_slice(html.as_bytes());
            let c_enc = CString::from_slice("".as_bytes());
            let doc_ptr = htmlReadMemory(c_html.as_ptr(), html.len() as c_int, c_enc.as_ptr(), ptr::null(), HTML_PARSE_NOWARNING | HTML_PARSE_NOERROR);
            if doc_ptr.is_null() {
                None
            } else {
                Some(XmlDoc { ptr: doc_ptr })
            }
        }
    }

    fn root_element<'a>(&'a self) -> XmlNode {
        unsafe {
            XmlNode {
                ptr: xmlDocGetRootElement(mem::transmute(self.ptr)),
                lt: ContravariantLifetime::<'a>
            }
        }
    }
}

struct XmlNode<'a> {
    ptr: *mut xmlNode,
    lt: ContravariantLifetime<'a>
}

#[unsafe_destructor]
impl<'a> Drop for XmlNode<'a> {
    fn drop(&mut self) {
        unsafe {
            if (*self.ptr).parent.is_null() {
               xmlFreeNode(self.ptr);
            }
        }
    }
}

impl<'a> XmlNode<'a> {
    fn name(&self) -> &str {
        unsafe { xml_str_to_slice((*self.ptr).name) }
    }

    fn content(&self) -> Option<String> {
        unsafe {
            let content = xmlNodeGetContent(self.ptr as *const xmlNode);
            if content.is_null() {
                None
            } else {
                let content_string = xml_str_to_slice(content as *const xmlChar).to_string();
                xmlFree(content as *mut c_void);
                Some(content_string)
            }
        }
    }

    fn children<'b>(&'b self) -> XmlNodeIterator {
        XmlNodeIterator {
            ptr: unsafe { (*self.ptr).children },
            lt: ContravariantLifetime::<'b>
        }
    }
}

struct XmlNodeIterator<'a> {
    ptr: *mut xmlNode,
    lt: ContravariantLifetime<'a>
}

impl<'a> Iterator for XmlNodeIterator<'a> {
    type Item = XmlNode<'a>;

    fn next<'b>(&mut self) -> Option<XmlNode<'b>> {
        //  ^^  ^^^^ -- Not `&'a mut`. Why? (This might help: http://stackoverflow.com/a/24848424 )
        unsafe {
            if self.ptr.is_null() {
                return None
            }
            let node = self.ptr;
            self.ptr = self.ptr.offset(1);
            Some(XmlNode {
                ptr: node,
                lt: ContravariantLifetime::<'b>
            })
        }
    }
}

fn main() {
    let doc = XmlDoc::from_html_str("<html><body>BODY</body></html>").expect("must be parsable");
    let root = doc.root_element();
    println!("root element name: {}", root.name());
    println!("root element content: {:?}", root.content());
    for node in root.children() {
        println!("child element name: {}", node.name());
        println!("child element content: {:?}", node.content());
    }
}
