use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Bracket},
    Expr, ExprLet, Ident, Result, Token, Type,
};

mod kw {
    syn::custom_keyword!(var);
    syn::custom_keyword!(cloned);
}

struct Action {
    fn_name: Ident,
    arg: Option<Expr>,
    is_call_optional: bool,
}
impl Parse for Action {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Brace) {
            let fn_name_and_value;
            let _ = braced!(fn_name_and_value in input);

            let arg: Expr = fn_name_and_value.fork().parse()?;
            let fn_name_and_value: Ident = fn_name_and_value.parse()?;

            Ok(Action {
                fn_name: fn_name_and_value,
                arg: Some(arg),
                is_call_optional: false,
            })
        } else {
            let fn_name: Ident = input.parse()?;
            let is_call_optional = input.parse::<Token![?]>().is_ok();

            match input.parse::<Token![=]>() {
                Ok(_) => {
                    let arg;
                    let _ = braced!(arg in input);

                    if arg.is_empty() {
                        let error_msg = format!("Argument for `{fn_name}` expected. If `{fn_name}` does not require an argument, call it like this: `{fn_name}`");
                        return Err(arg.error(error_msg));
                    }

                    let arg: Expr = arg.parse()?;

                    Ok(Action {
                        fn_name,
                        arg: Some(arg),
                        is_call_optional,
                    })
                }
                Err(_) => Ok(Action {
                    fn_name,
                    arg: None,
                    is_call_optional: false,
                }),
            }
        }
    }
}
impl From<Action> for TokenStream {
    fn from(
        Action {
            fn_name,
            arg,
            is_call_optional,
        }: Action,
    ) -> Self {
        let arg = if let Some(arg) = arg {
            quote!(#arg)
        } else {
            quote!()
        };

        if is_call_optional {
            quote!(
                if let Some(arg) = #arg {
                    __current_shape__.#fn_name(arg);
                }
            )
        } else {
            quote!(
                __current_shape__.#fn_name(#arg);
            )
        }
    }
}

struct Actions(Vec<Action>);
impl Parse for Actions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut res = vec![];

        while !input.is_empty() {
            res.push(input.parse()?);
        }

        Ok(Actions(res))
    }
}
impl From<Actions> for TokenStream {
    fn from(Actions(actions): Actions) -> Self {
        let actions = actions
            .into_iter()
            .map(TokenStream::from)
            .collect::<Vec<_>>();

        quote!(#(#actions)*)
    }
}

// ()
// ( f={v} )
// ( {v} )
struct ShapeActions {
    is_style: bool,
    actions: Actions,
}
impl Parse for ShapeActions {
    fn parse(input: ParseStream) -> Result<Self> {
        let is_style = input.parse::<Token![#]>().is_ok();

        let actions;
        let _ = parenthesized!(actions in input);
        let actions: Actions = actions.parse()?;

        Ok(ShapeActions { is_style, actions })
    }
}

// var(...)
// cloned(...)

// Item: ()
//
// Item: #()
struct DessinItem {
    item_name: Type,
    shape_actions: ShapeActions,
}
impl Parse for DessinItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item_name: Type = input.parse()?;
        let _: Token![:] = input.parse()?;
        let shape_actions: ShapeActions = input.parse()?;

