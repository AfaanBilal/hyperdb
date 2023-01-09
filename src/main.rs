mod hyper;

/**
 * HyperDB
 *
 * In-memory hyper-fast key-value store.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */

fn main() {
    let mut hs = hyper::HyperStore::new("store.hyper");

    println!("[HyperStore] init: {}", hs.get_file());

    assert_eq!(hs.is_empty(), true);
    assert_eq!(hs.has("hyper"), false);

    hs.set("hyper", "store");

    assert_ne!(hs.is_empty(), true);
    assert_ne!(hs.has("hyper"), false);

    println!("{}", hs.get("keyNotFound"));

    hs.set("hyper", "fast");

    println!("{}", hs.get("hyper"));

    hs.print_all();

    hs.clear();

    assert_eq!(hs.is_empty(), true);

    hs.print_all();
}
