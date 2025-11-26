use leptos::prelude::*;

use thaw::{
    Button, ButtonAppearance, DrawerBody, DrawerHeader, DrawerHeaderTitle, DrawerHeaderTitleAction,
    DrawerPosition, DrawerSize, OverlayDrawer,
};

#[component]
// pub fn BottomModal(open: RwSignal<bool>, title: RwSignal<String>, children: Children) -> impl IntoView {
pub fn RightDrawer(
    open: RwSignal<bool>,
    title: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <OverlayDrawer open position=DrawerPosition::Right >
            <DrawerHeader>
                <DrawerHeaderTitle>
                    <DrawerHeaderTitleAction slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            on_click=move |_| open.set(false)
                        >
                            "x"
                        </Button>
                    </DrawerHeaderTitleAction>
                            <h6>{ move ||title.get() }</h6>
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody>

                    { children() }

            </DrawerBody>
        </OverlayDrawer>
    }
}