        Ok(DessinItem {
            item_name,
            shape_actions,
        })
    }
}
impl From<DessinItem> for TokenStream {
    fn from(
        DessinItem {
            item_name,
            shape_actions,
        }: DessinItem,
    ) -> Self {
        let base_shape = if shape_actions.is_style {
            quote!(::dessin::prelude::Style::new(<#item_name>::default()))
        } else {
            quote!(<#item_name>::default())
        };

        let is_actions_empty = shape_actions.actions.0.is_empty();
        let actions = TokenStream::from(shape_actions.actions);

        if is_actions_empty {
            base_shape
        } else {
            quote!({
                let mut __current_shape__ = #base_shape;
                #actions
                __current_shape__
            })
        }
    }
}

struct DessinVar {
    var: TokenStream,
    shape_actions: ShapeActions,
}
impl Parse for DessinVar {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: kw::var = input.parse()?;

        let var;
        let _ = parenthesized!(var in input);
        let var: TokenStream = var.parse()?;

        let _: Token![:] = input.parse()?;
        let shape_actions: ShapeActions = input.parse()?;

        Ok(DessinVar { var, shape_actions })
    }
}
impl From<DessinVar> for TokenStream {
    fn from(DessinVar { var, shape_actions }: DessinVar) -> Self {
        let base_shape = if shape_actions.is_style {
            quote!(::dessin::prelude::Style::new(#var))
        } else {
            quote!(#var)
        };

        let is_actions_empty = shape_actions.actions.0.is_empty();
        let actions = TokenStream::from(shape_actions.actions);

        if is_actions_empty {
            base_shape
        } else {
            quote!({
                let mut __current_shape__ = #base_shape;
                #actions
                __current_shape__
            })
        }
    }
}

struct DessinCloned {
    var: Expr,
    shape_actions: ShapeActions,
}
impl Parse for DessinCloned {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: kw::cloned = input.parse()?;

        let var;
        let _ = parenthesized!(var in input);
        let var: Expr = var.parse()?;

        let _: Token![:] = input.parse()?;
        let shape_actions: ShapeActions = input.parse()?;

        Ok(DessinCloned { var, shape_actions })
    }
}
impl From<DessinCloned> for TokenStream {
    fn from(DessinCloned { var, shape_actions }: DessinCloned) -> Self {
        let base_shape = if shape_actions.is_style {
            quote!(::dessin::prelude::Style::new(#var.clone()))
        } else {
            quote!(#var.clone())
        };

        let is_actions_empty = shape_actions.actions.0.is_empty();
        let actions = TokenStream::from(shape_actions.actions);

        if is_actions_empty {
            base_shape
        } else {
            quote!({
                let mut __current_shape__ = #base_shape;
                #actions
                __current_shape__
            })
        }
    }
}

struct DessinGroup {
    children: Punctuated<Dessin, Token![,]>,
}
impl Parse for DessinGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let children;
        let _ = bracketed!(children in input);

        let children = children.parse_terminated(Dessin::parse, Token![,])?;

        Ok(DessinGroup { children })
    }
}
impl From<DessinGroup> for TokenStream {
    fn from(DessinGroup { children }: DessinGroup) -> Self {
        let children = children
            .into_iter()
            .map(TokenStream::from)
            .collect::<Vec<_>>();

        quote!(::dessin::prelude::Shape::Group(::dessin::prelude::Group {
            local_transform: ::dessin::nalgebra::Transform2::default(),
            metadata: ::std::vec::Vec::new(),
            shapes: ::std::vec![
                #(::dessin::prelude::Shape::from(#children)),*
            ],
        }))
    }
}

enum DessinArg {
    Let(ExprLet),
    Ident(Ident),
    Expr(Expr),
}
impl Parse for DessinArg {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![let]) {
            let let_exp: ExprLet = input.parse()?;
            return Ok(DessinArg::Let(let_exp));
        }

        let is_ident = input.peek(Ident) && input.peek2(Brace);
        if is_ident {
            let ident: Ident = input.parse()?;
            return Ok(DessinArg::Ident(ident));
        }

        let expr: Expr = input.parse()?;
        Ok(DessinArg::Expr(expr))
    }
}
impl From<DessinArg> for TokenStream {
    fn from(dessin_arg: DessinArg) -> Self {
        match dessin_arg {
            DessinArg::Let(v) => quote!(#v),
            DessinArg::Ident(v) => quote!(#v),
            DessinArg::Expr(v) => quote!(#v),
        }
    }
}

struct DessinFor {
    variable: Ident,
    it: DessinArg,
    body: TokenStream,
}
impl Parse for DessinFor {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![for] = input.parse()?;
        let variable: Ident = input.parse()?;
        let _: Token![in] = input.parse()?;
        let it: DessinArg = input.parse()?;
        let body;
        let _ = braced!(body in input);
        let body: TokenStream = body.parse()?;

        Ok(DessinFor { variable, it, body })
    }
}
impl From<DessinFor> for TokenStream {
    fn from(DessinFor { variable, it, body }: DessinFor) -> Self {
        let it = TokenStream::from(it);
        quote!(::dessin::prelude::Shape::Group(::dessin::prelude::Group {
            metadata: ::std::vec::Vec::new(),
            local_transform: ::dessin::nalgebra::Transform2::default(),
            shapes: {
                let __current_iterator__ = (#it).into_iter();
                let mut __current_shapes__ = ::std::vec::Vec::with_capacity(__current_iterator__.size_hint().0);
                for #variable in __current_iterator__ {
                    let __current_shape__ = ::dessin::prelude::Shape::from({#body});
                    __current_shapes__.push(__current_shape__);
                }
                __current_shapes__
            },
        }))
    }
}

struct DessinIfElse {
    condition: DessinArg,
    if_body: Box<Dessin>,
    else_body: Option<Box<Dessin>>,
}
impl Parse for DessinIfElse {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![if] = input.parse()?;
        let condition: DessinArg = input.parse()?;
        let if_body;
        let _ = braced!(if_body in input);
        let if_body: Dessin = if_body.parse()?;
        let else_body = if input.parse::<Token![else]>().is_ok() {
            let else_body;
            let _ = braced!(else_body in input);
            Some(Box::new(else_body.parse()?))
        } else {
            None
        };

        Ok(DessinIfElse {
            condition,
            if_body: Box::new(if_body),
            else_body,
        })
    }
}
impl From<DessinIfElse> for TokenStream {
    fn from(
        DessinIfElse {
            condition,
            if_body,
            else_body,
        }: DessinIfElse,
    ) -> Self {
        let else_body = if let Some(else_body) = else_body {
            TokenStream::from(*else_body)
        } else {
            TokenStream::from(DessinType::Empty)
        };

        let condition = TokenStream::from(condition);
        let if_body = TokenStream::from(*if_body);

        quote!(
            if #condition {
                ::dessin::prelude::Shape::from(#if_body)
            } else {
                ::dessin::prelude::Shape::from(#else_body)
            }
        )
    }
}

enum DessinType {
    Empty,
    Item(DessinItem),
    Var(DessinVar),
    Cloned(DessinCloned),
    Group(DessinGroup),
    For(DessinFor),
    IfElse(DessinIfElse),
}
impl Parse for DessinType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(DessinType::Empty)
        } else if input.peek(kw::var) {
            input.parse().map(DessinType::Var)
        } else if input.peek(kw::cloned) {
            input.parse().map(DessinType::Cloned)
        } else if input.peek(Token![for]) {
            input.parse().map(DessinType::For)
        } else if input.peek(Token![if]) {
            input.parse().map(DessinType::IfElse)
        } else if input.peek(Bracket) {
            input.parse().map(DessinType::Group)
        } else {
            input.parse().map(DessinType::Item)
        }
    }
}
impl From<DessinType> for TokenStream {
    fn from(value: DessinType) -> Self {
        match value {
            DessinType::Empty => quote!(::dessin::prelude::Shape::default()),
            DessinType::Item(i) => i.into(),
            DessinType::Var(v) => v.into(),
            DessinType::Cloned(c) => c.into(),
            DessinType::Group(g) => g.into(),
            DessinType::For(f) => f.into(),
            DessinType::IfElse(i) => i.into(),
        }
    }
}

pub struct Dessin {
    dessin_type: DessinType,
    erased_type_shape_actions: Option<ShapeActions>,
}
impl Parse for Dessin {
    fn parse(input: ParseStream) -> Result<Self> {
        let dessin_type: DessinType = input.parse()?;

        let erased_type_shape_actions = if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;

            Some(input.parse()?)
        } else {
            None
        };

        Ok(Dessin {
            dessin_type,
            erased_type_shape_actions,
        })
    }
}
impl From<Dessin> for TokenStream {
    fn from(value: Dessin) -> Self {
        let base = TokenStream::from(value.dessin_type);
        if let Some(shape_actions) = value.erased_type_shape_actions {
            let var = quote!(::dessin::prelude::Shape::from(#base));
            TokenStream::from(DessinVar { var, shape_actions })
        } else {
            quote!(#base)
        }
    }
}

#[test]
fn simple() {
    syn::parse_str::<Dessin>("Item: ()").unwrap();
}
#[test]
fn simple_with_style() {
    syn::parse_str::<Dessin>("Item: #()").unwrap();
}
#[test]
fn action_with_arg() {
    syn::parse_str::<Action>("my_fn={(1., 1.)}").unwrap();
}
#[test]
fn action_with_opt() {
    syn::parse_str::<Action>("my_fn?={Some((1., 1.))}").unwrap();
}
#[test]
fn action_without_arg() {
    syn::parse_str::<Action>("my_fn").unwrap();
}
#[test]
fn action_same_name_arg() {
    syn::parse_str::<Action>("{my_fn}").unwrap();
}
#[test]
fn simple_and_actions() {
    syn::parse_str::<Dessin>("Item: ( my_fn={(1., 1.)} {close} closed )").unwrap();
}
#[test]
fn var() {
    syn::parse_str::<Dessin>("var(v): ( my_fn={(1., 1.)} {close} closed )").unwrap();
}
#[test]
fn cloned() {
    syn::parse_str::<Dessin>("cloned(v): ( my_fn={(1., 1.)} {close} closed )").unwrap();
}
#[test]
fn group() {
    syn::parse_str::<Dessin>("[ Item: (), Item: () ]").unwrap();
}
#[test]
fn as_shape() {
    syn::parse_str::<Dessin>("Item: () -> ()").unwrap();
}
#[test]
fn group_complex() {
    syn::parse_str::<Dessin>("[ Item: (), Item: () ] -> ()").unwrap();
}
#[test]
fn for_loop() {
    syn::parse_str::<Dessin>(
        "for x in 0..10 {
            let y = x as f32 * 2.;
            dessin!(Circle: ( radius={y}) )
        }",
    )
    .unwrap();
}
#[test]
fn for_loop_par() {
    syn::parse_str::<Dessin>(
        "for x in (it) {
            let y = x as f32 * 2.;
            dessin!(Circle: ( radius={y}) )
        }",
    )
    .unwrap();
}
#[test]
fn for_loop_var() {
    syn::parse_str::<Dessin>(
        "for x in it {
            let y = x as f32 * 2.;
            dessin!(Circle: ( radius={y}) )
        }",
    )
    .unwrap();
}
// #[test]
// fn for_loop_range_var() {
//     syn::parse_str::<Dessin>(
//         "for x in 0..n {
//             let y = x as f32 * 2.;
//             dessin!(Circle: ( radius={y}) )
//         }",
//     )
//     .unwrap();
// }
#[test]
fn simple_for_loop() {
    syn::parse_str::<Dessin>(
        "for x in xs {
            let y = x as f32 * 2.;
            dessin!(Circle: ( radius={y}) )
        }",
    )
    .unwrap();
}
#[test]
fn for_loop_range_var_par() {
    syn::parse_str::<Dessin>(
        "for x in 0..(n) {
            let y = x as f32 * 2.;
            dessin!(Circle: ( radius={y}) )
        }",
    )
    .unwrap();
}
#[test]
fn branch_if() {
    syn::parse_str::<Dessin>(
        "if test_fn() == 2 {
            Circle: ()
        }",
    )
    .unwrap();
}
#[test]
fn branch_if_else() {
    syn::parse_str::<Dessin>(
        "if test_fn() == 2 {
            Circle: ()
        } else {
            Ellipse: ()
        }",
    )
    .unwrap();
}
#[test]
fn combined_group_erased() {
    syn::parse_str::<Dessin>(
        "[
			Shape: (),
			Shape: () -> (),
			var(shape): () -> (),
		] -> ()",
    )
    .unwrap();
}
#[test]
fn simple_if() {
    syn::parse_str::<Dessin>(
        "if my_condition {
            Circle: ()
        }",
    )
    .unwrap();
}
// #[test]
// fn if_let() {
//     syn::parse_str::<Dessin>(
//         "if let Some(x) = my_condition {
//             Circle: ()
//         }",
//     )
//     .unwrap();
// }
#[test]
fn combined_if() {
    syn::parse_str::<Dessin>(
        "if test_fn() == 2 {
            Circle: () -> ()
        }",
    )
    .unwrap();
}
#[test]
fn mod_if() {
    syn::parse_str::<Dessin>(
        "if test_fn() == 2 {
            my_mod::Circle: () -> ()
        }",
    )
    .unwrap();
}
#[test]
fn var_if() {
    syn::parse_str::<Dessin>(
        "if test_fn() == 2 {
            var(circle): () -> ()
        }",
    )
    .unwrap();
}
#[test]
fn if_if_group() {
    syn::parse_str::<Dessin>(
        "[
			cloned(circle): (),
			if test_fn() == 2 {
            	var(circle): () -> ()
        	},
            for x in 0..1 {
                dessin!()
            },
			Circle: (),
		]",
    )
    .unwrap();
}
#[test]
fn group_in_group() {
    syn::parse_str::<Dessin>(
        "[
			[
				Circle: (),
				var(circle): () -> (),
				if test_fn() == 2 {
					var(circle): () -> ()
				},
				var(circle): (),
			],
			cloned(circle): (),
            for x in (var) {
                dessin!()
            },
			[],
			if test_fn() == 2 {
            	[
					[],
					cloned(circle): (),
				]
        	},
			Circle: (),
		]",
    )
    .unwrap();
}
