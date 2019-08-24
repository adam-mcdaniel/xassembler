extern crate xmachine;
use xmachine::{Machine, Value, Ref};


fn dict(xasm: &mut Machine) {
    xasm.push(Value::tree());
}

fn list(xasm: &mut Machine) {
    xasm.push(Value::list());
}

fn push(xasm: &mut Machine) {
    if let Some(list_value) = xasm.pop() {
        if let Value::List(mut l) = (*list_value).clone() {
            if let Some(value) = xasm.pop() {
                l.push(value);
                xasm.push(Ref::new(Value::List(l)));
            }
        }
    }
}

fn pop(xasm: &mut Machine) {
    if let Some(value) = xasm.pop() {
        if let Value::List(mut l) = (*value).clone() {
            let last_value = l[l.len() - 1].clone();
            l.pop();
            xasm.push(last_value.copy());
            xasm.push(Ref::new(Value::List(l)));
        }
    }
}

fn len(xasm: &mut Machine) {
    if let Some(value) = xasm.pop() {
        if let Value::List(l) = (*value).clone() {
            xasm.push(Value::number(l.len() as f64));
        }
    }
}

fn print(xasm: &mut Machine) {
    if let Some(string) = xasm.pop() {
        print!("{}", string);
    }
}

fn println(xasm: &mut Machine) {
    if let Some(string) = xasm.pop() {
        println!("{}", string);
    }
}

fn new(xasm: &mut Machine) {
    if let Some(class) = xasm.pop() {
        class.call(xasm);
        xasm.push(Value::string("new"));
        xasm.method_call();
    }
}


fn add(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f + n_f)
        );
    }
}

fn sub(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f - n_f)
        );
    }
}

fn mul(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f * n_f)
        );
    }
}

fn div(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f / n_f)
        );
    }
}

fn main() {
    let mut xasm = Machine::new();
    xasm.push(Value::function(dict, &xasm));
    xasm.copy();
    xasm.push(Value::string("dict"));
    xasm.store();

    xasm.push(Value::function(list, &xasm));
    xasm.copy();
    xasm.push(Value::string("list"));
    xasm.store();
    xasm.push(Value::function(len, &xasm));
    xasm.copy();
    xasm.push(Value::string("len"));
    xasm.store();
    xasm.push(Value::function(push, &xasm));
    xasm.copy();
    xasm.push(Value::string("push"));
    xasm.store();
    xasm.push(Value::function(pop, &xasm));
    xasm.copy();
    xasm.push(Value::string("pop"));
    xasm.store();

    xasm.push(Value::function(print, &xasm));
    xasm.copy();
    xasm.push(Value::string("print"));
    xasm.store();
    xasm.push(Value::function(println, &xasm));
    xasm.copy();
    xasm.push(Value::string("println"));
    xasm.store();
    xasm.push(Value::function(new, &xasm));
    xasm.copy();
    xasm.push(Value::string("new"));
    xasm.store();

    xasm.push(Value::function(add, &xasm));
    xasm.copy();
    xasm.push(Value::string("add"));
    xasm.store();
    xasm.push(Value::function(sub, &xasm));
    xasm.copy();
    xasm.push(Value::string("sub"));
    xasm.store();
    xasm.push(Value::function(mul, &xasm));
    xasm.copy();
    xasm.push(Value::string("mul"));
    xasm.store();
    xasm.push(Value::function(div, &xasm));
    xasm.copy();
    xasm.push(Value::string("div"));
    xasm.store();


xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("dict"));
xasm.load();
xasm.call();
xasm.copy();
xasm.push(Value::string("self"));
xasm.store();
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("self"));
xasm.store();
xasm.push(Value::string("list"));
xasm.load();
xasm.call();
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("list"));
xasm.index();
xasm.assign();
xasm.push(Value::string("self"));
xasm.load();
}, &xasm));
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("new"));
xasm.index();
xasm.assign();
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("self"));
xasm.store();

xasm.push(Value::string("item"));
xasm.store();
xasm.push(Value::string("item"));
xasm.load();
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("list"));
xasm.index();
xasm.copy();
xasm.push(Value::string("push"));
xasm.load();
xasm.call();
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("list"));
xasm.index();
xasm.assign();
}, &xasm));
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("push"));
xasm.index();
xasm.assign();
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("self"));
xasm.store();
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("None"));
xasm.load();
}, &xasm));
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("list"));
xasm.index();
xasm.copy();
xasm.push(Value::string("pop"));
xasm.load();
xasm.call();
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("list"));
xasm.index();
xasm.assign();
}, &xasm));
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("list"));
xasm.index();
xasm.copy();
xasm.push(Value::string("len"));
xasm.load();
xasm.call();
}, &xasm));
xasm.if_then_else();
}, &xasm));
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("pop"));
xasm.index();
xasm.assign();
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("self"));
xasm.store();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("list"));
xasm.index();
xasm.copy();
xasm.push(Value::string("len"));
xasm.load();
xasm.call();
}, &xasm));
xasm.copy();
xasm.push(Value::string("self"));
xasm.load();
xasm.push(Value::string("len"));
xasm.index();
xasm.assign();
xasm.push(Value::string("self"));
xasm.load();
}, &xasm));
xasm.copy();
xasm.push(Value::string("List"));
xasm.store();
xasm.push(Value::string("List"));
xasm.load();
xasm.copy();
xasm.push(Value::string("new"));
xasm.load();
xasm.call();
xasm.copy();
xasm.push(Value::string("l"));
xasm.store();
xasm.push(Value::number(1000));
xasm.copy();
xasm.push(Value::string("n"));
xasm.store();
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("n"));
xasm.load();
xasm.copy();
xasm.push(Value::string("l"));
xasm.load();
xasm.push(Value::string("push"));
xasm.method_call();
xasm.push(Value::number(1));
xasm.copy();
xasm.push(Value::string("n"));
xasm.load();
xasm.copy();
xasm.push(Value::string("sub"));
xasm.load();
xasm.call();
xasm.copy();
xasm.push(Value::string("n"));
xasm.store();
}, &xasm));
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("n"));
xasm.load();
}, &xasm));
xasm.while_loop();
xasm.push(Value::number(100000));
xasm.copy();
xasm.push(Value::string("n"));
xasm.store();
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::number(1));
xasm.copy();
xasm.push(Value::string("n"));
xasm.load();
xasm.copy();
xasm.push(Value::string("sub"));
xasm.load();
xasm.call();
xasm.copy();
xasm.push(Value::string("n"));
xasm.store();
xasm.push(Value::string("n"));
xasm.load();
xasm.copy();
xasm.push(Value::string("println"));
xasm.load();
xasm.call();
}, &xasm));
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("n"));
xasm.load();
}, &xasm));
xasm.while_loop();
xasm.push(Value::string("l"));
xasm.load();
xasm.copy();
xasm.push(Value::string("println"));
xasm.load();
xasm.call();

    println!("{}", xasm);


}
