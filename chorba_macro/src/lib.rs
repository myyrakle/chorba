#[proc_macro_derive(Encode)]
pub fn derive_encode(item: TokenStream) -> TokenStream {
    let mut new_code = "".to_string();

    let ast = syn::parse_macro_input!(item as syn::ItemStruct);

    let struct_name = ast.ident.to_string();

    new_code += format!(r#"impl chorba::Encoder for {struct_name} {{"#).as_str();

    {
        new_code += r#"fn encode(&self) -> Vec<u8> {"#;

        {
            new_code += r#"let mut buffer = Vec::new();"#;

            let mut need_comma = false;

            for field in ast.fields.iter() {
                let field_name = field.ident.as_ref().unwrap().to_string();

                new_code +=
                    format!(r#"let bytes = chorba::Encoder::encode(self.{field_name});"#).as_str();
                new_code += format!(r#"buffer.extend(bytes);"#).as_str();
            }

            new_code += r#"buffer"#;
        }

        new_code += r#"}"#;
    }

    new_code += "}";

    return TokenStream::from_str(new_code.as_str()).unwrap();
}
