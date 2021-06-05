use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn public(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut pub_token = TokenStream::from(TokenTree::Ident(Ident::new("pub", Span::call_site())));
    if args.clone().is_empty() {
        let crate_token = TokenStream::from(TokenTree::Ident(Ident::new("crate", Span::call_site())));
        let scope_group = Group::new(Delimiter::Parenthesis, crate_token);
        pub_token.extend(TokenStream::from(TokenTree::Group(scope_group)));
    } else {
        let scope_group = Group::new(Delimiter::Parenthesis, args);
        pub_token.extend(TokenStream::from(TokenTree::Group(scope_group)));
    }

    let mut output = TokenStream::new();
    let mut is_first_ident = true;

    let token_stream_iter = input.clone().into_iter();
    for token in token_stream_iter {
        match token {
            TokenTree::Punct(p) => {
                output.extend(TokenStream::from(TokenTree::Punct(p)));
            }
            TokenTree::Group(g) => match g.delimiter() {
                Delimiter::Brace => {
                    let group_token_stream_iter = g.stream().clone().into_iter();

                    let mut group_token = TokenStream::new();
                    let mut is_identifier = true;

                    for token_ in group_token_stream_iter {
                        match token_ {
                            TokenTree::Ident(i) => {
                                if is_identifier {
                                    group_token.extend(pub_token.clone());
                                    is_identifier = false;
                                }
                                group_token.extend(TokenStream::from(TokenTree::Ident(i)));
                            }
                            TokenTree::Punct(p) => {
                                let p_str = &p.to_string();
                                if p_str == "," {
                                    is_identifier = true;
                                }
                                group_token.extend(TokenStream::from(TokenTree::Punct(p)));
                            }
                            TokenTree::Literal(l) => {
                                group_token.extend(TokenStream::from(TokenTree::Literal(l)))
                            }
                            TokenTree::Group(g) => {
                                group_token.extend(TokenStream::from(TokenTree::Group(g)))
                            }
                        }
                    }
                    let group = Group::new(Delimiter::Brace, group_token);
                    output.extend(TokenStream::from(TokenTree::Group(group)));
                }
                _ => output.extend(TokenStream::from(TokenTree::Group(g))),
            },
            TokenTree::Ident(i) => {
                if is_first_ident {
                    output.extend(pub_token.clone());
                    is_first_ident = false;
                }
                output.extend(TokenStream::from(TokenTree::Ident(i)));
            }
            TokenTree::Literal(l) => {
                output.extend(TokenStream::from(TokenTree::Literal(l)));
            }
        }
    }

    output
}
