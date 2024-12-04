use std::cell::RefCell;

thread_local{
    static MSG: RefCell<String> = RefCell::new(String::new());
}


#[ic_cdk::query]
fn greet(name: String) -> String {
    MSG.with(|msg|{
        msg.borrow.mut()
    })
}


#[ic_cdk::query]
fn get_msg() -> String {
    msg.with(|msg| msg.borrow().clone())
}
