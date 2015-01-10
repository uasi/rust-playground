#[derive(Show)]
struct Noncloneable;

#[derive(Show, Clone)]
struct Cloneable;

fn main() {
    let vn: Vec<Noncloneable> = Vec::new();
    let _: &[Noncloneable] = vn.clone();
    //                       ^^^^^^^^^^
    //                       vn auto-derefs to &[Noncloneable], which impls Clone,
    //                       so this expr returns a cloned &[Nonclonable].

    let vc: Vec<Cloneable> = Vec::new();
    let _: Vec<Cloneable> = vc.clone();
    //                      ^^^^^^^^^^
    //                      Vec<Clonable> itself impls Clone,
    //                      so this expr returns a cloned Vec<Cloneable>.
}

