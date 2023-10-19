use proc_macro2::TokenStream;
use quote::quote;

use super::data::Data;

pub(crate) fn generate_struct_definition(data: &Data) -> TokenStream {
    let name = &data.type_name;
    let block_count = calc_size::<usize>(data);

    quote! {
        struct #name([usize; #block_count]);
    }
}

pub(crate) fn generate_struct_impl(data: &Data) -> TokenStream {
    let type_name = &data.type_name;
    
    let field_body = generate_impl_body(&data);

    quote! {
        impl #type_name {
            #(#field_body)*
        }
    }
}

fn calc_size<T>(data: &Data) -> usize {
    let total_size = data.fields
        .pairs()
        .map(|p| p.value().bits.base10_parse::<usize>())
        .map(|lit| lit.unwrap_or(0))
        .sum::<usize>();

    let total_size = match total_size % 8 {
        0 => total_size / 8,
        _ => total_size / 8 + 1,
    };

    let ptr_size = std::mem::size_of::<T>();
    let blocks = (total_size / ptr_size).min(1);
    let rem = total_size.checked_rem(blocks * ptr_size).unwrap_or(1);

    match rem {
        0 => blocks,
        _ => blocks + 1,
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
                let size_ratio = std::mem::size_of::<#ty>();
                let ptr = unsafe {
                    self.0
                        .as_slice()
                        .as_ptr()
                        .cast::<u8>()
                        .add(#offset / 8)
                };

                let mut output = 0;

                let bytes_to_traverse = #size / 8;

                for i in 0..=bytes_to_traverse {
                    let data = unsafe { *ptr.add(i) };

                    for n in 0..8 {
                        output += ((data >> n) & 1) << (bytes_to_traverse - i + n);
                    }
                }

                output as #ty
            }
        };

        streams.push(stream);
    }

    streams
}
