// import {ClayIconSpriteContext} from '@clayui/icon';
// import React, {useContext, useMemo} from 'react';

// import {DataClient} from './DataClient';

use std::{
    cell::{Ref, RefCell},
    num::NonZeroUsize,
    rc::Rc,
};

use yew::{
    html, virtual_dom::AttrValue, Children, Component, Context, ContextProvider, Html, Properties,
};

use crate::ClayIconSpriteContext;

use super::data_client::DataClient;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    /// The content of the Provider.
    children: Children,

    /// Path to the location of the spritemap resource.
    spritemap: AttrValue,

    // Set the amount of items that can be cached, set to zero will be
    // treated as infinite, be aware to set an ideal size to offer a
    // positive experience for your user but not use a large amount of memory.
    #[prop_or(NonZeroUsize::new(20).unwrap())]
    storage_max_size: NonZeroUsize,

    #[prop_or_default]
    theme: Option<AttrValue>,
}

#[derive(Debug, Clone)]
struct ProviderContext<'a> {
    client: Rc<RefCell<DataClient>>,
    theme: Option<&'a AttrValue>,
}

impl<'a> ProviderContext<'a> {
    pub fn new(client: DataClient, theme: Option<&'a AttrValue>) -> Self {
        Self {
            client: Rc::new(RefCell::new(client)),
            theme,
        }
    }

    pub fn borrow(&self) -> Ref<'_, DataClient> {
        self.client.as_ref().borrow()
    }
}

impl<'a> PartialEq for ProviderContext<'a> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.client, &other.client) && self.theme == other.theme
    }
}

// const Context = React.createContext<IProviderContext>({} as IProviderContext);

// Context.displayName = 'ClayProviderContext';

pub struct Provider<'a> {
    context: ProviderContext<'a>,
}

impl<'a> Component for Provider<'static> {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let client = DataClient::new(ctx.props().storage_max_size);
        let theme = ctx.props().theme.as_ref();
        let context = ProviderContext::new(client, theme);
        Self { context }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            children,
            spritemap,
            storage_max_size,
            theme,
            ..
        } = ctx.props().clone();

        if self.context.client.borrow().max_size() != storage_max_size {
            self.context.client.borrow_mut().resize(storage_max_size);
        };

        let sprite_context = ClayIconSpriteContext::new(spritemap);
        html! {
            <ContextProvider<ProviderContext> context={self.context.clone()}>

                    {children}

            </ContextProvider<ProviderContext>>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

// export const Provider = ({
// 	children,
// 	spritemap,
// 	storageMaxSize = 20,
// 	theme,
// 	...otherProps
// }: IProviderProps) => {
// 	// Use `useMemo` to instantiate the DataClient only once and when
// 	// updating the property.
// 	const client = useMemo(
// 		() => new DataClient({storageMaxSize}),
// 		[storageMaxSize]
// 	);

// 	return (
// 		<Context.Provider value={{client, theme, ...otherProps}}>
// 			<ClayIconSpriteContext.Provider value={spritemap}>
// 				{theme ? <div className={theme}>{children}</div> : children}
// 			</ClayIconSpriteContext.Provider>
// 		</Context.Provider>
// 	);
// };

// export const useProvider = () => {
// 	return useContext(Context);
// };
