#![allow(deref_nullptr)]

#[macro_export]
macro_rules! gen_attrib_pointers {
    ($struct_name:ident, $($index:expr => $field_name:ident: $dimension:expr),*) => {
        $(
            let offset = &((*std::ptr::null::<$struct_name>()).$field_name) as *const _ as *const std::ffi::c_void;
            EnableVertexAttribArray($index);
            VertexAttribPointer($index, $dimension, FLOAT, FALSE, std::mem::size_of::<$struct_name>() as GLsizei, offset);
        )*
    };
}

#[macro_export]
macro_rules! bind_buffer {
    ($buffer_type:expr, $buffer:expr, $data:expr) => {{
        BindBuffer($buffer_type, $buffer);
        let size = ($data.len() * std::mem::size_of_val(&$data[0])) as isize;
        let data_ptr = &$data[0] as *const _ as *const std::ffi::c_void;
        BufferData($buffer_type, size, data_ptr, STATIC_DRAW);
    }};
}

#[macro_export]
macro_rules! im_debug {
    ($name:expr, $data:expr, $frame:expr) => {{
        let name = $name;
        let increase_indent = ['(', '{', '[', ':', ','];
        let decrease_indent = [')', '}', ']'];

        let mut result = String::new();
        let mut indent_level = 0;
        
        let data_str = format!("{:?}", $data);

        for part in data_str.split(|c| increase_indent.contains(&c) || decrease_indent.contains(&c)) {
            let trimmed_part = part.trim();
            if trimmed_part.is_empty() {
                continue;
            }

            if let Some(first_char) = trimmed_part.chars().next() {
                if decrease_indent.contains(&first_char) && indent_level > 0 {
                    indent_level -= 1;
                }
            }

            let indentation = "    ".repeat(indent_level);
            result.push_str(&indentation);
            result.push_str(trimmed_part);
            result.push_str("\n");
            
            if let Some(last_char) = trimmed_part.chars().last() {
                if increase_indent.contains(&last_char) {
                    indent_level += 1;
                }
            }
        }

        $frame.text(format!("{}:\n{}", name, result));
    }};
}