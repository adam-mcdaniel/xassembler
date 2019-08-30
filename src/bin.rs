extern crate xassembler;
use xassembler::*;

fn main() {
    println!(
        "extern crate xmachine;
use xmachine::{{Machine, Value, Ref}};


fn dict(xasm: &mut Machine) {{
    xasm.push(Value::tree());
}}

fn list(xasm: &mut Machine) {{
    xasm.push(Value::list());
}}

fn push(xasm: &mut Machine) {{
    if let Some(list_value) = xasm.pop() {{
        if let Value::List(mut l) = (*list_value).clone() {{
            if let Some(value) = xasm.pop() {{
                l.push(value);
                xasm.push(Ref::new(Value::List(l)));
            }}
        }}
    }}
}}

fn pop(xasm: &mut Machine) {{
    if let Some(value) = xasm.pop() {{
        if let Value::List(mut l) = (*value).clone() {{
            let last_value = l[l.len() - 1].clone();
            l.pop();
            xasm.push(last_value.copy());
            xasm.push(Ref::new(Value::List(l)));
        }}
    }}
}}

fn len(xasm: &mut Machine) {{
    if let Some(value) = xasm.pop() {{
        if let Value::List(l) = (*value).clone() {{
            xasm.push(Value::number(l.len() as f64));
        }}
    }}
}}

fn print(xasm: &mut Machine) {{
    if let Some(string) = xasm.pop() {{
        print!(\"{{}}\", string);
    }}
}}

fn println(xasm: &mut Machine) {{
    if let Some(string) = xasm.pop() {{
        println!(\"{{}}\", string);
    }}
}}

fn new(xasm: &mut Machine) {{
    if let Some(class) = xasm.pop() {{
        class.call(xasm);
        xasm.push(Value::string(\"new\"));
        xasm.method_call();
    }}
}}


fn add(xasm: &mut Machine) {{
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {{
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f + n_f)
        );
    }}
}}

fn sub(xasm: &mut Machine) {{
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {{
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f - n_f)
        );
    }}
}}

fn mul(xasm: &mut Machine) {{
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {{
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f * n_f)
        );
    }}
}}

fn div(xasm: &mut Machine) {{
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {{
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f / n_f)
        );
    }}
}}

fn main() {{
    let mut xasm = Machine::new();
    xasm.push(Value::function(dict, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"dict\"));
    xasm.store();

    xasm.push(Value::function(list, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"list\"));
    xasm.store();
    xasm.push(Value::function(len, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"len\"));
    xasm.store();
    xasm.push(Value::function(push, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"push\"));
    xasm.store();
    xasm.push(Value::function(pop, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"pop\"));
    xasm.store();

    xasm.push(Value::function(print, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"print\"));
    xasm.store();
    xasm.push(Value::function(println, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"println\"));
    xasm.store();
    xasm.push(Value::function(new, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"new\"));
    xasm.store();

    xasm.push(Value::function(add, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"add\"));
    xasm.store();
    xasm.push(Value::function(sub, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"sub\"));
    xasm.store();
    xasm.push(Value::function(mul, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"mul\"));
    xasm.store();
    xasm.push(Value::function(div, &xasm));
    xasm.copy();
    xasm.push(Value::string(\"div\"));
    xasm.store();


\t{}
    println!(\"{{}}\", xasm);


}}",
        program()
            .parse(
                r#"


class List {
    fn new(self) {
        self.list = list()
        self
    }

    fn push(self, item) {
        self.list = push(self.list, item)
    }

    fn pop(self) {
        if len(self.list) {
            self.list = pop(self.list)
        } else {
            None
        }
    }

    fn len(self) {
        len(self.list)
    }
}


l = new(List)
l.push("test")
l.push(l)
l.push(l)


println(l)

if l {
    println("l is true!!")
} else {
    println("l does not evaluate to true :(")
}


n = 1000
while n {
    n = sub(n, 1)
    println(n)

    if n {}
    else {}
}

"#
            )
            .unwrap()
            .compile()
            .unwrap()
            .replace(";", ";\n\t")
    )
}