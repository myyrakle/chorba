#[proc_macro_derive(Encoder)]
pub fn derive_encode(item: TokenStream) -> TokenStream {
    let mut new_code = "".to_string();

    let ast = syn::parse_macro_input!(item as syn::ItemStruct);

    let struct_name = ast.ident.to_string();

    new_code += format!(r#"impl chorba::Encoder for {struct_name} {{"#).as_str();

    {
        new_code += r#"fn encode(&self) -> Vec<u8> {"#;

        {
            new_code += r#"let mut buffer = Vec::new();"#;

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

#[proc_macro_derive(Decoder)]
pub fn derive_encode(item: TokenStream) -> TokenStream {
    let mut new_code = "".to_string();

    let ast = syn::parse_macro_input!(item as syn::ItemStruct);

    let struct_name = ast.ident.to_string();

    new_code += format!(r#"impl chorba::Decoder<{struct_name}> for {struct_name} {{"#).as_str();

    {
        new_code += format!(r#"fn decode(buffer: &[u8]) -> Result<{struct_name}, DecodeError> {{"#)
            .as_str();

        {
            let mut field_names = vec![];

            for field in ast.fields.iter() {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let field_type = field.ty.to_token_stream().to_string();

                field_names.push(field_name.clone());

                new_code +=
                    format!(r#"let ({field_name}_bytes, buffer) = chorba::deserialize(buffer)?;"#)
                        .as_str();

                new_code +=
                    format!(r#"let {field_name} = chorba::Decoder<{field_type}>::decode({field_name}_bytes);"#)
                        .as_str();
            }

            new_code += format!(r#"Ok({struct_name} {{"#).as_str();

            for field_name in field_names.iter() {
                new_code += format!(r#"{field_name},"#).as_str();
            }

            new_code += "}";
        }

        new_code += r#"}"#;
    }

    new_code += "}";

    return TokenStream::from_str(new_code.as_str()).unwrap();
}
