use proc_macro2::TokenStream;
use quote::quote;

use super::data::Data;

pub(crate) fn generate_struct_definition(data: &Data) -> TokenStream {
    let name = &data.type_name;
    let block_count = data.field_sizes().iter().sum::<usize>();
    let block_count = match block_count % 8 {
        0 => block_count / 8,
        _ => block_count / 8 + 1,
    };

    quote! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        struct #name([u8; #block_count]);
    }
}

pub(crate) fn generate_struct_impl(data: &Data) -> TokenStream {
    let type_name = &data.type_name;

    let type_name_literal = type_name.to_string();

    let new_fn = generate_new_body(&data);
    let field_body = generate_impl_body(&data);

    let field_name = data.field_names();
    let field_name_literal = data.field_names().into_iter().map(|i| i.to_string());

    quote! {
        #[allow(unused_variables)]
        impl #type_name {
            #new_fn
            #(#field_body)*
        }

        impl std::fmt::Debug for #type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.debug_struct(#type_name_literal)
                 #(.field(#field_name_literal, &self.#field_name()))*
                 .finish()
            }
        }
    }
}


fn generate_impl_body(data: &Data) -> Vec<TokenStream> {
    let mut streams = Vec::<TokenStream>::with_capacity(data.field_count());

    let names = data.field_names();
    let types = data.field_types();
    let offsets = data.field_cumulative_offsets();
    let sizes = data.field_sizes();

    let i = names
        .iter()
        .zip(types.iter())
        .zip(offsets.iter())
        .zip(sizes.iter())
        .map(|a| (a.0.0.0, a.0.0.1, a.0.1, a.1));

    for (name, ty, &offset, &size) in i {
        let stream = quote! {
            pub fn #name(&self) -> #ty {
                let ptr = &self.0[0] as *const u8;

                let mut output: #ty = 0;
                
                let bytes = match #size % 8 {
                    0 => #size / 8,
                    _ => #size / 8 + 1,
                };

                let mut cursor = #offset;

                while (cursor - #offset + 1) <= #size {
                    let bit_cursor = cursor - #offset + 1;
                    unsafe {
                        let current_byte_ptr = ptr.add(cursor / 8);
                        output |= ((((*current_byte_ptr) >> (7 - cursor % 8)) & 1) << (#size - bit_cursor)) as #ty;
                    }

                    cursor += 1;
                }

                output
            }
        };

        streams.push(stream);
    }

    streams
}

fn generate_new_body(data: &Data) -> TokenStream {
    let field_name = data.field_names();
    let field_type = data.field_types();
    let field_size = data.field_sizes();

    let total_size: usize = field_size.iter().sum();
    let total_size = match total_size % 8 {
        0 => total_size / 8,
        _ => total_size / 8 + 1,
    };

    let offset = data.field_cumulative_offsets();

    quote! {
        pub fn new(#(#field_name: #field_type),*) -> Self {
            let mut inner = [0u8; #total_size];

            let mut bit_cursor = 0;
        
            let start_ptr = &mut inner[0] as *mut u8;

            #(
                let field_size = #field_size;

                while let bit_pos @ (1..=#field_size) = (bit_cursor - #offset + 1) {
                    let current_byte_ptr = unsafe { start_ptr.add((bit_pos - 1) / 8) };

                    unsafe { *(&mut *current_byte_ptr) |= (1 << 7 - ((bit_pos - 1) % 8)) };

                    bit_cursor += 1;
                }
            )*

            Self(inner)
        }

        pub const fn new_zeroed() -> Self {
            Self([0u8; #total_size])
        }
    }
}
