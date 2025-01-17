use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

pub(crate) fn macroquad_main(
    core_crate: &Ident,
    core_impl: &Ident,
    window_title: String,
    error_type: &Path,
) -> TokenStream {
    let macroquad_rename = format!("{}::macroquad", &core_crate);

    quote! {
        pub fn window_conf() -> #core_crate::macroquad::window::Conf {
            let path = #core_impl::config_path();

            let config = #core_crate::config::load_config_sync(&path)
                .unwrap_or_else(|err| panic!("Error: {}", &path));

            let mq = config.to_macroquad(#window_title, #core_impl::window_icon(), true);

            #core_crate::config::set_config(config);

            mq
        }

        #[#core_crate::macroquad::main(window_conf)]
        #[macroquad(crate_rename = #macroquad_rename)]
        async fn main() -> std::result::Result<(), #error_type> {
            main_inner().await?;

            Ok(())
        }
    }
}
