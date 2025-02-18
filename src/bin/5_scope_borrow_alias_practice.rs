struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };

    // Immutable borrow
    let rect_ref1 = &rect;
    let rect_ref2 = &rect;

    // Access data via immutable references
    println!("Rectangle dimensions: width = {}, height = {}", rect_ref1.width, rect_ref2.height);

    // Error! Can't borrow `rect` as mutable because it's currently borrowed as immutable.
    // let rect_mut_ref = &mut rect;
    // TODO ^ Try uncommenting this line

    // Use the immutable references again
    println!("Rectangle dimensions: width = {}, height = {}", rect_ref1.width, rect_ref2.height);

    // The immutable references are no longer used, so we can borrow mutably
    let rect_mut_ref = &mut rect;

    // Modify the rectangle via the mutable reference
    rect_mut_ref.width = 15;
    rect_mut_ref.height = 25;

    // Error! Can't borrow `rect` as immutable because it's currently borrowed as mutable.
    // let width = &rect.width;
    // TODO ^ Try uncommenting this line

    // Error! Can't print using the original variable because it's mutably borrowed.
    // println!("Rectangle width: {}", rect.width);
    // TODO ^ Try uncommenting this line

    // Print using the mutable reference
    println!("Rectangle dimensions: width = {}, height = {}", rect_mut_ref.width, rect_mut_ref.height);

    // The mutable reference is no longer used, so we can borrow immutably again
    let new_rect_ref = &rect;
    println!("Rectangle dimensions: width = {}, height = {}", new_rect_ref.width, new_rect_ref.height);
}
