pub fn menu(tools_array:&[Tool]) {
    let mut text = String::new();
    text.push_str("SCMDT: Simple Command line tools\n");
    text.push_str("Type the number of the tool you want or type q or quit to quit the program\n");
    let mut num:u8 = 0;
    for tool in tools_array {
        num +=1;
        text.push_str(&num.to_string());
        text.push_str("-");
        text.push_str(&tool.name);
        text.push_str("\n")
    }
    print!("{}",text); // Apended all the text to a variable before printing to avoid using multiple print! macros
    let tool = get_tool(tools_array.len());
    (tools_array[tool].function)()
}

fn get_tool(n_tools:usize) -> usize
{
    loop {
        let input = crate::input::get_tool_or_quit("Please enter a number")-1;
        if input <= n_tools {
            return input;
        }
        else {
            println!("Invalid option")
        }
    };
}

#[derive(Clone,PartialEq, Eq)]
pub struct Tool {
    function: fn(),
    name: String,
}
impl Tool {
    pub fn from(function:fn(), name:String) -> Tool {
        Tool{
            function:function,
            name:name
        }
    }
}

